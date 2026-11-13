//! Stitching directed segments into closed regions.
//!
//! The segments are oriented with the high region on their left, so following
//! an end point to the next segment's start point walks one boundary loop. An
//! outer loop and a hole therefore run in opposite directions, which is exactly
//! what the non-zero fill rule needs to cut the hole.

use super::level_set::{SVContourPoint, SVContourSegment};
use std::collections::HashMap;

/// Stitches the segments into closed loops of at least three vertices.
pub(crate) fn sv_stitch_regions(
    segments: &[SVContourSegment],
    epsilon: f64,
) -> Vec<Vec<SVContourPoint>> {
    let mut starts: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (index, segment) in segments.iter().enumerate() {
        starts
            .entry(sv_key(segment.start, epsilon))
            .or_default()
            .push(index);
    }

    let mut used = vec![false; segments.len()];
    let mut regions = Vec::new();
    for first in 0..segments.len() {
        if used[first] {
            continue;
        }
        let mut points = vec![segments[first].start];
        let mut current = first;
        loop {
            used[current] = true;
            points.push(segments[current].end);
            let next = starts
                .get_mut(&sv_key(segments[current].end, epsilon))
                .and_then(|candidates| candidates.iter().copied().find(|index| !used[*index]));
            match next {
                Some(index) => current = index,
                None => break,
            }
        }
        if points.len() > 1
            && sv_key(points[0], epsilon) == sv_key(points[points.len() - 1], epsilon)
        {
            points.pop();
        }
        if points.len() >= 3 {
            regions.push(points);
        }
    }
    regions
}

fn sv_key(point: SVContourPoint, epsilon: f64) -> (i64, i64) {
    let epsilon = epsilon.max(f64::MIN_POSITIVE);
    (
        (point.x / epsilon).round() as i64,
        (point.y / epsilon).round() as i64,
    )
}

/// Returns a coordinate tolerance relative to the field extent.
pub(crate) fn sv_contour_epsilon(extent: f64) -> f64 {
    if extent.is_finite() && extent > 0.0 {
        extent * 1.0e-9
    } else {
        1.0e-9
    }
}
