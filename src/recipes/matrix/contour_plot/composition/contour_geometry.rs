//! Turning the level sets into backend-agnostic path primitives.

use super::super::configuration::SVContourConfiguration;
use super::super::field::SVContourField;
use super::super::geometry::{
    SVContourPoint, SVContourSegment, sv_boundary_segments, sv_contour_epsilon,
    sv_marching_segments, sv_stitch_regions,
};
use super::super::levels::SVContourLevels;
use super::super::options::SVContourStyle;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVGeometryPath, SVGeometryRectangle, SVGeometryText, SVMarkGeometry, SVMarkPaint, SVMarkPoint,
    SVMarkPrimitive, SVMarkShape, SVMarkTheme, SVPathCommand, SVTextAlign, SVTextBaseline,
};
use crate::sv_scale::{SVScale, SVScaleValue};

pub(super) fn sv_build_contours(
    configuration: &SVContourConfiguration<'_>,
    field: &SVContourField,
    levels: &SVContourLevels,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let mut geometries = Vec::new();
    let band_colours = levels.band_colours()?;
    if configuration.style == SVContourStyle::Filled {
        let background = SVMarkPrimitive::new(
            SVMarkShape::Rectangle(SVGeometryRectangle::new(plot, 0.0)),
            SVMarkPaint::filled(band_colours[0]),
        );
        geometries.push(SVMarkGeometry::new(vec![background]));
    }
    let epsilon = sv_extent_epsilon(field);
    let mut labels = Vec::new();

    for (index, threshold) in levels.thresholds.iter().copied().enumerate() {
        let segments = sv_marching_segments(field, threshold);
        let anchor = sv_label_anchor(&segments, index, levels.thresholds.len());
        let path = match configuration.style {
            SVContourStyle::Filled => {
                let mut all = segments.clone();
                all.extend(sv_boundary_segments(field, threshold));
                let regions = sv_stitch_regions(&all, epsilon);
                sv_region_geometry(&regions, horizontal, vertical)?
            }
            SVContourStyle::Lines => sv_line_geometry(&segments, horizontal, vertical)?,
        };
        let Some(path) = path else {
            continue;
        };
        let paint = match configuration.style {
            SVContourStyle::Filled => SVMarkPaint::filled(band_colours[index + 1]),
            SVContourStyle::Lines => {
                SVMarkPaint::stroked(levels.line_colour(threshold)?, configuration.line_width)
            }
        };
        let primitive = SVMarkPrimitive::new(SVMarkShape::Path(path), paint);
        if configuration.labels {
            if let Some(anchor) = anchor {
                labels.push(sv_label_primitive(anchor, threshold, horizontal, vertical)?);
            }
        }
        geometries.push(SVMarkGeometry::new(vec![primitive]));
    }
    if !labels.is_empty() {
        geometries.push(SVMarkGeometry::new(labels));
    }
    Ok(geometries)
}

/// Picks a point on the level curve for its label, spread across the curve so
/// neighbouring levels do not stack their labels at the same vertex.
fn sv_label_anchor(
    segments: &[SVContourSegment],
    level_index: usize,
    level_count: usize,
) -> Option<SVContourPoint> {
    if segments.is_empty() {
        return None;
    }
    let position = (level_index + 1) * segments.len() / (level_count + 1);
    let segment = &segments[position.min(segments.len() - 1)];
    Some(SVContourPoint::new(
        (segment.start.x + segment.end.x) / 2.0,
        (segment.start.y + segment.end.y) / 2.0,
    ))
}

fn sv_label_primitive(
    anchor: SVContourPoint,
    threshold: f64,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVMarkPrimitive<'static>, SVError> {
    let theme = SVMarkTheme::default();
    let position = sv_display(anchor, horizontal, vertical)?;
    let text = SVGeometryText::new(
        position,
        SVContourLevels::label(threshold),
        theme.text_font_family(),
        theme.text_size(),
        SVTextAlign::Center,
        SVTextBaseline::Middle,
    );
    Ok(SVMarkPrimitive::new(
        SVMarkShape::Text(text),
        SVMarkPaint::filled(theme.text_colour()),
    ))
}

fn sv_region_geometry(
    regions: &[Vec<SVContourPoint>],
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<Option<SVGeometryPath>, SVError> {
    if regions.is_empty() {
        return Ok(None);
    }
    let mut commands = Vec::new();
    for region in regions {
        sv_push_closed_region(region, horizontal, vertical, &mut commands)?;
    }
    Ok(Some(SVGeometryPath::new(commands)))
}

fn sv_line_geometry(
    segments: &[SVContourSegment],
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<Option<SVGeometryPath>, SVError> {
    if segments.is_empty() {
        return Ok(None);
    }
    let mut commands = Vec::with_capacity(segments.len() * 2);
    for segment in segments {
        commands.push(SVPathCommand::MoveTo(sv_display(
            segment.start,
            horizontal,
            vertical,
        )?));
        commands.push(SVPathCommand::LineTo(sv_display(
            segment.end,
            horizontal,
            vertical,
        )?));
    }
    Ok(Some(SVGeometryPath::new(commands)))
}

fn sv_push_closed_region(
    region: &[SVContourPoint],
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    commands: &mut Vec<SVPathCommand>,
) -> Result<(), SVError> {
    let Some(first) = region.first() else {
        return Ok(());
    };
    commands.push(SVPathCommand::MoveTo(sv_display(
        *first, horizontal, vertical,
    )?));
    for point in &region[1..] {
        commands.push(SVPathCommand::LineTo(sv_display(
            *point, horizontal, vertical,
        )?));
    }
    commands.push(SVPathCommand::Close);
    Ok(())
}

fn sv_display(
    point: SVContourPoint,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVMarkPoint, SVError> {
    let x = horizontal.position(SVScaleValue::Numeric(point.x))?;
    let y = vertical.position(SVScaleValue::Numeric(point.y))?;
    Ok(SVMarkPoint::new(x, y))
}

fn sv_extent_epsilon(field: &SVContourField) -> f64 {
    let x_extent = field.x_coordinates().last().copied().unwrap_or(0.0)
        - field.x_coordinates().first().copied().unwrap_or(0.0);
    let y_extent = field.y_coordinates().last().copied().unwrap_or(0.0)
        - field.y_coordinates().first().copied().unwrap_or(0.0);
    sv_contour_epsilon(x_extent.max(y_extent))
}
