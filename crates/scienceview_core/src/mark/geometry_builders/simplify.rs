//! Polyline simplification: the Ramer–Douglas–Peucker pass of the line mark.

use crate::sv_mark::SVPoint;

/// Drops the points that sit within `tolerance` of the simplified line.
///
/// The tolerance is expressed in display units, so a simplification is a
/// rasterization-independent decision: the same tolerance removes the same
/// points on every backend. A tolerance at or below zero, or a polyline of two
/// points, returns the input unchanged.
pub(crate) fn sv_simplify(points: &[SVPoint], tolerance: f64) -> Vec<SVPoint> {
    let last_index = match points.len().checked_sub(1) {
        Some(last_index) if points.len() >= 3 && tolerance > 0.0 => last_index,
        _ => return points.to_vec(),
    };

    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[last_index] = true;
    let mut pending = vec![(0_usize, last_index)];

    while let Some((first, last)) = pending.pop() {
        let mut furthest = first;
        let mut furthest_distance = 0.0;
        for (index, point) in points
            .iter()
            .copied()
            .enumerate()
            .take(last)
            .skip(first.saturating_add(1))
        {
            let distance = sv_distance_to_segment(point, points[first], points[last]);
            if distance > furthest_distance {
                furthest_distance = distance;
                furthest = index;
            }
        }

        if furthest != first && furthest_distance > tolerance {
            keep[furthest] = true;
            pending.push((first, furthest));
            pending.push((furthest, last));
        }
    }

    points
        .iter()
        .copied()
        .zip(keep)
        .filter_map(|(point, retain)| retain.then_some(point))
        .collect()
}

/// Returns the perpendicular distance from a point to a segment.
fn sv_distance_to_segment(point: SVPoint, start: SVPoint, end: SVPoint) -> f64 {
    let horizontal = end.x() - start.x();
    let vertical = end.y() - start.y();
    let length_squared = horizontal * horizontal + vertical * vertical;
    if length_squared == 0.0 {
        return point.distance_to(start);
    }

    let projection = ((point.x() - start.x()) * horizontal + (point.y() - start.y()) * vertical)
        / length_squared;
    let projection = projection.clamp(0.0, 1.0);
    let closest = SVPoint::new(
        start.x() + horizontal * projection,
        start.y() + vertical * projection,
    );

    point.distance_to(closest)
}
