//! Companion tests for the empirical-CDF kernel.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_stat::{
    SVEmpiricalCdfOptions, SVEmpiricalCdfResult, SVStat, SVStatInput, SVStatResult,
};

fn apply_cdf(options: SVEmpiricalCdfOptions, input: SVStatInput<'_>) -> SVEmpiricalCdfResult {
    match SVStat::empirical_cdf(options)
        .apply(input)
        .expect("the empirical CDF configuration is valid")
    {
        SVStatResult::Cumulative(result) => result,
        other => panic!("the CDF stat must produce a cumulative curve, received {other:?}"),
    }
}

fn dense(values: &[f64]) -> SVStatInput<'_> {
    SVStatInput::new(SVColumnView::Dense(values))
}

#[test]
fn the_cumulative_curve_has_one_breakpoint_per_distinct_value() {
    let result = apply_cdf(SVEmpiricalCdfOptions::new(), dense(&[3.0, 1.0, 3.0, 2.0]));

    assert_eq!(result.points(), &[1.0, 2.0, 3.0]);
    assert_eq!(result.probabilities(), &[0.25, 0.5, 1.0]);
}

#[test]
fn disabling_the_normalisation_returns_cumulative_counts() {
    let result = apply_cdf(
        SVEmpiricalCdfOptions::new().normalise(false),
        dense(&[1.0, 1.0, 2.0]),
    );

    assert_eq!(result.probabilities(), &[2.0, 3.0]);
}

#[test]
fn a_sparse_column_cdf_includes_its_implicit_zeros() {
    let indices = [2usize];
    let values = [4.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");

    let result = apply_cdf(
        SVEmpiricalCdfOptions::new(),
        SVStatInput::new(SVColumnView::Sparse(sparse)),
    );

    assert_eq!(result.points(), &[0.0, 4.0]);
    assert_eq!(result.probabilities(), &[0.75, 1.0]);
}

#[test]
fn a_streaming_cdf_reports_that_it_is_windowed() {
    let window = [1.0, 2.0];
    let view = SVStreamingView::new(&window, 8, 6);

    let result = apply_cdf(
        SVEmpiricalCdfOptions::new(),
        SVStatInput::new(SVColumnView::Streaming(view)),
    );

    assert!(result.is_windowed());
}

#[test]
fn a_dense_cdf_is_not_windowed() {
    let result = apply_cdf(SVEmpiricalCdfOptions::new(), dense(&[1.0]));

    assert!(!result.is_windowed());
}

#[test]
fn one_observation_has_one_breakpoint_at_probability_one() {
    let result = apply_cdf(SVEmpiricalCdfOptions::new(), dense(&[7.0]));

    assert_eq!(result.points(), &[7.0]);
    assert_eq!(result.probabilities(), &[1.0]);
}

#[test]
fn the_cumulative_curve_is_independent_of_the_input_order() {
    let first = apply_cdf(SVEmpiricalCdfOptions::new(), dense(&[3.0, 1.0, 2.0]));
    let second = apply_cdf(SVEmpiricalCdfOptions::new(), dense(&[1.0, 2.0, 3.0]));

    assert_eq!(first.points(), second.points());
    assert_eq!(first.probabilities(), second.probabilities());
}

#[test]
fn an_empty_column_is_rejected() {
    let error = SVStat::empirical_cdf(SVEmpiricalCdfOptions::new())
        .apply(dense(&[]))
        .expect_err("there is no CDF of nothing");

    assert_eq!(
        error,
        SVError::EmptyInput {
            operation: "empirical_cdf",
        }
    );
}

#[test]
fn a_non_finite_value_is_rejected() {
    let error = SVStat::empirical_cdf(SVEmpiricalCdfOptions::new())
        .apply(dense(&[f64::NAN]))
        .expect_err("NaN is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "empirical_cdf",
            position: 0,
        }
    );
}
