//! The level-set primitives and the marching-squares case table.

use super::super::grid::SVDensityGrid;

/// One position in data space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVContourPoint {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl SVContourPoint {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// One directed contour segment with the high region on its left.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVContourSegment {
    pub(crate) start: SVContourPoint,
    pub(crate) end: SVContourPoint,
}

impl SVContourSegment {
    pub(crate) fn new(start: SVContourPoint, end: SVContourPoint) -> Self {
        Self { start, end }
    }
}

/// Appends the level-set segments of one grid cell.
pub(super) fn sv_cell_segments(
    grid: &SVDensityGrid,
    threshold: f64,
    x_index: usize,
    y_index: usize,
    segments: &mut Vec<SVContourSegment>,
) {
    let x0 = grid.x_edges()[x_index];
    let x1 = grid.x_edges()[x_index + 1];
    let y0 = grid.y_edges()[y_index];
    let y1 = grid.y_edges()[y_index + 1];
    let bottom_left = sv_vertex(grid, x_index, y_index);
    let bottom_right = sv_vertex(grid, x_index + 1, y_index);
    let top_right = sv_vertex(grid, x_index + 1, y_index + 1);
    let top_left = sv_vertex(grid, x_index, y_index + 1);

    let p0 = SVContourPoint::new(x0, y0);
    let p1 = SVContourPoint::new(x1, y0);
    let p2 = SVContourPoint::new(x1, y1);
    let p3 = SVContourPoint::new(x0, y1);

    let mut case = 0u8;
    if bottom_left >= threshold {
        case |= 1;
    }
    if bottom_right >= threshold {
        case |= 2;
    }
    if top_right >= threshold {
        case |= 4;
    }
    if top_left >= threshold {
        case |= 8;
    }
    if case == 0 || case == 15 {
        return;
    }

    let bottom = sv_crossing(p0, bottom_left, p1, bottom_right, threshold);
    let right = sv_crossing(p1, bottom_right, p2, top_right, threshold);
    let top = sv_crossing(p2, top_right, p3, top_left, threshold);
    let left = sv_crossing(p3, top_left, p0, bottom_left, threshold);

    let centre_high = (bottom_left + bottom_right + top_right + top_left) / 4.0 >= threshold;
    let mut push = |start, end| segments.push(SVContourSegment::new(start, end));
    match case {
        1 => push(bottom, left),
        2 => push(right, bottom),
        3 => push(right, left),
        4 => push(top, right),
        5 if centre_high => {
            push(bottom, right);
            push(top, left);
        }
        5 => {
            push(bottom, left);
            push(top, right);
        }
        6 => push(top, bottom),
        7 => push(top, left),
        8 => push(left, top),
        9 => push(bottom, top),
        10 if centre_high => {
            push(left, bottom);
            push(right, top);
        }
        10 => {
            push(right, bottom);
            push(left, top);
        }
        11 => push(right, top),
        12 => push(left, right),
        13 => push(bottom, right),
        14 => push(left, bottom),
        _ => {}
    }
}

/// Reads a grid-vertex value from the cell-centred density field.
///
/// The outermost vertices replicate the nearest cell, so a contour never sees
/// an artificial zero row or column at the domain edge.
pub(super) fn sv_vertex(grid: &SVDensityGrid, x_index: usize, y_index: usize) -> f64 {
    let x = x_index.min(grid.x_cell_count().saturating_sub(1));
    let y = y_index.min(grid.y_cell_count().saturating_sub(1));
    grid.density_at(x, y)
}

pub(super) fn sv_crossing(
    start: SVContourPoint,
    start_value: f64,
    end: SVContourPoint,
    end_value: f64,
    threshold: f64,
) -> SVContourPoint {
    let span = end_value - start_value;
    let fraction = if span.abs() > f64::MIN_POSITIVE {
        ((threshold - start_value) / span).clamp(0.0, 1.0)
    } else {
        0.5
    };
    SVContourPoint::new(
        start.x + (end.x - start.x) * fraction,
        start.y + (end.y - start.y) * fraction,
    )
}
