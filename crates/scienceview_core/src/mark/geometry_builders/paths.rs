//! Path building shared by the mark variants.

use crate::sv_mark::{SVGeometryPath, SVMarkPoint, SVPathCommand};

/// Builds an open path through the points.
pub(crate) fn sv_open_path(points: &[SVMarkPoint]) -> SVGeometryPath {
    let mut path = SVGeometryPath::default();
    for (position, point) in points.iter().copied().enumerate() {
        if position == 0 {
            path.push(SVPathCommand::MoveTo(point));
        } else {
            path.push(SVPathCommand::LineTo(point));
        }
    }

    path
}

/// Builds a closed path through the points.
pub(crate) fn sv_closed_path(points: &[SVMarkPoint]) -> SVGeometryPath {
    let mut path = sv_open_path(points);
    if !points.is_empty() {
        path.push(SVPathCommand::Close);
    }

    path
}

/// Builds a closed polygon from offsets around a centre.
pub(crate) fn sv_regular_polygon(
    centre: SVMarkPoint,
    offsets: &[(f64, f64)],
    rotation: f64,
) -> SVGeometryPath {
    let vertices: Vec<SVMarkPoint> = offsets
        .iter()
        .map(|(horizontal, vertical)| {
            let rotated_horizontal = horizontal * rotation.cos() - vertical * rotation.sin();
            let rotated_vertical = horizontal * rotation.sin() + vertical * rotation.cos();
            SVMarkPoint::new(
                centre.x() + rotated_horizontal,
                centre.y() + rotated_vertical,
            )
        })
        .collect();

    sv_closed_path(&vertices)
}

/// Returns the vertices of a regular polygon with `sides` sides.
pub(crate) fn sv_polygon_offsets(radius: f64, sides: usize) -> Vec<(f64, f64)> {
    let step = std::f64::consts::TAU / sides as f64;
    (0..sides)
        .map(|side| {
            let angle = -std::f64::consts::FRAC_PI_2 + step * side as f64;
            (radius * angle.cos(), radius * angle.sin())
        })
        .collect()
}
