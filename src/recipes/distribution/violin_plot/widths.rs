//! Scaling the raw density estimates into the category half-width.
//!
//! The density integrates to one over the value axis, so a single factor keeps
//! every area equal; the count and width modes derive their own factors. The
//! factors are computed from the whole figure, never per mark.

use super::scale_mode::SVViolinScale;
use super::statistics::SVViolinSummary;

/// Assigns each summary its maximum half-width and its density factor.
pub(crate) fn sv_scale_widths(summaries: &mut [SVViolinSummary], scale: SVViolinScale, width: f32) {
    let maximum_half_width = f64::from(width) / 2.0;
    let global_peak = summaries
        .iter()
        .map(sv_peak_density)
        .fold(0.0_f64, f64::max)
        .max(f64::MIN_POSITIVE);
    let count_peak = summaries
        .iter()
        .map(|summary| summary.number_of_points() as f64 * sv_peak_density(summary))
        .fold(0.0_f64, f64::max)
        .max(f64::MIN_POSITIVE);
    for summary in summaries.iter_mut() {
        let peak = sv_peak_density(summary).max(f64::MIN_POSITIVE);
        let (factor, half_width) = match scale {
            SVViolinScale::Area => {
                let factor = maximum_half_width / global_peak;
                (factor, factor * peak)
            }
            SVViolinScale::Count => {
                let factor = maximum_half_width * summary.number_of_points() as f64 / count_peak;
                (factor, factor * peak)
            }
            SVViolinScale::Width => {
                let factor = maximum_half_width / peak;
                (factor, maximum_half_width)
            }
        };
        summary.width_factor = factor;
        summary.maximum_half_width = half_width.min(maximum_half_width);
    }
}

fn sv_peak_density(summary: &SVViolinSummary) -> f64 {
    summary.density().iter().copied().fold(0.0_f64, f64::max)
}
