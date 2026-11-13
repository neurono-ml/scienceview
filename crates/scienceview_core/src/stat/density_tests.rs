//! Companion tests for the kernel-density kernel.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_stat::{
    SVKernelDensityOptions, SVKernelDensityResult, SVKernelShape, SVStat, SVStatInput, SVStatResult,
};

fn apply_density(options: SVKernelDensityOptions, input: SVStatInput<'_>) -> SVKernelDensityResult {
    match SVStat::kernel_density(options)
        .apply(input)
        .expect("the density configuration is valid")
    {
        SVStatResult::Density(result) => result,
        other => panic!("the density stat must produce a density, received {other:?}"),
    }
}

fn dense(values: &[f64]) -> SVStatInput<'_> {
    SVStatInput::new(SVColumnView::Dense(values))
}

fn trapezoid_integral(result: &SVKernelDensityResult) -> f64 {
    result
        .grid()
        .windows(2)
        .zip(result.density().windows(2))
        .map(|(grid, density)| {
            let width = grid[1] - grid[0];
            width * (density[0] + density[1]) / 2.0
        })
        .sum()
}

#[test]
fn the_density_grid_spans_the_data_plus_the_requested_cut() {
    let result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(7)
            .cut(2.0),
        dense(&[0.0, 4.0]),
    );

    assert_eq!(result.grid().len(), 7);
    assert_eq!(result.grid()[0], -2.0);
    assert_eq!(result.grid()[6], 6.0);
    assert_eq!(result.bandwidth(), 1.0);
}

#[test]
fn a_gaussian_density_at_the_centre_of_one_observation_is_the_hand_computed_value() {
    let result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(7)
            .cut(3.0),
        dense(&[0.0]),
    );

    let expected = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
    assert!((result.density()[3] - expected).abs() < 1e-12);
}

#[test]
fn an_epanechnikov_kernel_has_its_own_normalisation() {
    let result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(7)
            .cut(3.0)
            .kernel(SVKernelShape::Epanechnikov),
        dense(&[0.0]),
    );

    assert!((result.density()[3] - 0.75).abs() < 1e-12);
}

#[test]
fn a_uniform_kernel_has_its_own_normalisation() {
    let result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(7)
            .cut(3.0)
            .kernel(SVKernelShape::Uniform),
        dense(&[0.0]),
    );

    assert!((result.density()[3] - 0.5).abs() < 1e-12);
}

#[test]
fn the_derived_bandwidth_follows_the_rule_of_thumb() {
    let result = apply_density(SVKernelDensityOptions::new(), dense(&[1.0, 2.0, 3.0, 4.0]));

    assert!(result.bandwidth() > 0.0);
    assert!(result.bandwidth().is_finite());
}

#[test]
fn a_density_integrates_to_approximately_one() {
    let result = apply_density(
        SVKernelDensityOptions::new().grid_count(512).cut(4.0),
        dense(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]),
    );

    assert!((trapezoid_integral(&result) - 1.0).abs() < 1e-3);
}

#[test]
fn a_sparse_density_includes_its_implicit_zeros() {
    let indices = [2usize];
    let values = [4.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let sparse_result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(7)
            .cut(1.0),
        SVStatInput::new(SVColumnView::Sparse(sparse)),
    );
    let dense_result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(7)
            .cut(1.0),
        dense(&[0.0, 0.0, 0.0, 4.0]),
    );

    assert_eq!(sparse_result.grid(), dense_result.grid());
    for (sparse_value, dense_value) in sparse_result.density().iter().zip(dense_result.density()) {
        assert!((sparse_value - dense_value).abs() < 1e-12);
    }
}

#[test]
fn a_streaming_density_reports_that_it_is_windowed() {
    let window = [1.0, 2.0, 3.0];
    let view = SVStreamingView::new(&window, 20, 17);
    let result = apply_density(
        SVKernelDensityOptions::new(),
        SVStatInput::new(SVColumnView::Streaming(view)),
    );

    assert!(result.is_windowed());
}

#[test]
fn a_dense_density_is_not_windowed() {
    let result = apply_density(SVKernelDensityOptions::new(), dense(&[1.0, 2.0]));

    assert!(!result.is_windowed());
}

#[test]
fn all_equal_observations_do_not_produce_a_nan_density() {
    let result = apply_density(
        SVKernelDensityOptions::new().grid_count(16),
        dense(&[5.0, 5.0, 5.0]),
    );

    assert!(result.bandwidth() > 0.0);
    assert!(result.density().iter().all(|value| value.is_finite()));
}

#[test]
fn an_invalid_grid_count_is_rejected() {
    let error = SVStat::kernel_density(SVKernelDensityOptions::new().grid_count(1))
        .apply(dense(&[1.0]))
        .expect_err("a grid needs at least two points");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_invalid_bandwidth_is_rejected() {
    let error = SVStat::kernel_density(SVKernelDensityOptions::new().bandwidth(0.0))
        .apply(dense(&[1.0]))
        .expect_err("a bandwidth must be positive");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_invalid_cut_is_rejected() {
    let error = SVStat::kernel_density(SVKernelDensityOptions::new().cut(-1.0))
        .apply(dense(&[1.0]))
        .expect_err("a cut cannot be negative");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_empty_column_is_rejected() {
    let error = SVStat::kernel_density(SVKernelDensityOptions::new())
        .apply(dense(&[]))
        .expect_err("there is no density of nothing");

    assert_eq!(
        error,
        SVError::EmptyInput {
            operation: "kernel_density",
        }
    );
}

#[test]
fn a_non_finite_value_is_rejected() {
    let error = SVStat::kernel_density(SVKernelDensityOptions::new())
        .apply(dense(&[1.0, f64::INFINITY]))
        .expect_err("infinity is not an observation");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "kernel_density",
            position: 1,
        }
    );
}

#[test]
fn all_ones_weights_reproduce_the_unweighted_density_exactly() {
    let values = [0.0, 1.5, 4.0];
    let weights = [1.0; 3];
    let options = SVKernelDensityOptions::new()
        .bandwidth(1.0)
        .grid_count(17)
        .cut(2.0);

    let unweighted = apply_density(options.clone(), dense(&values));
    let weighted = apply_density(
        options,
        dense(&values).with_weights(SVColumnView::Dense(&weights)),
    );

    assert_eq!(weighted.density(), unweighted.density());
}

#[test]
fn a_weighted_density_integrates_to_one() {
    let values = [0.0, 2.0, 2.5];
    let weights = [1.0, 3.0, 2.0];
    let result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(0.75)
            .grid_count(2001)
            .cut(5.0),
        dense(&values).with_weights(SVColumnView::Dense(&weights)),
    );

    assert!((trapezoid_integral(&result) - 1.0).abs() < 1e-6);
}

#[test]
fn weighted_sparse_implicit_zeros_use_their_own_weights() {
    let indices = [2usize];
    let values = [4.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let weights = [1.0, 2.0, 3.0, 4.0];

    let sparse_result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(33)
            .cut(2.0),
        SVStatInput::new(SVColumnView::Sparse(sparse)).with_weights(SVColumnView::Dense(&weights)),
    );
    let dense_values = [0.0, 0.0, 4.0, 0.0];
    let dense_result = apply_density(
        SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(33)
            .cut(2.0),
        SVStatInput::new(SVColumnView::Dense(&dense_values))
            .with_weights(SVColumnView::Dense(&weights)),
    );

    sparse_result
        .density()
        .iter()
        .zip(dense_result.density())
        .for_each(|(sparse, dense)| assert!((sparse - dense).abs() < 1e-12));
}

#[test]
fn a_negative_weight_is_rejected_with_its_position() {
    let weights = [1.0, -1.0];
    let error = SVStat::kernel_density(SVKernelDensityOptions::new())
        .apply(dense(&[0.0, 1.0]).with_weights(SVColumnView::Dense(&weights)))
        .expect_err("weights cannot be negative");

    assert_eq!(
        error,
        SVError::NegativeWeight {
            operation: "kernel_density",
            position: 1,
        }
    );
}

#[test]
fn a_non_finite_weight_is_rejected_with_its_position() {
    let weights = [1.0, f64::INFINITY];
    let error = SVStat::kernel_density(SVKernelDensityOptions::new())
        .apply(dense(&[0.0, 1.0]).with_weights(SVColumnView::Dense(&weights)))
        .expect_err("a non-finite weight is invalid");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "kernel_density",
            position: 1,
        }
    );
}

#[test]
fn a_zero_weight_total_is_rejected() {
    let weights = [0.0, 0.0];
    let error = SVStat::kernel_density(SVKernelDensityOptions::new())
        .apply(dense(&[0.0, 1.0]).with_weights(SVColumnView::Dense(&weights)))
        .expect_err("a density needs positive total weight");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_weight_column_of_a_different_length_is_rejected() {
    let weights = [1.0];
    let error = SVStat::kernel_density(SVKernelDensityOptions::new())
        .apply(dense(&[0.0, 1.0]).with_weights(SVColumnView::Dense(&weights)))
        .expect_err("columns must agree in length");

    assert_eq!(
        error,
        SVError::MismatchedStatisticLengths {
            operation: "kernel_density",
            first: 2,
            second: 1,
        }
    );
}
