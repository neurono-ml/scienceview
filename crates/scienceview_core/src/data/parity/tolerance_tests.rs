//! Companion tests for the documented tolerance policy.

use crate::sv_data::SVRepresentationTolerance;

#[test]
fn exact_tolerance_only_accepts_identical_values() {
    assert!(SVRepresentationTolerance::EXACT.is_within_tolerance(1.0, 1.0));
    assert!(!SVRepresentationTolerance::EXACT.is_within_tolerance(1.0, 1.0 + f64::EPSILON));
}

#[test]
fn the_default_tolerance_absorbs_rounding_noise() {
    let tolerance = SVRepresentationTolerance::default();

    assert!(tolerance.is_within_tolerance(1.0, 1.0 + 1e-15));
    assert!(tolerance.is_within_tolerance(0.0, 1e-15));
    assert!(!tolerance.is_within_tolerance(0.0, 1e-6));
}

#[test]
fn the_relative_margin_scales_with_the_operands() {
    let tolerance = SVRepresentationTolerance::new(0.0, 1e-9);

    assert!(tolerance.is_within_tolerance(1.0e9, 1.0e9 + 0.5));
    assert!(!tolerance.is_within_tolerance(1.0, 1.0 + 1e-6));
}

#[test]
fn infinities_and_nan_are_compared_honestly() {
    let tolerance = SVRepresentationTolerance::DEFAULT;

    assert!(tolerance.is_within_tolerance(f64::INFINITY, f64::INFINITY));
    assert!(!tolerance.is_within_tolerance(f64::INFINITY, f64::NEG_INFINITY));
    assert!(!tolerance.is_within_tolerance(f64::NAN, f64::NAN));
}
