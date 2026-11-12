//! Companion tests for the numerically stable primitives.

use crate::sv_stat::numeric::{SVCompensatedSum, SVWelford};

#[test]
fn the_compensated_sum_survives_values_of_different_magnitude() {
    let mut sum = SVCompensatedSum::new();
    sum.add(1.0e16);
    sum.add(1.0);
    sum.add(-1.0e16);

    assert_eq!(sum.value(), 1.0);
}

#[test]
fn welford_matches_the_hand_computed_variance() {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let mut accumulator = SVWelford::new();
    values
        .iter()
        .copied()
        .for_each(|value| accumulator.add(value));

    assert_eq!(accumulator.count(), 8);
    assert_eq!(accumulator.mean(), Some(5.0));
    assert_eq!(accumulator.sample_variance(), Some(32.0 / 7.0));
}

#[test]
fn welford_folds_repeated_values_in_constant_time() {
    let mut repeated = SVWelford::new();
    repeated.add_repeated(0.0, 1_000_000);

    let mut one_by_one = SVWelford::new();
    for _ in 0..1_000_000 {
        one_by_one.add(0.0);
    }

    assert_eq!(repeated.count(), one_by_one.count());
    assert_eq!(repeated.mean(), one_by_one.mean());
    assert_eq!(repeated.sample_variance(), one_by_one.sample_variance());
}

#[test]
fn one_observation_has_no_sample_variance() {
    let mut accumulator = SVWelford::new();
    accumulator.add(3.0);

    assert_eq!(accumulator.mean(), Some(3.0));
    assert_eq!(accumulator.sample_variance(), None);
    assert_eq!(accumulator.sample_standard_deviation(), None);
}

#[test]
fn the_normal_quantile_matches_the_reference_values() {
    use crate::sv_stat::numeric::sv_normal_quantile;

    assert_eq!(sv_normal_quantile(0.5), 0.0);
    assert!((sv_normal_quantile(0.975) - 1.959_963_984_540_054).abs() < 1e-8);
    assert!((sv_normal_quantile(0.025) + 1.959_963_984_540_054).abs() < 1e-8);
    assert!(sv_normal_quantile(0.999) > 3.09);
    assert_eq!(sv_normal_quantile(0.0), f64::NEG_INFINITY);
    assert_eq!(sv_normal_quantile(1.0), f64::INFINITY);
}
