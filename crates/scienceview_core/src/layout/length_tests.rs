//! Executable specification of the length model and its constraints.

use crate::sv_layout::{SVLayoutConstraints, SVLength};

const TOLERANCE: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= TOLERANCE,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn absolute_length_ignores_the_available_space() {
    let length = SVLength::absolute(320.0);

    assert_close(length.resolve_width(1000.0, 50.0, 200.0), 320.0);
    assert_close(length.resolve_height(1000.0, 50.0, 200.0), 320.0);
}

#[test]
fn fraction_length_takes_a_share_of_the_available_space() {
    let length = SVLength::fraction(0.5);

    assert_close(length.resolve_width(800.0, 50.0, 200.0), 400.0);
    assert_close(length.resolve_height(600.0, 50.0, 200.0), 300.0);
}

#[test]
fn fraction_length_is_clamped_to_the_unit_interval() {
    assert_close(
        SVLength::fraction(2.0).resolve_width(800.0, 0.0, 0.0),
        800.0,
    );
    assert_close(SVLength::fraction(-1.0).resolve_width(800.0, 0.0, 0.0), 0.0);
}

#[test]
fn intrinsic_length_reports_the_measured_content() {
    let length = SVLength::intrinsic();

    assert_close(length.resolve_width(800.0, 275.0, 100.0), 275.0);
    assert_close(length.resolve_height(600.0, 125.0, 100.0), 125.0);
}

#[test]
fn aspect_ratio_length_derives_the_opposite_dimension() {
    let length = SVLength::aspect_ratio(2.0);

    // A width of 400 at ratio 2 gives a height of 200, and vice versa.
    assert_close(length.resolve_width(0.0, 0.0, 200.0), 400.0);
    assert_close(length.resolve_height(0.0, 0.0, 400.0), 200.0);
}

#[test]
fn aspect_ratio_with_a_non_positive_ratio_falls_back_to_one() {
    let length = SVLength::aspect_ratio(0.0);

    assert_close(length.resolve_width(0.0, 0.0, 200.0), 200.0);
    assert_close(length.resolve_height(0.0, 0.0, 200.0), 200.0);
}

#[test]
fn fill_length_adopts_the_available_space() {
    let length = SVLength::fill();

    assert_close(length.resolve_width(800.0, 50.0, 100.0), 800.0);
    assert_close(length.resolve_height(600.0, 50.0, 100.0), 600.0);
}

#[test]
fn non_finite_lengths_resolve_to_zero_instead_of_panicking() {
    assert_close(
        SVLength::absolute(f64::NAN).resolve_width(10.0, 0.0, 0.0),
        0.0,
    );
    assert_close(
        SVLength::absolute(f64::INFINITY).resolve_width(10.0, 0.0, 0.0),
        0.0,
    );
    assert_close(
        SVLength::fraction(f64::NAN).resolve_width(800.0, 0.0, 0.0),
        0.0,
    );
    assert_close(
        SVLength::intrinsic().resolve_width(f64::NAN, f64::NAN, 0.0),
        0.0,
    );
}

#[test]
fn minimum_width_is_honoured_when_the_container_is_smaller() {
    let constraints = SVLayoutConstraints::unbounded().with_minimum_width(200.0);

    assert_close(constraints.clamp_width(120.0), 200.0);
    assert_close(constraints.clamp_width(320.0), 320.0);
}

#[test]
fn maximum_width_is_honoured_when_the_container_is_larger() {
    let constraints = SVLayoutConstraints::unbounded().with_maximum_width(200.0);

    assert_close(constraints.clamp_width(120.0), 120.0);
    assert_close(constraints.clamp_width(320.0), 200.0);
}

#[test]
fn minimum_height_is_honoured_when_the_container_is_smaller() {
    let constraints = SVLayoutConstraints::unbounded().with_minimum_height(90.0);

    assert_close(constraints.clamp_height(40.0), 90.0);
    assert_close(constraints.clamp_height(120.0), 120.0);
}

#[test]
fn maximum_height_is_honoured_when_the_container_is_larger() {
    let constraints = SVLayoutConstraints::unbounded().with_maximum_height(90.0);

    assert_close(constraints.clamp_height(40.0), 40.0);
    assert_close(constraints.clamp_height(120.0), 90.0);
}

#[test]
fn an_inverted_constraint_pair_lets_the_minimum_win_deterministically() {
    let constraints = SVLayoutConstraints::unbounded()
        .with_minimum_width(300.0)
        .with_maximum_width(100.0)
        .with_minimum_height(50.0)
        .with_maximum_height(20.0);

    assert_close(constraints.clamp_width(10.0), 300.0);
    assert_close(constraints.clamp_width(1000.0), 300.0);
    assert_close(constraints.clamp_height(10.0), 50.0);
    assert_close(constraints.clamp_height(1000.0), 50.0);
}

#[test]
fn constraints_never_return_a_negative_or_non_finite_size() {
    let constraints = SVLayoutConstraints::unbounded()
        .with_minimum_width(f64::NAN)
        .with_maximum_width(f64::NAN)
        .with_minimum_height(-5.0)
        .with_maximum_height(f64::INFINITY);

    assert_close(constraints.clamp_width(-10.0), 0.0);
    assert_close(constraints.clamp_width(f64::NAN), 0.0);
    assert_close(constraints.clamp_height(-10.0), 0.0);
    assert_close(constraints.clamp_height(f64::INFINITY), 0.0);
}

#[test]
fn the_constraint_accessors_report_the_configured_bounds() {
    let constraints = SVLayoutConstraints::unbounded()
        .with_minimum_width(10.0)
        .with_maximum_width(20.0)
        .with_minimum_height(30.0)
        .with_maximum_height(40.0);

    assert_close(constraints.minimum_width(), 10.0);
    assert_close(constraints.maximum_width(), 20.0);
    assert_close(constraints.minimum_height(), 30.0);
    assert_close(constraints.maximum_height(), 40.0);
}
