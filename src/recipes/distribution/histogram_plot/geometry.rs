//! Placing the prepared bins through the marks.
//!
//! The bar element draws one bar mark instance per bin, with the bin's own
//! pixel width as the mark's slot so an explicit, non-uniform edge set keeps
//! exact widths. The step and polygon elements draw the bin tops as a stepped
//! line and a filled area, reusing the line, area and ribbon marks.

use super::SVBarElement;
use super::SVHistogramMultiple;
use super::configuration::SVHistogramConfiguration;
use super::preparation::{SVPreparedHistogram, SVPreparedSeries};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{
    SVBarOrientation, SVGeometryImage, SVImageInterpolation, SVMark, SVMarkPaint, SVMarkPrimitive,
    SVMarkRequest, SVMarkShape, SVMarkTheme,
};
use crate::sv_scale::{SVScale, SVScaleValue};

/// The data-space domains the composition solves its scales over.
pub(super) struct SVDomains {
    pub(super) horizontal_low: f64,
    pub(super) horizontal_high: f64,
    pub(super) vertical_low: f64,
    pub(super) vertical_high: f64,
}

/// Derives the domains from the prepared bins for the configured orientation.
pub(super) fn sv_domains(
    configuration: &SVHistogramConfiguration<'_>,
    prepared: &SVPreparedHistogram,
) -> SVDomains {
    let mut edge_low = f64::INFINITY;
    let mut edge_high = f64::NEG_INFINITY;
    let mut value_low = 0.0f64;
    let mut value_high = 0.0f64;
    for series in &prepared.series {
        for edge in &series.edges {
            edge_low = edge_low.min(*edge);
            edge_high = edge_high.max(*edge);
        }
        for (top, baseline) in series.tops.iter().zip(series.baselines.iter()) {
            value_low = value_low.min(*top).min(*baseline);
            value_high = value_high.max(*top).max(*baseline);
        }
    }
    let (edge_low, edge_high) = sv_pad(edge_low, edge_high);
    let (value_low, value_high) = sv_pad(value_low, value_high);
    match configuration.orientation {
        SVBarOrientation::Vertical => SVDomains {
            horizontal_low: edge_low,
            horizontal_high: edge_high,
            vertical_low: value_low,
            vertical_high: value_high,
        },
        SVBarOrientation::Horizontal => SVDomains {
            horizontal_low: value_low,
            horizontal_high: value_high,
            vertical_low: edge_low,
            vertical_high: edge_high,
        },
    }
}

fn sv_pad(low: f64, high: f64) -> (f64, f64) {
    if !low.is_finite() || !high.is_finite() {
        return (0.0, 1.0);
    }
    if high - low > f64::EPSILON {
        return (low, high);
    }
    let padding = if low.abs() <= f64::EPSILON {
        1.0
    } else {
        low.abs() * 0.1
    };
    (low - padding, high + padding)
}

/// Builds the primitives of one prepared series.
pub(super) fn sv_series_primitives(
    configuration: &SVHistogramConfiguration<'_>,
    prepared: &SVPreparedSeries,
    number_of_series: usize,
    plot: SVResolvedRectangle,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    theme: &SVMarkTheme,
) -> Result<Vec<SVMarkPrimitive<'static>>, SVError> {
    let mut primitives = Vec::new();
    match configuration.element {
        SVBarElement::Bar => sv_push_bars(
            configuration,
            prepared,
            number_of_series,
            plot,
            horizontal,
            vertical,
            theme,
            &mut primitives,
        )?,
        SVBarElement::Step => {
            sv_push_step(prepared, plot, horizontal, vertical, theme, &mut primitives)?
        }
        SVBarElement::Polygon => {
            sv_push_polygon(prepared, plot, horizontal, vertical, &mut primitives)?
        }
    }
    Ok(primitives)
}

#[allow(clippy::too_many_arguments)]
fn sv_push_bars(
    configuration: &SVHistogramConfiguration<'_>,
    series: &SVPreparedSeries,
    number_of_series: usize,
    plot: SVResolvedRectangle,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    theme: &SVMarkTheme,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    let opacity = if configuration.multiple == SVHistogramMultiple::Layer && number_of_series > 1 {
        0.75
    } else {
        1.0
    };
    for bin_index in 0..series.tops.len() {
        let (Some(&left), Some(&right)) =
            (series.edges.get(bin_index), series.edges.get(bin_index + 1))
        else {
            continue;
        };
        let center = (left + right) / 2.0 + series.offsets.get(bin_index).copied().unwrap_or(0.0);
        let top = series.tops.get(bin_index).copied().unwrap_or(0.0);
        let baseline = series.baselines.get(bin_index).copied().unwrap_or(0.0);
        let width = series
            .width_fractions
            .get(bin_index)
            .copied()
            .unwrap_or(configuration.shrink)
            .clamp(1.0e-4, 1.0);
        let slot = (sv_position(horizontal, right)? - sv_position(horizontal, left)?).abs();
        let vertical_slot = (sv_position(vertical, right)? - sv_position(vertical, left)?).abs();
        let (primary, secondary, area) = match configuration.orientation {
            SVBarOrientation::Vertical => (
                vec![center],
                vec![top],
                SVResolvedRectangle::new(
                    sv_position(horizontal, left)?.min(sv_position(horizontal, right)?),
                    plot.y,
                    slot,
                    plot.height,
                ),
            ),
            SVBarOrientation::Horizontal => (
                vec![top],
                vec![center],
                SVResolvedRectangle::new(
                    plot.x,
                    sv_position(vertical, left)?.min(sv_position(vertical, right)?),
                    plot.width,
                    vertical_slot,
                ),
            ),
        };
        if area.width <= 0.0 || area.height <= 0.0 {
            continue;
        }
        let options = SVMark::bar()
            .orientation(configuration.orientation)
            .width(width)
            .baseline(baseline)
            .colour(series.colour)
            .opacity(opacity);
        let mark: SVMark = options.into();
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&primary),
            SVColumnView::Dense(&secondary),
            horizontal,
            vertical,
            area,
        )
        .with_theme(theme.clone());
        sv_owned_primitives(&mark.geometry(&request)?, primitives)?;
    }
    Ok(())
}

fn sv_push_step(
    series: &SVPreparedSeries,
    plot: SVResolvedRectangle,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    theme: &SVMarkTheme,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    let mut horizontal_values = Vec::new();
    let mut vertical_values = Vec::new();
    for bin_index in 0..series.tops.len() {
        let (Some(&left), Some(&right)) =
            (series.edges.get(bin_index), series.edges.get(bin_index + 1))
        else {
            continue;
        };
        let top = series.tops.get(bin_index).copied().unwrap_or(0.0);
        let offset = series.offsets.get(bin_index).copied().unwrap_or(0.0);
        horizontal_values.push(left + offset);
        horizontal_values.push(right + offset);
        vertical_values.push(top);
        vertical_values.push(top);
        if let Some(next) = series.tops.get(bin_index + 1) {
            horizontal_values.push(right + offset);
            vertical_values.push(*next);
        }
    }
    let mark: SVMark = SVMark::line()
        .colour(series.colour)
        .width(SVLength::absolute(theme.line_width()))
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&horizontal_values),
        SVColumnView::Dense(&vertical_values),
        horizontal,
        vertical,
        plot,
    )
    .with_theme(theme.clone());
    sv_owned_primitives(&mark.geometry(&request)?, primitives)
}

fn sv_push_polygon(
    series: &SVPreparedSeries,
    plot: SVResolvedRectangle,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    let centers: Vec<f64> = series
        .edges
        .windows(2)
        .zip(series.offsets.iter())
        .map(|(pair, offset)| (pair[0] + pair[1]) / 2.0 + offset)
        .collect();
    let stacked = series.baselines.iter().any(|baseline| *baseline != 0.0);
    let mark: SVMark = if stacked {
        SVMark::ribbon().colour(series.colour).opacity(0.6).into()
    } else {
        SVMark::area().colour(series.colour).opacity(0.6).into()
    };
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&centers),
        SVColumnView::Dense(&series.tops),
        horizontal,
        vertical,
        plot,
    );
    let request = if stacked {
        request
            .with_lower(SVColumnView::Dense(&series.baselines))
            .with_upper(SVColumnView::Dense(&series.tops))
    } else {
        request
    };
    sv_owned_primitives(&mark.geometry(&request)?, primitives)
}

fn sv_position(scale: &SVScale<'_>, value: f64) -> Result<f64, SVError> {
    scale.position(SVScaleValue::Numeric(value))
}

fn sv_owned_primitives(
    geometry: &crate::sv_mark::SVMarkGeometry<'_>,
    primitives: &mut Vec<SVMarkPrimitive<'static>>,
) -> Result<(), SVError> {
    for primitive in geometry.primitives() {
        let paint: SVMarkPaint = primitive.paint().clone();
        let shape = match primitive.shape() {
            SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
            SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
            SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
            SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
            SVMarkShape::Image(_) => SVMarkShape::Image(SVGeometryImage::new(
                SVResolvedRectangle::new(0.0, 0.0, 0.0, 0.0),
                0,
                0,
                &[],
                SVImageInterpolation::Nearest,
            )),
        };
        primitives.push(SVMarkPrimitive::new(shape, paint));
    }
    Ok(())
}
