//! Composing the nested letter-value boxes, the median and the outliers.
//!
//! Every primitive comes from [`SVMark::geometry`](scienceview_core::sv_mark::SVMark::geometry):
//! a box is the `Ribbon` mark, the median is the `Line` mark and an outlier is
//! the `Dot` mark. The recipe only decides the endpoints of each box and hands
//! them to the kernel through a scale.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVBarOrientation, SVDotShape, SVMark, SVMarkGeometry, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape,
};
use scienceview_core::sv_scale::{SVColour, SVScalePalette, sv_default_scale_palette};

use super::configuration::SVBoxenPlotConfiguration;
use super::grouping::SVBoxenGrouping;
use super::scales::SVBoxenScales;
use super::statistics::{SVBoxenSummaries, SVBoxenSummary};

const BOX_OPACITY: f32 = 0.75;

/// Builds every mark geometry of one boxen plot.
pub(crate) fn sv_compose_boxen_geometries(
    summaries: &SVBoxenSummaries,
    scales: &SVBoxenScales<'_>,
    configuration: &SVBoxenPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let palette = sv_default_scale_palette();
    let mut geometries = Vec::new();
    for summary in &summaries.summaries {
        let colour = sv_boxen_colour(summary, &summaries.grouping, &palette);
        for level in summary.boxes() {
            sv_push_box(
                &mut geometries,
                summary,
                level.lower(),
                level.upper(),
                level.relative_width(),
                scales,
                configuration,
                plot,
                colour,
            )?;
        }
        if configuration.show_median {
            sv_push_median(
                &mut geometries,
                summary,
                scales,
                configuration,
                plot,
                colour,
            )?;
        }
        if configuration.show_outliers && !summary.outliers().is_empty() {
            sv_push_outliers(
                &mut geometries,
                summary,
                scales,
                configuration,
                plot,
                colour,
            )?;
        }
    }
    Ok(geometries)
}

fn sv_boxen_colour(
    summary: &SVBoxenSummary,
    grouping: &SVBoxenGrouping,
    palette: &SVScalePalette,
) -> SVColour {
    let index = summary
        .series()
        .and_then(|series| {
            grouping
                .series
                .iter()
                .position(|candidate| sv_same_boxen_series(*candidate, Some(series)))
        })
        .unwrap_or(0);
    palette.colour_at_index(index)
}

#[allow(clippy::too_many_arguments)]
fn sv_push_box(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxenSummary,
    value_low: f64,
    value_high: f64,
    relative_width: f64,
    scales: &SVBoxenScales<'_>,
    configuration: &SVBoxenPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let category = summary.category_position();
    let half_width = summary.box_width() * relative_width / 2.0;
    let geometry = match configuration.orientation {
        SVBarOrientation::Vertical => {
            let primary = [category - half_width, category + half_width];
            let lower = [value_low, value_low];
            let upper = [value_high, value_high];
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&lower),
                &scales.category,
                &scales.value,
                plot,
            )
            .with_lower(SVColumnView::Dense(&lower))
            .with_upper(SVColumnView::Dense(&upper));
            sv_own_boxen_geometry(
                SVMark::from(SVMark::ribbon().colour(colour).opacity(BOX_OPACITY))
                    .geometry(&request)?,
            )?
        }
        SVBarOrientation::Horizontal => {
            let primary = [value_low, value_high];
            let lower = [category - half_width, category - half_width];
            let upper = [category + half_width, category + half_width];
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&lower),
                &scales.value,
                &scales.category,
                plot,
            )
            .with_lower(SVColumnView::Dense(&lower))
            .with_upper(SVColumnView::Dense(&upper));
            sv_own_boxen_geometry(
                SVMark::from(SVMark::ribbon().colour(colour).opacity(BOX_OPACITY))
                    .geometry(&request)?,
            )?
        }
    };
    geometries.push(geometry);
    Ok(())
}

fn sv_push_median(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxenSummary,
    scales: &SVBoxenScales<'_>,
    configuration: &SVBoxenPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let category = summary.category_position();
    let half_width = summary.box_width() / 2.0;
    sv_push_segment(
        geometries,
        configuration.orientation,
        (category - half_width, summary.median()),
        (category + half_width, summary.median()),
        scales,
        plot,
        colour,
    )
}

fn sv_push_outliers(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxenSummary,
    scales: &SVBoxenScales<'_>,
    configuration: &SVBoxenPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let category = summary.category_position();
    let points: Vec<(f64, f64)> = summary
        .outliers()
        .iter()
        .map(|value| (category, *value))
        .collect();
    sv_push_dots(
        geometries,
        configuration.orientation,
        &points,
        configuration.outlier_marker,
        scales,
        plot,
        colour,
    )
}

#[allow(clippy::too_many_arguments)]
fn sv_push_dots(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    orientation: SVBarOrientation,
    points: &[(f64, f64)],
    shape: SVDotShape,
    scales: &SVBoxenScales<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    if points.is_empty() {
        return Ok(());
    }
    let geometry = match orientation {
        SVBarOrientation::Vertical => {
            let primary: Vec<f64> = points.iter().map(|point| point.0).collect();
            let secondary: Vec<f64> = points.iter().map(|point| point.1).collect();
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&secondary),
                &scales.category,
                &scales.value,
                plot,
            );
            sv_own_boxen_geometry(
                SVMark::from(SVMark::dot().shape(shape).colour(colour)).geometry(&request)?,
            )?
        }
        SVBarOrientation::Horizontal => {
            let primary: Vec<f64> = points.iter().map(|point| point.1).collect();
            let secondary: Vec<f64> = points.iter().map(|point| point.0).collect();
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&secondary),
                &scales.value,
                &scales.category,
                plot,
            );
            sv_own_boxen_geometry(
                SVMark::from(SVMark::dot().shape(shape).colour(colour)).geometry(&request)?,
            )?
        }
    };
    geometries.push(geometry);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn sv_push_segment(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    orientation: SVBarOrientation,
    from: (f64, f64),
    to: (f64, f64),
    scales: &SVBoxenScales<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let geometry = match orientation {
        SVBarOrientation::Vertical => {
            let primary = [from.0, to.0];
            let secondary = [from.1, to.1];
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&secondary),
                &scales.category,
                &scales.value,
                plot,
            );
            sv_own_boxen_geometry(SVMark::from(SVMark::line().colour(colour)).geometry(&request)?)?
        }
        SVBarOrientation::Horizontal => {
            let primary = [from.1, to.1];
            let secondary = [from.0, to.0];
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&secondary),
                &scales.value,
                &scales.category,
                plot,
            );
            sv_own_boxen_geometry(SVMark::from(SVMark::line().colour(colour)).geometry(&request)?)?
        }
    };
    geometries.push(geometry);
    Ok(())
}

fn sv_own_boxen_geometry(geometry: SVMarkGeometry<'_>) -> Result<SVMarkGeometry<'static>, SVError> {
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            Ok(SVMarkPrimitive::new(
                sv_own_boxen_shape(primitive.shape())?,
                primitive.paint().clone(),
            ))
        })
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVMarkGeometry::new(primitives))
}

fn sv_own_boxen_shape(shape: &SVMarkShape<'_>) -> Result<SVMarkShape<'static>, SVError> {
    match shape {
        SVMarkShape::Path(path) => Ok(SVMarkShape::Path(path.clone())),
        SVMarkShape::Rectangle(rectangle) => Ok(SVMarkShape::Rectangle(*rectangle)),
        SVMarkShape::Circle(circle) => Ok(SVMarkShape::Circle(*circle)),
        SVMarkShape::Text(text) => Ok(SVMarkShape::Text(text.clone())),
        SVMarkShape::Image(_) => Err(super::configuration::sv_invalid(
            "a boxen plot draws paths and markers only; an image primitive is not part of its composition",
        )),
    }
}

fn sv_same_boxen_series(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => first.to_bits() == second.to_bits(),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}
