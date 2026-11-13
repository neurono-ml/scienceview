//! Composing the box, whiskers, median, mean and outliers from the kernels.
//!
//! Every primitive comes from [`SVMark::geometry`](scienceview_core::sv_mark::SVMark::geometry):
//! the box is the `Ribbon` mark, the median/whiskers/caps are the `Line` mark and
//! the outliers/mean are the `Dot` mark. The recipe only decides the two data
//! endpoints of each segment and hands them to the kernel through a scale.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVBarOrientation, SVDotShape, SVMark, SVMarkGeometry, SVMarkPrimitive, SVMarkRequest,
    SVMarkShape,
};
use scienceview_core::sv_scale::{SVColour, SVScalePalette, sv_default_scale_palette};

use super::configuration::SVBoxPlotConfiguration;
use super::grouping::SVBoxGrouping;
use super::scales::SVBoxScales;
use super::statistics::{SVBoxSummaries, SVBoxSummary};

const BOX_OPACITY: f32 = 0.75;
const NOTCH_WIDTH_FRACTION: f64 = 0.5;

/// Builds every mark geometry of one box plot.
pub(crate) fn sv_compose_box_geometries(
    summaries: &SVBoxSummaries,
    scales: &SVBoxScales<'_>,
    configuration: &SVBoxPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let palette = sv_default_scale_palette();
    let mut geometries = Vec::new();
    for summary in &summaries.summaries {
        let colour = sv_box_colour(summary, &summaries.grouping, &palette);
        sv_push_box(
            &mut geometries,
            summary,
            scales,
            configuration,
            plot,
            colour,
        )?;
        sv_push_median(
            &mut geometries,
            summary,
            scales,
            configuration,
            plot,
            colour,
        )?;
        sv_push_whiskers(
            &mut geometries,
            summary,
            scales,
            configuration,
            plot,
            colour,
        )?;
        if configuration.show_mean {
            sv_push_mean(
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

fn sv_box_colour(
    summary: &SVBoxSummary,
    grouping: &SVBoxGrouping,
    palette: &SVScalePalette,
) -> SVColour {
    let index = summary
        .series()
        .and_then(|series| {
            grouping
                .series
                .iter()
                .position(|candidate| sv_same_series(*candidate, Some(series)))
        })
        .unwrap_or(0);
    palette.colour_at_index(index)
}

fn sv_push_box(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxSummary,
    scales: &SVBoxScales<'_>,
    configuration: &SVBoxPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let category = summary.category_position();
    let half_width = summary.box_width() / 2.0;
    if !configuration.notch || summary.notch_half_width() <= 0.0 {
        return sv_push_rectangle(
            geometries,
            configuration.orientation,
            category,
            half_width,
            summary.lower_quartile(),
            summary.upper_quartile(),
            scales,
            plot,
            colour,
        );
    }
    let notch = summary.notch_half_width();
    let narrow_half_width = half_width * NOTCH_WIDTH_FRACTION;
    sv_push_rectangle(
        geometries,
        configuration.orientation,
        category,
        half_width,
        summary.median() + notch,
        summary.upper_quartile(),
        scales,
        plot,
        colour,
    )?;
    sv_push_rectangle(
        geometries,
        configuration.orientation,
        category,
        half_width,
        summary.lower_quartile(),
        summary.median() - notch,
        scales,
        plot,
        colour,
    )?;
    sv_push_rectangle(
        geometries,
        configuration.orientation,
        category,
        narrow_half_width,
        summary.median() - notch,
        summary.median() + notch,
        scales,
        plot,
        colour,
    )
}

#[allow(clippy::too_many_arguments)]
fn sv_push_rectangle(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    orientation: SVBarOrientation,
    category_position: f64,
    category_half_width: f64,
    value_low: f64,
    value_high: f64,
    scales: &SVBoxScales<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let geometry = match orientation {
        SVBarOrientation::Vertical => {
            let primary = [
                category_position - category_half_width,
                category_position + category_half_width,
            ];
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
            let geometry = SVMark::from(SVMark::ribbon().colour(colour).opacity(BOX_OPACITY))
                .geometry(&request)?;
            sv_own_geometry(geometry)?
        }
        SVBarOrientation::Horizontal => {
            let primary = [value_low, value_high];
            let lower = [
                category_position - category_half_width,
                category_position - category_half_width,
            ];
            let upper = [
                category_position + category_half_width,
                category_position + category_half_width,
            ];
            let request = SVMarkRequest::new(
                SVColumnView::Dense(&primary),
                SVColumnView::Dense(&lower),
                &scales.value,
                &scales.category,
                plot,
            )
            .with_lower(SVColumnView::Dense(&lower))
            .with_upper(SVColumnView::Dense(&upper));
            let geometry = SVMark::from(SVMark::ribbon().colour(colour).opacity(BOX_OPACITY))
                .geometry(&request)?;
            sv_own_geometry(geometry)?
        }
    };
    geometries.push(geometry);
    Ok(())
}

fn sv_push_median(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxSummary,
    scales: &SVBoxScales<'_>,
    configuration: &SVBoxPlotConfiguration<'_>,
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

fn sv_push_whiskers(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxSummary,
    scales: &SVBoxScales<'_>,
    configuration: &SVBoxPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let category = summary.category_position();
    let half_width = summary.box_width() / 2.0;
    let cap_half_width = half_width / 2.0;
    for (from, to) in [
        (
            (category, summary.upper_quartile()),
            (category, summary.upper_whisker()),
        ),
        (
            (category, summary.lower_whisker()),
            (category, summary.lower_quartile()),
        ),
        (
            (category - cap_half_width, summary.upper_whisker()),
            (category + cap_half_width, summary.upper_whisker()),
        ),
        (
            (category - cap_half_width, summary.lower_whisker()),
            (category + cap_half_width, summary.lower_whisker()),
        ),
    ] {
        sv_push_segment(
            geometries,
            configuration.orientation,
            from,
            to,
            scales,
            plot,
            colour,
        )?;
    }
    Ok(())
}

fn sv_push_mean(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxSummary,
    scales: &SVBoxScales<'_>,
    configuration: &SVBoxPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let point = [(summary.category_position(), summary.mean())];
    sv_push_dots(
        geometries,
        configuration.orientation,
        &point,
        SVDotShape::Diamond,
        scales,
        plot,
        colour,
    )
}

fn sv_push_outliers(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVBoxSummary,
    scales: &SVBoxScales<'_>,
    configuration: &SVBoxPlotConfiguration<'_>,
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
    scales: &SVBoxScales<'_>,
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
            let geometry =
                SVMark::from(SVMark::dot().shape(shape).colour(colour)).geometry(&request)?;
            sv_own_geometry(geometry)?
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
            let geometry =
                SVMark::from(SVMark::dot().shape(shape).colour(colour)).geometry(&request)?;
            sv_own_geometry(geometry)?
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
    scales: &SVBoxScales<'_>,
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
            let geometry = SVMark::from(SVMark::line().colour(colour)).geometry(&request)?;
            sv_own_geometry(geometry)?
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
            let geometry = SVMark::from(SVMark::line().colour(colour)).geometry(&request)?;
            sv_own_geometry(geometry)?
        }
    };
    geometries.push(geometry);
    Ok(())
}

fn sv_own_geometry(geometry: SVMarkGeometry<'_>) -> Result<SVMarkGeometry<'static>, SVError> {
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            Ok(SVMarkPrimitive::new(
                sv_own_shape(primitive.shape())?,
                primitive.paint().clone(),
            ))
        })
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVMarkGeometry::new(primitives))
}

fn sv_own_shape(shape: &SVMarkShape<'_>) -> Result<SVMarkShape<'static>, SVError> {
    match shape {
        SVMarkShape::Path(path) => Ok(SVMarkShape::Path(path.clone())),
        SVMarkShape::Rectangle(rectangle) => Ok(SVMarkShape::Rectangle(*rectangle)),
        SVMarkShape::Circle(circle) => Ok(SVMarkShape::Circle(*circle)),
        SVMarkShape::Text(text) => Ok(SVMarkShape::Text(text.clone())),
        SVMarkShape::Image(_) => Err(super::configuration::sv_invalid(
            "a box plot draws paths and markers only; an image primitive is not part of its composition",
        )),
    }
}

fn sv_same_series(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => first.to_bits() == second.to_bits(),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}
