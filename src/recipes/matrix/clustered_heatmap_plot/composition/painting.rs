//! Cell paint and the continuous colour scale the cells and the bar share.

use super::super::SVMissingValue;
use super::super::configuration::SVClusteredHeatmapConfiguration;
use super::super::preparation::SVPreparedClusteredHeatmap;
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkPaint;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVContinuousColourScaleBuilder};

pub(super) fn sv_cell_paint(
    configuration: &SVClusteredHeatmapConfiguration<'_>,
    finite: bool,
    colour: Option<SVColour>,
    style: &SVDecorationStyle,
) -> Option<SVMarkPaint> {
    if finite {
        return Some(SVMarkPaint::filled(colour.unwrap_or_else(|| style.text())));
    }
    match configuration.missing {
        SVMissingValue::Omit => None,
        SVMissingValue::Transparent => Some(SVMarkPaint::filled(style.surface()).with_opacity(0.0)),
        SVMissingValue::Neutral => Some(SVMarkPaint::filled(style.surface())),
    }
}

pub(super) fn sv_colour_scale(
    configuration: &SVClusteredHeatmapConfiguration<'_>,
    prepared: &SVPreparedClusteredHeatmap,
) -> Result<SVContinuousColourScale, SVError> {
    let mut lower = f64::INFINITY;
    let mut upper = f64::NEG_INFINITY;
    for value in prepared
        .cells
        .iter()
        .map(|cell| cell.value)
        .filter(|value| value.is_finite())
    {
        lower = lower.min(value);
        upper = upper.max(value);
    }
    if !lower.is_finite() || !upper.is_finite() {
        lower = 0.0;
        upper = 1.0;
    }
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
