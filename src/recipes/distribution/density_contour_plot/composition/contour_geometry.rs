//! Turning the level sets into backend-agnostic path primitives.

use super::super::configuration::SVDensityContourConfiguration;
use super::super::contour::{
    SVContourPoint, SVContourSegment, sv_boundary_segments, sv_contour_epsilon,
    sv_marching_segments, sv_stitch_regions,
};
use super::super::grid::SVDensityGrid;
use super::super::levels::SVContourLevels;
use super::super::options::SVContourStyle;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVGeometryPath, SVGeometryRectangle, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive,
    SVMarkShape, SVPathCommand,
};
use crate::sv_scale::{SVScale, SVScaleValue};

pub(super) fn sv_build_contours(
    configuration: &SVDensityContourConfiguration<'_>,
    grids: &[SVDensityGrid],
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

    for grid in grids {
        let epsilon = sv_extent_epsilon(grid);
        for (index, threshold) in levels.thresholds.iter().copied().enumerate() {
            let path = match configuration.style {
                SVContourStyle::Filled => {
                    let mut segments = sv_marching_segments(grid, threshold);
                    segments.extend(sv_boundary_segments(grid, threshold));
                    let regions = sv_stitch_regions(&segments, epsilon);
                    sv_region_geometry(&regions, horizontal, vertical)?
                }
                SVContourStyle::Lines => {
                    let segments = sv_marching_segments(grid, threshold);
                    sv_line_geometry(&segments, horizontal, vertical)?
                }
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
            geometries.push(SVMarkGeometry::new(vec![primitive]));
        }
    }
    Ok(geometries)
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

fn sv_extent_epsilon(grid: &SVDensityGrid) -> f64 {
    let x_extent = grid.x_edges().last().copied().unwrap_or(0.0)
        - grid.x_edges().first().copied().unwrap_or(0.0);
    let y_extent = grid.y_edges().last().copied().unwrap_or(0.0)
        - grid.y_edges().first().copied().unwrap_or(0.0);
    sv_contour_epsilon(x_extent.max(y_extent))
}
