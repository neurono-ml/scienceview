//! Coverage sampling shared by the shape and path rasterizers.
//!
//! Anti-aliasing comes from four samples per axis inside each device pixel.
//! Sixteen samples are enough to give an edge a stable intermediate value
//! without a jitter table, which keeps the output deterministic: the same
//! primitives always produce the same bytes.

/// The number of samples taken per axis inside one pixel.
pub(crate) const SV_SAMPLES_PER_AXIS: usize = 4;

/// The total number of samples inside one pixel.
pub(crate) const SV_SAMPLES_PER_PIXEL: u32 = (SV_SAMPLES_PER_AXIS * SV_SAMPLES_PER_AXIS) as u32;

/// Returns one sample coordinate inside the pixel starting at `pixel`.
pub(crate) fn sv_sample_coordinate(pixel: f64, sample: usize) -> f64 {
    pixel + (sample as f64 + 0.5) / SV_SAMPLES_PER_AXIS as f64
}

/// Counts the samples inside one pixel that satisfy `is_inside`.
pub(crate) fn sv_sampled_coverage(
    pixel_x: f64,
    pixel_y: f64,
    mut is_inside: impl FnMut(f64, f64) -> bool,
) -> f32 {
    let mut inside = 0_u32;
    for sample_y in 0..SV_SAMPLES_PER_AXIS {
        let y = sv_sample_coordinate(pixel_y, sample_y);
        for sample_x in 0..SV_SAMPLES_PER_AXIS {
            let x = sv_sample_coordinate(pixel_x, sample_x);
            if is_inside(x, y) {
                inside += 1;
            }
        }
    }

    inside as f32 / SV_SAMPLES_PER_PIXEL as f32
}

/// Converts a signed distance to a shape into pixel coverage.
///
/// A pixel whose centre is on the boundary gets half coverage, a pixel fully
/// inside gets one and a pixel fully outside gets zero. The result is the same
/// intermediate ramp a supersampled edge produces, without the samples.
pub(crate) fn sv_coverage_from_distance(distance: f64) -> f32 {
    if !distance.is_finite() {
        return 0.0;
    }
    (0.5 - distance).clamp(0.0, 1.0) as f32
}

/// Converts a device-space span to the pixel range it touches, clamped to the
/// frame.
pub(crate) fn sv_device_pixel_range(minimum: f64, maximum: f64, limit: usize) -> (usize, usize) {
    let clamp = |value: f64| -> usize {
        if !value.is_finite() || value <= 0.0 {
            0
        } else {
            value.min(limit as f64) as usize
        }
    };
    (clamp(minimum.floor()), clamp(maximum.ceil()))
}

/// Returns the distance from a point to a segment and the unclamped
/// projection of the point onto it.
///
/// The projection is `0.0` at the start, `1.0` at the end and outside that
/// range when the point lies beyond an endpoint; a caller uses it to give a
/// butt cap its flat end. `extend` lengthens the segment by that many device
/// pixels at both ends, which is how a square cap is drawn.
pub(crate) fn sv_segment_distance(
    point_x: f64,
    point_y: f64,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    extend: f64,
) -> (f64, f64) {
    let direction_x = end_x - start_x;
    let direction_y = end_y - start_y;
    let length_squared = direction_x * direction_x + direction_y * direction_y;
    let projection = if length_squared <= f64::EPSILON {
        0.0
    } else {
        ((point_x - start_x) * direction_x + (point_y - start_y) * direction_y) / length_squared
    };
    let extra = if length_squared <= f64::EPSILON {
        0.0
    } else {
        extend / length_squared.sqrt()
    };
    let clamped = projection.clamp(-extra, 1.0 + extra);
    let closest_x = start_x + clamped * direction_x;
    let closest_y = start_y + clamped * direction_y;
    let offset_x = point_x - closest_x;
    let offset_y = point_y - closest_y;
    (
        (offset_x * offset_x + offset_y * offset_y).sqrt(),
        projection,
    )
}
