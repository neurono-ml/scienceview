//! Companion tests for the four scale transforms and their inverses.

use crate::sv_scale::{SVDomain, SVScaleTransform};

const VALUES: [f64; 7] = [-1_000.0, -1.0, -0.001, 0.0, 0.001, 1.0, 1_000.0];

#[test]
fn the_transforms_are_spelled_out() {
    assert_eq!(SVScaleTransform::Linear.as_str(), "Linear");
    assert_eq!(SVScaleTransform::Logarithmic.as_str(), "Logarithmic");
    assert_eq!(
        SVScaleTransform::SymmetricLogarithmic.as_str(),
        "SymmetricLogarithmic"
    );
    assert_eq!(SVScaleTransform::SquareRoot.as_str(), "SquareRoot");
}

#[test]
fn the_default_transform_is_linear() {
    assert_eq!(SVScaleTransform::default(), SVScaleTransform::Linear);
}

#[test]
fn the_linear_transform_is_the_identity() {
    let transform = SVScaleTransform::Linear;

    for value in VALUES {
        assert_eq!(transform.forward(value), Some(value));
        assert_eq!(transform.inverse(value), Some(value));
    }
}

#[test]
fn the_logarithmic_transform_works_in_decades() {
    let transform = SVScaleTransform::Logarithmic;

    assert_eq!(transform.forward(1.0), Some(0.0));
    assert_eq!(transform.forward(10.0), Some(1.0));
    assert_eq!(transform.forward(100.0), Some(2.0));
    assert_eq!(transform.inverse(2.0), Some(100.0));
}

#[test]
fn the_logarithmic_transform_rejects_non_positive_values() {
    let transform = SVScaleTransform::Logarithmic;

    assert_eq!(transform.forward(0.0), None);
    assert_eq!(transform.forward(-1.0), None);
    assert_eq!(transform.forward(f64::NAN), None);
}

#[test]
fn the_symmetric_logarithmic_transform_is_odd_and_defined_at_zero() {
    let transform = SVScaleTransform::SymmetricLogarithmic;

    let positive = transform.forward(1.0).expect("one is in the domain");
    let negative = transform.forward(-1.0).expect("minus one is in the domain");

    assert_eq!(negative, -positive);
    assert_eq!(transform.forward(0.0), Some(0.0));
}

#[test]
fn the_square_root_transform_rejects_negative_values() {
    let transform = SVScaleTransform::SquareRoot;

    assert_eq!(transform.forward(4.0), Some(2.0));
    assert_eq!(transform.forward(-1.0), None);
    assert_eq!(transform.inverse(2.0), Some(4.0));
}

#[test]
fn every_transform_inverts_its_own_forward_map() {
    for (transform, values) in [
        (SVScaleTransform::Linear, VALUES.as_slice()),
        (SVScaleTransform::SymmetricLogarithmic, VALUES.as_slice()),
        (SVScaleTransform::SquareRoot, &VALUES[3..]),
        (SVScaleTransform::Logarithmic, &VALUES[4..]),
    ] {
        for value in values.iter().copied().filter(|value| *value != 0.0) {
            let transformed = transform
                .forward(value)
                .unwrap_or_else(|| panic!("{transform} must accept {value}"));
            let restored = transform
                .inverse(transformed)
                .expect("the forward map stays inside the inverse domain");

            assert!(
                (restored - value).abs() <= 1e-12 * value.abs().max(1.0),
                "{transform} did not restore {value}: received {restored}"
            );
        }
    }
}

#[test]
fn a_domain_is_defined_only_when_its_lower_bound_is_in_the_transform_domain() {
    let positive = SVDomain::new(0.001, 100.0).expect("valid");
    let crossing_zero = SVDomain::new(0.0, 100.0).expect("valid");
    let negative = SVDomain::new(-1.0, 100.0).expect("valid");

    assert!(SVScaleTransform::Logarithmic.domain_is_defined(positive));
    assert!(!SVScaleTransform::Logarithmic.domain_is_defined(crossing_zero));
    assert!(!SVScaleTransform::Logarithmic.domain_is_defined(negative));
    assert!(SVScaleTransform::SquareRoot.domain_is_defined(crossing_zero));
    assert!(!SVScaleTransform::SquareRoot.domain_is_defined(negative));
    assert!(SVScaleTransform::SymmetricLogarithmic.domain_is_defined(negative));
    assert!(SVScaleTransform::Linear.domain_is_defined(negative));
}
