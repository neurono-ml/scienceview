//! Cell paint and the continuous colour scale the cells and the bar share.

use super::super::SVMissingValue;
use super::super::configuration::SVHeatmapConfiguration;
use super::super::preparation::SVPreparedHeatmap;
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkPaint;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVContinuousColourScaleBuilder};

pub(super) fn sv_cell_paint(
    configuration: &SVHeatmapConfiguration<'_>,
    finite: bool,
    colour: Option<SVColour>,
    style: &SVDecorationStyle,
) -> Option<SVMarkPaint> {
    let paint = if finite {
        SVMarkPaint::filled(colour.unwrap_or_else(|| style.text()))
    } else {
        match configuration.missing {
            SVMissingValue::Omit => return None,
            SVMissingValue::Transparent => SVMarkPaint::filled(style.surface()).with_opacity(0.0),
            SVMissingValue::Neutral => SVMarkPaint::filled(style.surface()),
        }
    };
    let paint = if configuration.line_width > 0.0 {
        paint.with_stroke(style.background(), configuration.line_width)
    } else {
        paint
    };
    Some(paint)
}

pub(super) fn sv_colour_scale(
    configuration: &SVHeatmapConfiguration<'_>,
    prepared: &SVPreparedHeatmap,
) -> Result<SVContinuousColourScale, SVError> {
    let mut values: Vec<f64> = prepared
        .cells
        .iter()
        .map(|cell| cell.value)
        .filter(|value| value.is_finite())
        .collect();
    let (mut lower, mut upper) = if values.is_empty() {
        (0.0, 1.0)
    } else if configuration.robust && values.len() >= 5 {
        values.sort_by(f64::total_cmp);
        (sv_percentile(&values, 0.02), sv_percentile(&values, 0.98))
    } else {
        let mut lower = f64::INFINITY;
        let mut upper = f64::NEG_INFINITY;
        for value in &values {
            lower = lower.min(*value);
            upper = upper.max(*value);
        }
        (lower, upper)
    };
    if lower == upper {
        let padding = if lower.abs() <= f64::EPSILON {
            1.0
        } else {
            lower.abs() * 0.1
        };
        lower -= padding;
        upper += padding;
    }
    if let Some(centre) = configuration.centre {
        let extent = (centre - lower).abs().max((upper - centre).abs());
        lower = centre - extent;
        upper = centre + extent;
    }
    SVContinuousColourScaleBuilder::new()
        .domain(lower, upper)
        .colour_map(configuration.colour_map.clone())
        .build()
}

/// Returns the linearly interpolated percentile of an ascending slice.
///
/// The rank is interpolated between the two neighbouring order statistics, the
/// definition numpy and seaborn use. Rounding the rank instead would include
/// the outermost value at small sample counts, which defeats `robust`.
fn sv_percentile(sorted: &[f64], fraction: f64) -> f64 {
    let rank = fraction * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let weight = rank - lower as f64;
    sorted[lower] * (1.0 - weight) + sorted[upper] * weight
}
