//! Keeping axis labels from overlapping.
//!
//! The tick space decides how many ticks the scale derives; this pass decides
//! which of those labels fit beside each other. It is deliberately greedy and
//! ordered by position, so the same data and the same viewport always keep the
//! same ticks.

use super::measurement::SVAxisTickMeasurement;
use crate::sv_decoration::{SVDecorationStyle, SVTextBox};
use crate::sv_scale::SVTick;

/// Measures the ticks and drops the labels that would overlap the previous one.
pub(crate) fn sv_measure_axis_ticks(
    ticks: Vec<SVTick>,
    is_horizontal: bool,
    style: &SVDecorationStyle,
    padding: f64,
) -> Vec<SVAxisTickMeasurement> {
    let mut measured: Vec<SVAxisTickMeasurement> = Vec::new();
    let mut previous_end = f64::NEG_INFINITY;

    for tick in ticks {
        if !tick.position().is_finite() {
            continue;
        }
        let label_box = SVTextBox::new(
            tick.label().to_string(),
            style.label_font_size(),
            style.line_height(),
            style.regular_weight(),
        );
        let half_extent = if is_horizontal {
            label_box.width() / 2.0
        } else {
            label_box.height() / 2.0
        };
        let start = tick.position() - half_extent;
        if start < previous_end + padding {
            continue;
        }
        previous_end = tick.position() + half_extent;
        measured.push(SVAxisTickMeasurement::new(tick, label_box));
    }

    measured
}

/// Returns the widest label box in a set of measurements.
pub(crate) fn sv_widest_label(measurements: &[SVAxisTickMeasurement]) -> f64 {
    measurements
        .iter()
        .map(|measurement| measurement.label_box().width())
        .fold(0.0_f64, f64::max)
}
