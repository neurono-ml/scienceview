//! Marching squares over one density grid.

use super::super::grid::SVDensityGrid;
use super::level_set::{
    SVContourPoint, SVContourSegment, sv_cell_segments, sv_crossing, sv_vertex,
};

/// Walks every cell and returns the directed level-set segments.
pub(crate) fn sv_marching_segments(grid: &SVDensityGrid, threshold: f64) -> Vec<SVContourSegment> {
    let mut segments = Vec::new();
    for y_index in 0..grid.y_cell_count() {
        for x_index in 0..grid.x_cell_count() {
            sv_cell_segments(grid, threshold, x_index, y_index, &mut segments);
        }
    }
    segments
}

/// Returns the directed boundary pieces of the region `density >= threshold`.
///
/// The rectangle is walked counter-clockwise, so the high region stays on the
/// left and the boundary pieces stitch with the interior segments.
pub(crate) fn sv_boundary_segments(grid: &SVDensityGrid, threshold: f64) -> Vec<SVContourSegment> {
    let mut segments = Vec::new();
    let x_count = grid.x_cell_count();
    let y_count = grid.y_cell_count();

    for x_index in 0..x_count {
        sv_boundary_edge(
            grid,
            threshold,
            (x_index, 0),
            (x_index + 1, 0),
            &mut segments,
        );
    }
    for y_index in 0..y_count {
        sv_boundary_edge(
            grid,
            threshold,
            (x_count, y_index),
            (x_count, y_index + 1),
            &mut segments,
        );
    }
    for x_index in (0..x_count).rev() {
        sv_boundary_edge(
            grid,
            threshold,
            (x_index + 1, y_count),
            (x_index, y_count),
            &mut segments,
        );
    }
    for y_index in (0..y_count).rev() {
        sv_boundary_edge(
            grid,
            threshold,
            (0, y_index + 1),
            (0, y_index),
            &mut segments,
        );
    }
    segments
}

fn sv_boundary_edge(
    grid: &SVDensityGrid,
    threshold: f64,
    start: (usize, usize),
    end: (usize, usize),
    segments: &mut Vec<SVContourSegment>,
) {
    let start_point = SVContourPoint::new(grid.x_edges()[start.0], grid.y_edges()[start.1]);
    let end_point = SVContourPoint::new(grid.x_edges()[end.0], grid.y_edges()[end.1]);
    let start_value = sv_vertex(grid, start.0, start.1);
    let end_value = sv_vertex(grid, end.0, end.1);
    let start_high = start_value >= threshold;
    let end_high = end_value >= threshold;

    match (start_high, end_high) {
        (true, true) => segments.push(SVContourSegment::new(start_point, end_point)),
        (true, false) => segments.push(SVContourSegment::new(
            start_point,
            sv_crossing(start_point, start_value, end_point, end_value, threshold),
        )),
        (false, true) => segments.push(SVContourSegment::new(
            sv_crossing(start_point, start_value, end_point, end_value, threshold),
            end_point,
        )),
        (false, false) => {}
    }
}
