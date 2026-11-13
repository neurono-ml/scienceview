//! Grid placement, spacing and viewport re-resolution.

use crate::sv_error::SVError;
use crate::sv_grid::{SVDashboardGrid, SVGridPlacement};
use crate::sv_layout::SVViewport;

fn sv_assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {expected}, received {actual}"
    );
}

#[test]
fn a_spanning_placement_occupies_the_width_of_both_columns() {
    let grid = SVDashboardGrid::new(1, 3).expect("the grid is valid");
    let solution = grid
        .solve(SVViewport::new(300.0, 100.0))
        .expect("the viewport is valid");

    let spanning = SVGridPlacement::new(0, 0).with_column_span(2);
    let single = SVGridPlacement::new(0, 2);

    let spanning_rectangle = solution.rectangle_for(&spanning);
    sv_assert_close(spanning_rectangle.x, 0.0);
    sv_assert_close(spanning_rectangle.width, 200.0);
    sv_assert_close(solution.rectangle_for(&single).x, 200.0);
}

#[test]
fn a_spanning_cell_covers_the_union_of_rows_and_columns() {
    let grid = SVDashboardGrid::new(2, 2).expect("the grid is valid");
    let solution = grid
        .solve(SVViewport::new(200.0, 100.0))
        .expect("the viewport is valid");

    let rectangle =
        solution.rectangle_for(&SVGridPlacement::new(0, 0).with_row_span(2).with_column_span(2));

    sv_assert_close(rectangle.x, 0.0);
    sv_assert_close(rectangle.y, 0.0);
    sv_assert_close(rectangle.width, 200.0);
    sv_assert_close(rectangle.height, 100.0);
}

#[test]
fn a_viewport_change_resolves_every_cell_again() {
    let grid = SVDashboardGrid::new(1, 2).expect("the grid is valid");
    let narrow = grid
        .solve(SVViewport::new(300.0, 100.0))
        .expect("the viewport is valid");
    let wide = grid
        .solve(SVViewport::new(600.0, 100.0))
        .expect("the viewport is valid");

    let placement = SVGridPlacement::new(0, 0);
    sv_assert_close(narrow.rectangle_for(&placement).width, 150.0);
    sv_assert_close(wide.rectangle_for(&placement).width, 300.0);
}

#[test]
fn row_and_column_gaps_are_reserved_between_cells() {
    let grid = SVDashboardGrid::new(2, 2)
        .expect("the grid is valid")
        .with_horizontal_gap(10.0)
        .with_vertical_gap(10.0);
    let solution = grid
        .solve(SVViewport::new(210.0, 110.0))
        .expect("the viewport is valid");

    let lower_right = solution.rectangle_for(&SVGridPlacement::new(1, 1));
    sv_assert_close(lower_right.x, 110.0);
    sv_assert_close(lower_right.y, 60.0);
    sv_assert_close(lower_right.width, 100.0);
    sv_assert_close(lower_right.height, 50.0);
}

#[test]
fn a_placement_outside_the_grid_is_rejected() {
    let placement = SVGridPlacement::new(0, 1);
    let error = placement
        .validate("panel", 1, 1)
        .expect_err("the column is outside the grid");

    assert!(matches!(
        error,
        SVError::DashboardPlacementOutOfBounds {
            row: 0,
            column: 1,
            rows: 1,
            columns: 1,
            ..
        }
    ));
}

#[test]
fn a_grid_needs_at_least_one_row_and_column() {
    assert!(matches!(
        SVDashboardGrid::new(0, 1),
        Err(SVError::InvalidDashboardGrid { .. })
    ));
    assert!(matches!(
        SVDashboardGrid::new(1, 0),
        Err(SVError::InvalidDashboardGrid { .. })
    ));
}

#[test]
fn a_negative_or_non_finite_gap_is_rejected_at_solve_time() {
    let grid = SVDashboardGrid::new(1, 1)
        .expect("the grid is valid")
        .with_horizontal_gap(-1.0);

    assert!(matches!(
        grid.solve(SVViewport::new(100.0, 100.0)),
        Err(SVError::InvalidDashboardGrid { .. })
    ));
}

#[test]
fn every_cell_rectangle_stays_inside_the_viewport() {
    let grid = SVDashboardGrid::new(3, 4)
        .expect("the grid is valid")
        .with_horizontal_gap(4.0)
        .with_vertical_gap(6.0);
    let solution = grid
        .solve(SVViewport::new(640.0, 480.0))
        .expect("the viewport is valid");

    for row in 0..3 {
        for column in 0..4 {
            let rectangle = solution.rectangle_for(&SVGridPlacement::new(row, column));
            assert!(rectangle.x >= 0.0);
            assert!(rectangle.y >= 0.0);
            assert!(rectangle.right() <= 640.0 + 1e-9);
            assert!(rectangle.bottom() <= 480.0 + 1e-9);
            assert!(rectangle.width > 0.0);
            assert!(rectangle.height > 0.0);
        }
    }
}
