//! Companion tests for the binning kernel.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_stat::{
    SVBinOptions, SVBinResult, SVNormalisation, SVStat, SVStatInput, SVStatResult,
};

fn apply_bin(options: SVBinOptions, values: &[f64]) -> SVBinResult {
    let input = SVStatInput::new(SVColumnView::Dense(values));
    match SVStat::bin(options)
        .apply(input)
        .expect("the binning configuration is valid")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    }
}

#[test]
fn explicit_edges_win_over_a_requested_bin_count() {
    let result = apply_bin(
        SVBinOptions::new()
            .bin_edges(vec![0.0, 1.0, 2.0, 3.0])
            .bin_count(99),
        &[0.1, 0.9, 1.0, 2.5],
    );

    assert_eq!(result.edges(), &[0.0, 1.0, 2.0, 3.0]);
    assert_eq!(result.counts(), &[2.0, 1.0, 1.0]);
}

#[test]
fn a_bin_width_derives_contiguous_edges() {
    let result = apply_bin(
        SVBinOptions::new().bin_width(2.0),
        &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
    );

    assert_eq!(result.edges(), &[0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
    assert_eq!(result.counts(), &[2.0, 2.0, 2.0, 2.0, 2.0]);
}

#[test]
fn a_bin_count_splits_the_derived_range_evenly() {
    let result = apply_bin(SVBinOptions::new().bin_count(4), &[0.0, 1.0, 2.0, 3.0]);

    assert_eq!(result.edges(), &[0.0, 0.75, 1.5, 2.25, 3.0]);
    assert_eq!(result.counts(), &[1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn probability_normalisation_divides_by_the_observed_total() {
    let result = apply_bin(
        SVBinOptions::new()
            .bin_edges(vec![0.0, 1.0, 2.0])
            .normalise(SVNormalisation::Probability),
        &[0.1, 0.2, 0.3, 1.5],
    );

    assert_eq!(result.counts(), &[0.75, 0.25]);
}

#[test]
fn density_normalisation_divides_by_total_and_bin_width() {
    let result = apply_bin(
        SVBinOptions::new()
            .bin_edges(vec![0.0, 2.0, 4.0])
            .normalise(SVNormalisation::Density),
        &[0.5, 1.5, 3.0, 3.5],
    );

    assert_eq!(result.counts(), &[0.25, 0.25]);
}

#[test]
fn the_cumulative_mode_accumulates_the_normalised_counts() {
    let result = apply_bin(
        SVBinOptions::new()
            .bin_edges(vec![0.0, 1.0, 2.0, 3.0])
            .normalise(SVNormalisation::Probability)
            .cumulative(true),
        &[0.1, 0.2, 1.5, 2.5],
    );

    assert_eq!(result.counts(), &[0.5, 0.75, 1.0]);
}

#[test]
fn values_outside_explicit_edges_are_excluded() {
    let result = apply_bin(
        SVBinOptions::new().bin_edges(vec![0.0, 1.0, 2.0]),
        &[-1.0, 0.5, 2.5],
    );

    assert_eq!(result.counts(), &[1.0, 0.0]);
    assert_eq!(result.number_of_observed_values(), 3);
}

#[test]
fn an_explicit_range_replaces_the_derived_domain() {
    let result = apply_bin(
        SVBinOptions::new().range(-1.0, 3.0).bin_count(2),
        &[0.0, 1.0, 2.0],
    );

    assert_eq!(result.edges(), &[-1.0, 1.0, 3.0]);
    assert_eq!(result.counts(), &[1.0, 2.0]);
}

#[test]
fn an_empty_column_is_rejected() {
    let input = SVStatInput::new(SVColumnView::Dense(&[]));

    let error = SVStat::bin(SVBinOptions::new())
        .apply(input)
        .expect_err("an empty column cannot be binned");

    assert_eq!(error, SVError::EmptyInput { operation: "bin" });
}

#[test]
fn a_non_finite_value_is_rejected_with_its_position() {
    let values = [1.0, f64::NAN, 3.0];
    let input = SVStatInput::new(SVColumnView::Dense(&values));

    let error = SVStat::bin(SVBinOptions::new())
        .apply(input)
        .expect_err("a non-finite value is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "bin",
            position: 1,
        }
    );
}

#[test]
fn a_sparse_column_is_binned_from_its_stored_entries() {
    let indices = [1usize, 3];
    let values = [2.0, 4.0];
    let sparse = SVSparseView::new(&indices, &values, 6).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let result = match SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 3.0, 5.0]))
        .apply(input)
        .expect("the sparse column is binnable")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[5.0, 1.0]);
    assert_eq!(result.number_of_observed_values(), 6);
}

#[test]
fn a_sparse_column_counts_its_implicit_zeros_without_expanding() {
    let indices = [1usize];
    let values = [1.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let result = match SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 0.5, 2.0]))
        .apply(input)
        .expect("the sparse column is binnable")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[3.0, 1.0]);
}

#[test]
fn a_streaming_column_is_binned_over_its_current_window() {
    let window = [1.0, 2.0, 3.0];
    let view = SVStreamingView::new(&window, 42, 39);
    let input = SVStatInput::new(SVColumnView::Streaming(view));

    let result = match SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 2.0, 4.0]))
        .apply(input)
        .expect("the window is binnable")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[1.0, 2.0]);
    assert_eq!(result.number_of_observed_values(), 3);
}

#[test]
fn fewer_than_two_edges_are_rejected() {
    let input = SVStatInput::new(SVColumnView::Dense(&[1.0]));
    let error = SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0]))
        .apply(input)
        .expect_err("one edge cannot delimit a bin");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn non_increasing_edges_are_rejected() {
    let input = SVStatInput::new(SVColumnView::Dense(&[1.0]));
    let error = SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 1.0, 1.0]))
        .apply(input)
        .expect_err("edges must increase strictly");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_width_and_a_count_together_are_rejected() {
    let input = SVStatInput::new(SVColumnView::Dense(&[1.0]));
    let error = SVStat::bin(SVBinOptions::new().bin_width(1.0).bin_count(4))
        .apply(input)
        .expect_err("a width and a count are ambiguous");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_non_positive_width_is_rejected() {
    let input = SVStatInput::new(SVColumnView::Dense(&[1.0]));
    let error = SVStat::bin(SVBinOptions::new().bin_width(0.0))
        .apply(input)
        .expect_err("a width must be positive");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn all_equal_observations_still_produce_one_bin() {
    let result = apply_bin(SVBinOptions::new().bin_count(1), &[5.0, 5.0, 5.0]);

    assert_eq!(result.edges(), &[5.0, 6.0]);
    assert_eq!(result.counts(), &[3.0]);
}

#[test]
fn the_default_bin_count_is_thirty() {
    let result = apply_bin(SVBinOptions::new(), &[0.0, 30.0]);

    assert_eq!(result.counts().len(), 30);
}

#[test]
fn weighted_counts_accumulate_the_observation_weights() {
    let values = [0.5, 1.5, 2.5];
    let weights = [1.0, 3.0, 2.0];
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_weights(SVColumnView::Dense(&weights));

    let result = match SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 1.0, 2.0, 3.0]))
        .apply(input)
        .expect("the weighted configuration is valid")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[1.0, 3.0, 2.0]);
}

#[test]
fn all_ones_weights_reproduce_the_unweighted_counts_exactly() {
    let values = [0.1, 0.9, 1.0, 2.5, 2.9];
    let weights = [1.0; 5];
    let options = SVBinOptions::new().bin_edges(vec![0.0, 1.5, 3.0]);

    let unweighted = apply_bin(options.clone(), &values);
    let weighted = match SVStat::bin(options)
        .apply(
            SVStatInput::new(SVColumnView::Dense(&values))
                .with_weights(SVColumnView::Dense(&weights)),
        )
        .expect("the weighted configuration is valid")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(weighted.counts(), unweighted.counts());
}

#[test]
fn weighted_probability_divides_by_the_total_weight() {
    let values = [0.5, 1.5, 1.75];
    let weights = [1.0, 1.0, 2.0];
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_weights(SVColumnView::Dense(&weights));

    let result = match SVStat::bin(
        SVBinOptions::new()
            .bin_edges(vec![0.0, 1.0, 2.0])
            .normalise(SVNormalisation::Probability),
    )
    .apply(input)
    .expect("the weighted configuration is valid")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[0.25, 0.75]);
}

#[test]
fn weighted_density_divides_by_the_total_weight_and_the_bin_width() {
    let values = [0.5, 1.5, 1.75];
    let weights = [1.0, 1.0, 2.0];
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_weights(SVColumnView::Dense(&weights));

    let result = match SVStat::bin(
        SVBinOptions::new()
            .bin_edges(vec![0.0, 1.0, 2.0])
            .normalise(SVNormalisation::Density),
    )
    .apply(input)
    .expect("the weighted configuration is valid")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[0.25, 0.75]);
}

#[test]
fn weighted_sparse_implicit_zeros_use_their_own_weights() {
    let indices = [1usize];
    let values = [1.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let weights = [2.0, 4.0, 6.0, 8.0];
    let input =
        SVStatInput::new(SVColumnView::Sparse(sparse)).with_weights(SVColumnView::Dense(&weights));

    let result = match SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 0.5, 2.0]))
        .apply(input)
        .expect("the weighted sparse column is binnable")
    {
        SVStatResult::Bins(result) => result,
        other => panic!("the bin stat must produce bins, received {other:?}"),
    };

    assert_eq!(result.counts(), &[16.0, 4.0]);
}

#[test]
fn a_negative_weight_is_rejected_with_its_position() {
    let values = [1.0, 3.0];
    let weights = [1.0, -3.0];
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::bin(SVBinOptions::new())
        .apply(input)
        .expect_err("weights cannot be negative");

    assert_eq!(
        error,
        SVError::NegativeWeight {
            operation: "bin",
            position: 1,
        }
    );
}

#[test]
fn a_non_finite_weight_is_rejected_with_its_position() {
    let values = [1.0, 3.0];
    let weights = [1.0, f64::NAN];
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::bin(SVBinOptions::new())
        .apply(input)
        .expect_err("a non-finite weight is not an observation weight");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "bin",
            position: 1,
        }
    );
}

#[test]
fn a_weight_column_of_a_different_length_is_rejected() {
    let values = [1.0, 2.0, 3.0];
    let weights = [1.0, 1.0];
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_weights(SVColumnView::Dense(&weights));

    let error = SVStat::bin(SVBinOptions::new())
        .apply(input)
        .expect_err("columns must agree in length");

    assert_eq!(
        error,
        SVError::MismatchedStatisticLengths {
            operation: "bin",
            first: 3,
            second: 2,
        }
    );
}
