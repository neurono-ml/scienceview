//! Data-space interpolation for the line's `interpolate` option.
//!
//! The line mark draws the points it is handed as a polyline. The step and
//! spline shapes are therefore expressed as extra points *before* the mark is
//! composed: the recipe never replaces the mark kernel, it only gives it the
//! vertices it needs. Interpolation happens in data space so the scales and the
//! axes still see the observed positions.

use super::SVLineInterpolation;

const SPLINE_SAMPLES_PER_SEGMENT: usize = 16;

/// Expands a segment of observations into the vertices of the requested shape.
pub(super) fn sv_interpolate_segment(
    points: &[(f64, f64)],
    interpolation: SVLineInterpolation,
) -> Vec<(f64, f64)> {
    if points.len() < 2 || interpolation == SVLineInterpolation::Linear {
        return points.to_vec();
    }

    match interpolation {
        SVLineInterpolation::Linear => points.to_vec(),
        SVLineInterpolation::StepStart => sv_step_start(points),
        SVLineInterpolation::StepEnd => sv_step_end(points),
        SVLineInterpolation::StepMiddle => sv_step_middle(points),
        SVLineInterpolation::StepCentered => sv_step_centered(points),
        SVLineInterpolation::Spline => sv_spline(points),
    }
}

fn sv_step_start(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(points.len() * 2);
    for window in points.windows(2) {
        let (x, y) = window[0];
        let (next_x, _) = window[1];
        result.push((x, y));
        result.push((next_x, y));
    }
    result.push(*points.last().unwrap_or(&points[0]));
    result
}

fn sv_step_end(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(points.len() * 2);
    for window in points.windows(2) {
        let (x, y) = window[0];
        let (_, next_y) = window[1];
        result.push((x, y));
        result.push((x, next_y));
    }
    result.push(*points.last().unwrap_or(&points[0]));
    result
}

fn sv_step_middle(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(points.len() * 3);
    for window in points.windows(2) {
        let (x, y) = window[0];
        let (next_x, next_y) = window[1];
        let middle = (x + next_x) / 2.0;
        result.push((x, y));
        result.push((middle, y));
        result.push((middle, next_y));
    }
    result.push(*points.last().unwrap_or(&points[0]));
    result
}

fn sv_step_centered(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(points.len() * 4);
    for window in points.windows(2) {
        let (x, y) = window[0];
        let (next_x, next_y) = window[1];
        let middle_y = (y + next_y) / 2.0;
        result.push((x, y));
        result.push((x, middle_y));
        result.push((next_x, middle_y));
    }
    result.push(*points.last().unwrap_or(&points[0]));
    result
}

fn sv_spline(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(points.len() * SPLINE_SAMPLES_PER_SEGMENT);
    for segment in 0..points.len() - 1 {
        let previous = points[segment.saturating_sub(1)];
        let start = points[segment];
        let end = points[segment + 1];
        let next = points[(segment + 2).min(points.len() - 1)];
        for sample in 0..SPLINE_SAMPLES_PER_SEGMENT {
            let fraction = sample as f64 / SPLINE_SAMPLES_PER_SEGMENT as f64;
            result.push((
                sv_catmull_rom(previous.0, start.0, end.0, next.0, fraction),
                sv_catmull_rom(previous.1, start.1, end.1, next.1, fraction),
            ));
        }
    }
    result.push(*points.last().unwrap_or(&points[0]));
    result
}

fn sv_catmull_rom(previous: f64, start: f64, end: f64, next: f64, fraction: f64) -> f64 {
    let squared = fraction * fraction;
    let cubed = squared * fraction;
    0.5 * ((2.0 * start)
        + (-previous + end) * fraction
        + (2.0 * previous - 5.0 * start + 4.0 * end - next) * squared
        + (-previous + 3.0 * start - 3.0 * end + next) * cubed)
}
