//! Marching squares over one scalar field and along its boundary.

use super::super::field::SVContourField;
use super::level_set::{SVContourPoint, SVContourSegment, sv_cell_segments, sv_crossing};

/// Walks every cell and returns the directed level-set segments.
pub(crate) fn sv_marching_segments(
    field: &SVContourField,
    threshold: f64,
) -> Vec<SVContourSegment> {
    let mut segments = Vec::new();
    for y_index in 0..field.number_of_rows().saturating_sub(1) {
        for x_index in 0..field.number_of_columns().saturating_sub(1) {
            sv_cell_segments(field, threshold, x_index, y_index, &mut segments);
        }
    }
    segments
}

/// Returns the directed boundary pieces of the region `value >= threshold`.
///
/// The lattice rectangle is walked counter-clockwise, so the high region stays
/// on the left and the boundary pieces stitch with the interior segments.
pub(crate) fn sv_boundary_segments(
    field: &SVContourField,
    threshold: f64,
) -> Vec<SVContourSegment> {
    let mut segments = Vec::new();
    let columns = field.number_of_columns();
    let rows = field.number_of_rows();
    if columns < 2 || rows < 2 {
        return segments;
    }
    for x_index in 0..columns - 1 {
        sv_boundary_edge(
            field,
            threshold,
            (x_index, 0),
            (x_index + 1, 0),
            &mut segments,
        );
    }
    for y_index in 0..rows - 1 {
        sv_boundary_edge(
            field,
            threshold,
            (columns - 1, y_index),
            (columns - 1, y_index + 1),
            &mut segments,
        );
    }
    for x_index in (0..columns - 1).rev() {
        sv_boundary_edge(
            field,
            threshold,
            (x_index + 1, rows - 1),
            (x_index, rows - 1),
            &mut segments,
        );
    }
    for y_index in (0..rows - 1).rev() {
        sv_boundary_edge(
            field,
            threshold,
            (0, y_index + 1),
            (0, y_index),
            &mut segments,
        );
    }
    segments
}

fn sv_boundary_edge(
    field: &SVContourField,
    threshold: f64,
    start: (usize, usize),
    end: (usize, usize),
    segments: &mut Vec<SVContourSegment>,
) {
    let start_point = SVContourPoint::new(
        field.x_coordinates()[start.0],
        field.y_coordinates()[start.1],
    );
    let end_point = SVContourPoint::new(field.x_coordinates()[end.0], field.y_coordinates()[end.1]);
    let start_value = field.value_at(start.0, start.1);
    let end_value = field.value_at(end.0, end.1);
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
