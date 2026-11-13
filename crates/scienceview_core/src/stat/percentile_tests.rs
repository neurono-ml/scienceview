//! Companion tests for the percentile kernel.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_stat::{SVPercentileOptions, SVPercentileResult, SVStat, SVStatInput, SVStatResult};

fn apply_percentile(options: SVPercentileOptions, input: SVStatInput<'_>) -> SVPercentileResult {
    match SVStat::percentile(options)
        .apply(input)
        .expect("the percentile configuration is valid")
    {
        SVStatResult::Quantiles(result) => result,
        other => panic!("the percentile stat must produce quantiles, received {other:?}"),
    }
}

#[test]
fn the_default_quantiles_are_the_three_quartiles() {
    let result = apply_percentile(
        SVPercentileOptions::new(),
        SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0, 3.0, 4.0])),
    );

    assert_eq!(result.quantiles(), &[0.25, 0.5, 0.75]);
    assert_eq!(result.values(), &[1.75, 2.5, 3.25]);
}

#[test]
fn the_extreme_quantiles_are_the_minimum_and_the_maximum() {
    let result = apply_percentile(
        SVPercentileOptions::new().quantiles(vec![0.0, 1.0]),
        SVStatInput::new(SVColumnView::Dense(&[3.0, 1.0, 2.0])),
    );

    assert_eq!(result.values(), &[1.0, 3.0]);
}

#[test]
fn a_sparse_column_quantile_includes_its_implicit_zeros() {
    let indices = [2usize];
    let values = [4.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let result = apply_percentile(SVPercentileOptions::new().quantiles(vec![0.5, 1.0]), input);

    assert_eq!(result.values(), &[0.0, 4.0]);
}

#[test]
fn a_streaming_percentile_reports_that_it_is_windowed() {
    let window = [1.0, 2.0, 3.0, 4.0];
    let view = SVStreamingView::new(&window, 9, 5);
    let input = SVStatInput::new(SVColumnView::Streaming(view));

    let result = apply_percentile(SVPercentileOptions::new(), input);

    assert!(result.is_windowed());
}

#[test]
fn a_dense_percentile_is_not_windowed() {
    let result = apply_percentile(
        SVPercentileOptions::new(),
        SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0])),
    );

    assert!(!result.is_windowed());
}

#[test]
fn a_quantile_outside_the_unit_interval_is_rejected() {
    let error = SVStat::percentile(SVPercentileOptions::new().quantiles(vec![1.5]))
        .apply(SVStatInput::new(SVColumnView::Dense(&[1.0])))
        .expect_err("a quantile lives in the unit interval");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_non_finite_quantile_is_rejected() {
    let error = SVStat::percentile(SVPercentileOptions::new().quantiles(vec![f64::NAN]))
        .apply(SVStatInput::new(SVColumnView::Dense(&[1.0])))
        .expect_err("NaN is not a quantile");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_empty_column_is_rejected() {
    let error = SVStat::percentile(SVPercentileOptions::new())
        .apply(SVStatInput::new(SVColumnView::Dense(&[])))
        .expect_err("there is no quantile of nothing");

    assert_eq!(
        error,
        SVError::EmptyInput {
            operation: "percentile"
        }
    );
}

#[test]
fn the_quantiles_are_independent_of_the_input_order() {
    let first = apply_percentile(
        SVPercentileOptions::new(),
        SVStatInput::new(SVColumnView::Dense(&[4.0, 2.0, 1.0, 3.0])),
    );
    let second = apply_percentile(
        SVPercentileOptions::new(),
        SVStatInput::new(SVColumnView::Dense(&[1.0, 2.0, 3.0, 4.0])),
    );

    assert_eq!(first.values(), second.values());
}

#[test]
fn one_observation_is_every_quantile() {
    let result = apply_percentile(
        SVPercentileOptions::new(),
        SVStatInput::new(SVColumnView::Dense(&[7.0])),
    );

    assert_eq!(result.values(), &[7.0, 7.0, 7.0]);
}

#[test]
fn a_non_finite_observation_is_rejected_with_its_position() {
    let values = [1.0, 2.0, f64::NAN, 4.0];

    let error = SVStat::percentile(SVPercentileOptions::new())
        .apply(SVStatInput::new(SVColumnView::Dense(&values)))
        .expect_err("NaN is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "percentile",
            position: 2,
        }
    );
}

#[test]
fn a_configured_weight_column_is_rejected_instead_of_ignored() {
    let values = [1.0, 2.0, 3.0];
    let weights = [1.0, 1.0, 1.0];
    let error = SVStat::percentile(SVPercentileOptions::new())
        .apply(
            SVStatInput::new(SVColumnView::Dense(&values))
                .with_weights(SVColumnView::Dense(&weights)),
        )
        .expect_err("percentiles do not accept weights");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
