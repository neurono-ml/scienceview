//! Executable specification of the viewport description.

use crate::sv_layout::SVViewport;

const TOLERANCE: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn the_default_viewport_is_a_landscape_box() {
    let viewport = SVViewport::default();

    assert_close(viewport.width(), 640.0);
    assert_close(viewport.height(), 480.0);
    assert_close(viewport.device_pixel_ratio(), 1.0);
    assert_close(viewport.cell_width(), 1.0);
    assert_close(viewport.cell_height(), 1.0);
}

#[test]
fn non_finite_and_negative_sizes_become_zero() {
    let viewport = SVViewport::new(f64::NAN, -10.0);

    assert_close(viewport.width(), 0.0);
    assert_close(viewport.height(), 0.0);
}

#[test]
fn the_device_pixel_ratio_scales_the_physical_size() {
    let viewport = SVViewport::new(400.0, 300.0).with_device_pixel_ratio(2.0);

    assert_close(viewport.physical_width(), 800.0);
    assert_close(viewport.physical_height(), 600.0);
}

#[test]
fn an_invalid_device_pixel_ratio_falls_back_to_one() {
    let viewport = SVViewport::new(400.0, 300.0).with_device_pixel_ratio(f64::NAN);
    assert_close(viewport.device_pixel_ratio(), 1.0);

    let zero = SVViewport::new(400.0, 300.0).with_device_pixel_ratio(0.0);
    assert_close(zero.device_pixel_ratio(), 1.0);
}

#[test]
fn a_cell_grid_reports_how_many_cells_fit() {
    let viewport = SVViewport::new(800.0, 480.0).with_cell_grid(8.0, 16.0);

    assert_eq!(viewport.cells_wide(), 100);
    assert_eq!(viewport.cells_high(), 30);
}

#[test]
fn an_invalid_cell_grid_falls_back_to_one_pixel() {
    let viewport = SVViewport::new(800.0, 480.0).with_cell_grid(0.0, f64::NAN);

    assert_close(viewport.cell_width(), 1.0);
    assert_close(viewport.cell_height(), 1.0);
    assert_eq!(viewport.cells_wide(), 800);
}

#[test]
fn snapping_rounds_to_the_nearest_cell() {
    let viewport = SVViewport::new(800.0, 480.0).with_cell_grid(8.0, 16.0);

    assert_close(viewport.snap_horizontal(11.0), 8.0);
    assert_close(viewport.snap_horizontal(13.0), 16.0);
    assert_close(viewport.snap_vertical(20.0), 16.0);
    assert_close(viewport.snap_vertical(25.0), 32.0);
}

#[test]
fn snapping_a_non_finite_coordinate_yields_zero() {
    let viewport = SVViewport::new(800.0, 480.0).with_cell_grid(8.0, 16.0);

    assert_close(viewport.snap_horizontal(f64::NAN), 0.0);
    assert_close(viewport.snap_vertical(f64::INFINITY), 0.0);
}
