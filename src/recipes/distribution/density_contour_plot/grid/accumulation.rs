//! Normalising and smoothing the joint counts into probability densities.

/// Adds one observation to a cell and to the group total.
pub(super) fn sv_accumulate(
    counts: &mut [f64],
    total: &mut f64,
    x_edges: &[f64],
    y_edges: &[f64],
    x_value: f64,
    y_value: f64,
    weight: f64,
) {
    let horizontal = x_edges.len() - 1;
    let x_index = sv_bin_index(x_edges, x_value);
    let y_index = sv_bin_index(y_edges, y_value);
    counts[y_index * horizontal + x_index] += weight;
    *total += weight;
}

fn sv_bin_index(edges: &[f64], value: f64) -> usize {
    let last = edges.len() - 2;
    edges
        .partition_point(|edge| *edge <= value)
        .saturating_sub(1)
        .min(last)
}

pub(super) fn sv_normalise(
    counts: &[f64],
    x_edges: &[f64],
    y_edges: &[f64],
    total: f64,
) -> Vec<f64> {
    let horizontal = x_edges.len() - 1;
    let mut densities = vec![0.0; counts.len()];
    if total <= 0.0 {
        return densities;
    }
    for y_index in 0..y_edges.len() - 1 {
        let height = y_edges[y_index + 1] - y_edges[y_index];
        for x_index in 0..horizontal {
            let width = x_edges[x_index + 1] - x_edges[x_index];
            let position = y_index * horizontal + x_index;
            let area = width * height;
            if area > 0.0 {
                densities[position] = counts[position] / (total * area);
            }
        }
    }
    sv_renormalise(&mut densities, x_edges, y_edges);
    densities
}

pub(super) fn sv_renormalise(densities: &mut [f64], x_edges: &[f64], y_edges: &[f64]) {
    let integral = sv_integral(densities, x_edges, y_edges);
    if integral > 0.0 {
        densities.iter_mut().for_each(|value| *value /= integral);
    }
}

fn sv_integral(densities: &[f64], x_edges: &[f64], y_edges: &[f64]) -> f64 {
    let horizontal = x_edges.len() - 1;
    let mut integral = 0.0;
    for y_index in 0..y_edges.len() - 1 {
        let height = y_edges[y_index + 1] - y_edges[y_index];
        for x_index in 0..horizontal {
            let width = x_edges[x_index + 1] - x_edges[x_index];
            integral += densities[y_index * horizontal + x_index] * width * height;
        }
    }
    integral
}

pub(super) fn sv_smooth(
    densities: &[f64],
    horizontal: usize,
    vertical: usize,
    x_edges: &[f64],
    y_edges: &[f64],
    sigma: f64,
) -> Vec<f64> {
    let mean_width = (x_edges[horizontal] - x_edges[0]) / horizontal as f64;
    let mean_height = (y_edges[vertical] - y_edges[0]) / vertical as f64;
    if mean_width <= 0.0 || mean_height <= 0.0 || !sigma.is_finite() || sigma <= 0.0 {
        return densities.to_vec();
    }
    let sigma_x = sigma / mean_width;
    let sigma_y = sigma / mean_height;
    let blurred = sv_blur_axis(densities, horizontal, vertical, sigma_x, true);
    sv_blur_axis(&blurred, horizontal, vertical, sigma_y, false)
}

fn sv_blur_axis(
    densities: &[f64],
    horizontal: usize,
    vertical: usize,
    sigma: f64,
    along_x: bool,
) -> Vec<f64> {
    let (outer, inner) = if along_x {
        (vertical, horizontal)
    } else {
        (horizontal, vertical)
    };
    let mut output = vec![0.0; densities.len()];
    // The kernel samples with clamped edge indices, so a radius wider than the
    // axis only repeats the edge values. Capping it at the axis length keeps a
    // very large `bandwidth` finite in both memory and work.
    let radius_limit = inner.saturating_sub(1).max(1) as isize;
    let radius = ((3.0 * sigma).ceil() as isize).clamp(1, radius_limit);
    let mut kernel = Vec::with_capacity((2 * radius + 1) as usize);
    let mut norm = 0.0;
    for offset in -radius..=radius {
        let weight = (-0.5 * (offset as f64 / sigma.max(1.0e-9)).powi(2)).exp();
        kernel.push(weight);
        norm += weight;
    }
    kernel.iter_mut().for_each(|weight| *weight /= norm);

    for outer_index in 0..outer {
        for inner_index in 0..inner {
            let mut sum = 0.0;
            for (position, weight) in kernel.iter().enumerate() {
                let offset = position as isize - radius;
                let sample = (inner_index as isize + offset).clamp(0, inner as isize - 1) as usize;
                let (x_index, y_index) = if along_x {
                    (sample, outer_index)
                } else {
                    (outer_index, sample)
                };
                sum += densities[y_index * horizontal + x_index] * weight;
            }
            let (x_index, y_index) = if along_x {
                (inner_index, outer_index)
            } else {
                (outer_index, inner_index)
            };
            output[y_index * horizontal + x_index] = sum;
        }
    }
    output
}
