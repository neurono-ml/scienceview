//! Companion tests for the aggregation kernel.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_stat::{
    SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions, SVAggregateResult, SVStat,
    SVStatInput, SVStatResult,
};

fn apply_aggregate(options: SVAggregateOptions, input: SVStatInput<'_>) -> SVAggregateResult {
    match SVStat::aggregate(options)
        .apply(input)
        .expect("the aggregation configuration is valid")
    {
        SVStatResult::Groups(result) => result,
        other => panic!("the aggregate stat must produce groups, received {other:?}"),
    }
}

fn ungrouped(values: &[f64]) -> SVStatInput<'_> {
    SVStatInput::new(SVColumnView::Dense(values))
}

fn assert_value(result: &SVAggregateResult, expected: f64) {
    assert_eq!(result.groups().len(), 1);
    let value = result.groups()[0].value();
    assert!(
        (value - expected).abs() < 1e-12,
        "expected {expected}, received {value}"
    );
}

#[test]
fn the_mean_is_the_arithmetic_mean_of_one_group() {
    let result = apply_aggregate(SVAggregateOptions::new(), ungrouped(&[1.0, 2.0, 3.0, 4.0]));

    assert_value(&result, 2.5);
    assert_eq!(result.groups()[0].group(), None);
    assert_eq!(result.groups()[0].number_of_observations(), 4);
}

#[test]
fn every_aggregate_method_produces_its_hand_computed_value() {
    let values = [1.0, 2.0, 3.0, 4.0];

    assert_value(
        &apply_aggregate(
            SVAggregateOptions::new().method(SVAggregateMethod::Sum),
            ungrouped(&values),
        ),
        10.0,
    );
    assert_value(
        &apply_aggregate(
            SVAggregateOptions::new().method(SVAggregateMethod::Median),
            ungrouped(&values),
        ),
        2.5,
    );
    assert_value(
        &apply_aggregate(
            SVAggregateOptions::new().method(SVAggregateMethod::Minimum),
            ungrouped(&values),
        ),
        1.0,
    );
    assert_value(
        &apply_aggregate(
            SVAggregateOptions::new().method(SVAggregateMethod::Maximum),
            ungrouped(&values),
        ),
        4.0,
    );
    assert_value(
        &apply_aggregate(
            SVAggregateOptions::new().method(SVAggregateMethod::Count),
            ungrouped(&values),
        ),
        4.0,
    );
    assert_value(
        &apply_aggregate(
            SVAggregateOptions::new().method(SVAggregateMethod::StandardDeviation),
            ungrouped(&values),
        ),
        (5.0f64 / 3.0).sqrt(),
    );
}

#[test]
fn the_groups_follow_their_first_appearance_and_carry_their_code() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let groups = [7.0, 7.0, 2.0, 2.0];
    let input = ungrouped(&values).with_groups(SVColumnView::Dense(&groups));

    let result = apply_aggregate(SVAggregateOptions::new(), input);

    assert_eq!(result.groups().len(), 2);
    assert_eq!(result.groups()[0].group(), Some(7.0));
    assert_eq!(result.groups()[0].value(), 1.5);
    assert_eq!(result.groups()[1].group(), Some(2.0));
    assert_eq!(result.groups()[1].value(), 3.5);
}

#[test]
fn a_standard_deviation_error_band_is_symmetric_around_the_value() {
    let options = SVAggregateOptions::new()
        .method(SVAggregateMethod::Mean)
        .error(SVAggregateErrorKind::StandardDeviation);
    let result = apply_aggregate(options, ungrouped(&[1.0, 2.0, 3.0, 4.0]));
    let band = result.groups()[0]
        .error()
        .expect("the error band is configured");

    let deviation = (5.0f64 / 3.0).sqrt();
    assert!((band.lower() - (2.5 - deviation)).abs() < 1e-12);
    assert!((band.upper() - (2.5 + deviation)).abs() < 1e-12);
}

#[test]
fn a_standard_error_band_divides_the_deviation_by_the_root_of_the_count() {
    let options = SVAggregateOptions::new().error(SVAggregateErrorKind::StandardError);
    let result = apply_aggregate(options, ungrouped(&[1.0, 2.0, 3.0, 4.0]));
    let band = result.groups()[0].error().expect("the error band exists");

    let standard_error = (5.0f64 / 3.0).sqrt() / 2.0;
    assert!((band.upper() - 2.5 - standard_error).abs() < 1e-12);
}

#[test]
fn a_percentile_error_band_is_the_centred_interval() {
    let options = SVAggregateOptions::new()
        .method(SVAggregateMethod::Median)
        .error(SVAggregateErrorKind::Percentile(0.5));
    let result = apply_aggregate(options, ungrouped(&[1.0, 2.0, 3.0, 4.0]));
    let band = result.groups()[0].error().expect("the error band exists");

    assert!((band.lower() - 1.75).abs() < 1e-12);
    assert!((band.upper() - 3.25).abs() < 1e-12);
}

#[test]
fn weights_turn_the_mean_into_a_weighted_mean() {
    let values = [1.0, 3.0];
    let weights = [1.0, 3.0];
    let input = ungrouped(&values).with_weights(SVColumnView::Dense(&weights));

    let result = apply_aggregate(SVAggregateOptions::new(), input);

    assert_value(&result, 2.5);
}

#[test]
fn weights_scale_a_weighted_sum() {
    let values = [1.0, 3.0];
    let weights = [1.0, 3.0];
    let input = ungrouped(&values).with_weights(SVColumnView::Dense(&weights));

    let result = apply_aggregate(
        SVAggregateOptions::new().method(SVAggregateMethod::Sum),
        input,
    );

    assert_value(&result, 10.0);
}

#[test]
fn weights_on_a_method_that_cannot_use_them_are_rejected() {
    let values = [1.0, 3.0];
    let weights = [1.0, 3.0];
    let input = ungrouped(&values).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Median))
        .apply(input)
        .expect_err("a median is not weighted");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_negative_weight_is_rejected() {
    let values = [1.0, 3.0];
    let weights = [1.0, -3.0];
    let input = ungrouped(&values).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(input)
        .expect_err("weights cannot be negative");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_group_column_of_a_different_length_is_rejected() {
    let values = [1.0, 2.0, 3.0];
    let groups = [1.0, 1.0];
    let input = ungrouped(&values).with_groups(SVColumnView::Dense(&groups));

    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(input)
        .expect_err("columns must agree in length");

    assert_eq!(
        error,
        SVError::MismatchedStatisticLengths {
            operation: "aggregate",
            first: 3,
            second: 2,
        }
    );
}

#[test]
fn an_empty_column_is_rejected() {
    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(ungrouped(&[]))
        .expect_err("there is nothing to aggregate");

    assert_eq!(
        error,
        SVError::EmptyInput {
            operation: "aggregate"
        }
    );
}

#[test]
fn a_non_finite_value_is_rejected() {
    let values = [1.0, f64::NAN];
    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(ungrouped(&values))
        .expect_err("NaN is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "aggregate",
            position: 1,
        }
    );
}

#[test]
fn a_sparse_column_aggregates_without_being_densified() {
    let indices = [1usize, 3];
    let values = [2.0, 4.0];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let result = apply_aggregate(SVAggregateOptions::new(), input);

    assert_value(&result, 1.0);
    assert_eq!(result.groups()[0].number_of_observations(), 6);
}

#[test]
fn one_observation_has_no_error_band() {
    let options = SVAggregateOptions::new().error(SVAggregateErrorKind::StandardDeviation);
    let result = apply_aggregate(options, ungrouped(&[3.0]));

    assert_eq!(result.groups()[0].value(), 3.0);
    assert!(result.groups()[0].error().is_none());
}

#[test]
fn all_equal_observations_have_zero_standard_deviation() {
    let result = apply_aggregate(
        SVAggregateOptions::new().method(SVAggregateMethod::StandardDeviation),
        ungrouped(&[5.0, 5.0, 5.0]),
    );

    assert_value(&result, 0.0);
}

#[test]
fn the_aggregate_is_independent_of_the_input_order() {
    let first = apply_aggregate(SVAggregateOptions::new(), ungrouped(&[4.0, 1.0, 3.0, 2.0]));
    let second = apply_aggregate(SVAggregateOptions::new(), ungrouped(&[1.0, 2.0, 3.0, 4.0]));

    assert_eq!(first.groups()[0].value(), second.groups()[0].value());
}

#[test]
fn a_sparse_column_folds_its_implicit_zeros_in_stored_time() {
    // A logical length no loop could traverse turns any accidental
    // `0..length` iteration into a hang; the fold reads one stored entry and
    // one multiplicity instead.
    let indices = [2usize];
    let values = [4.0];
    let logical_length = 1_000_000_000_000usize;
    let sparse = SVSparseView::new(&indices, &values, logical_length).expect("valid sparse");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let mean = apply_aggregate(SVAggregateOptions::new(), input);
    let value = mean.groups()[0].value();
    assert!(
        (value - 4.0 / logical_length as f64).abs() < 1e-15,
        "the mean folds the implicit zeros: {value}"
    );
    assert_eq!(mean.groups()[0].number_of_observations(), logical_length);

    let count = apply_aggregate(
        SVAggregateOptions::new().method(SVAggregateMethod::Count),
        input,
    );
    assert_eq!(count.groups()[0].value(), logical_length as f64);
}

#[test]
fn a_sparse_median_includes_its_implicit_zeros() {
    let indices = [0usize, 5];
    let values = [-2.0, 8.0];
    let sparse = SVSparseView::new(&indices, &values, 10).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let result = apply_aggregate(
        SVAggregateOptions::new().method(SVAggregateMethod::Median),
        input,
    );

    assert_value(&result, 0.0);
    assert_eq!(result.groups()[0].number_of_observations(), 10);
}

#[test]
fn the_aggregate_agrees_across_the_three_representations() {
    let dataset = [1.0, 0.0, 3.0, 0.0, 5.0, 7.0, 0.0, 2.0, 9.0];
    let methods = [
        SVAggregateMethod::Mean,
        SVAggregateMethod::Sum,
        SVAggregateMethod::Median,
        SVAggregateMethod::StandardDeviation,
        SVAggregateMethod::Count,
    ];

    for method in methods {
        let options = SVAggregateOptions::new().method(method);
        crate::sv_data::sv_assert_representation_parity(&dataset, |view| match SVStat::aggregate(
            options,
        )
        .apply(SVStatInput::new(view))
        .expect("the dataset aggregates")
        {
            SVStatResult::Groups(result) => {
                result.groups().iter().map(|group| group.value()).collect()
            }
            other => panic!("the aggregate stat must produce groups, received {other:?}"),
        });
    }
}

#[test]
fn a_sparse_value_column_is_grouped_by_a_dense_group_column() {
    let indices = [1usize, 4];
    let values = [2.0, 4.0];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse storage");
    let groups = [1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
    let input =
        SVStatInput::new(SVColumnView::Sparse(sparse)).with_groups(SVColumnView::Dense(&groups));

    let result = apply_aggregate(SVAggregateOptions::new(), input);

    assert_eq!(result.groups().len(), 2);
    assert!((result.groups()[0].value() - 2.0 / 3.0).abs() < 1e-12);
    assert!((result.groups()[1].value() - 4.0 / 3.0).abs() < 1e-12);
    assert_eq!(result.groups()[0].number_of_observations(), 3);
    assert_eq!(result.groups()[1].number_of_observations(), 3);
}

#[test]
fn a_sparse_value_column_is_grouped_by_a_sparse_group_column() {
    let value_indices = [1usize, 4];
    let values = [2.0, 4.0];
    let values = SVSparseView::new(&value_indices, &values, 8).expect("valid sparse storage");
    let group_indices = [0usize, 5, 7];
    let group_codes = [2.0, 1.0, 3.0];
    let groups = SVSparseView::new(&group_indices, &group_codes, 8).expect("valid sparse storage");
    let input =
        SVStatInput::new(SVColumnView::Sparse(values)).with_groups(SVColumnView::Sparse(groups));

    let result = apply_aggregate(
        SVAggregateOptions::new().method(SVAggregateMethod::Sum),
        input,
    );

    let group_zero = result
        .groups()
        .iter()
        .find(|group| group.group() == Some(0.0))
        .expect("the stored values sit where the group column is implicit");
    assert_eq!(group_zero.value(), 6.0);
    assert_eq!(group_zero.number_of_observations(), 5);

    // A stored group code whose position holds an implicit zero contributes a
    // zero value, not a missing observation.
    for code in [1.0, 2.0, 3.0] {
        let group = result
            .groups()
            .iter()
            .find(|group| group.group() == Some(code))
            .unwrap_or_else(|| panic!("group {code} exists"));
        assert_eq!(group.value(), 0.0);
        assert_eq!(group.number_of_observations(), 1);
    }
}

#[test]
fn a_non_finite_weight_is_rejected_with_its_position() {
    let values = [1.0, 3.0];
    let weights = [1.0, f64::INFINITY];
    let input = ungrouped(&values).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(input)
        .expect_err("infinity is not a weight");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "aggregate",
            position: 1,
        }
    );
}

#[test]
fn a_non_finite_group_code_is_rejected_with_its_position() {
    let values = [1.0, 2.0];
    let groups = [1.0, f64::NAN];
    let input = ungrouped(&values).with_groups(SVColumnView::Dense(&groups));

    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(input)
        .expect_err("NaN is not a group code");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "aggregate",
            position: 1,
        }
    );
}

#[test]
fn a_non_finite_sparse_value_reports_its_logical_position() {
    let indices = [1usize, 4];
    let values = [2.0, f64::NAN];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(input)
        .expect_err("NaN is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "aggregate",
            position: 4,
        }
    );
}

#[test]
fn weights_that_sum_to_zero_are_rejected() {
    let values = [1.0, 3.0];
    let weights = [0.0, 0.0];
    let input = ungrouped(&values).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::aggregate(SVAggregateOptions::new())
        .apply(input)
        .expect_err("a weighted mean needs a positive total weight");

    assert_eq!(
        error,
        SVError::InvalidConfiguration {
            message: "the observation weights of a group sum to zero".to_string(),
        }
    );
}
