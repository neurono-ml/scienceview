//! Representation parity for the statistical kernels.
//!
//! The parity harness is adopted by the binning kernel as the reference: the
//! harness replays one dataset as a dense slice, as sparse storage over its
//! non-zero entries and through a bounded rolling window, and the kernel must
//! return the same counts for all three.

use crate::sv_data::{
    SVColumnView, SVRepresentationParityHarness, SVSparseView, sv_assert_representation_parity,
};
use crate::sv_stat::{
    SVBinOptions, SVCountOptions, SVCountOrder, SVCountResult, SVStat, SVStatInput, SVStatResult,
};

fn bin_counts(options: SVBinOptions, view: SVColumnView<'_>) -> Vec<f64> {
    match SVStat::bin(options)
        .apply(SVStatInput::new(view))
        .expect("the dataset is binnable")
    {
        SVStatResult::Bins(result) => result.counts().to_vec(),
        other => panic!("the bin stat must produce bins, received {other:?}"),
    }
}

fn count_result(options: SVCountOptions, view: SVColumnView<'_>) -> SVCountResult {
    match SVStat::count(options)
        .apply(SVStatInput::new(view))
        .expect("the dataset is countable")
    {
        SVStatResult::Categories(result) => result,
        other => panic!("the count stat must produce categories, received {other:?}"),
    }
}

const DATASET: [f64; 9] = [1.0, 0.0, 3.0, 0.0, 5.0, 7.0, 0.0, 2.0, 9.0];

#[test]
fn the_binning_kernel_agrees_across_the_three_representations() {
    let options = SVBinOptions::new().bin_edges(vec![0.0, 3.0, 6.0, 10.0]);

    sv_assert_representation_parity(&DATASET, |view| bin_counts(options.clone(), view));
}

#[test]
fn a_windowed_bin_is_compared_against_the_retained_tail() {
    let harness = SVRepresentationParityHarness::new(&DATASET).with_window_capacity(4);
    let options = SVBinOptions::new().bin_count(3);

    harness.assert_kernel_parity(|view| bin_counts(options.clone(), view));
}

#[test]
fn the_counting_kernel_agrees_across_dense_and_sparse_in_data_order() {
    let dense = count_result(SVCountOptions::new(), SVColumnView::Dense(&DATASET));
    let harness = SVRepresentationParityHarness::new(&DATASET);
    let sparse_view = harness.sparse_view().expect("valid sparse storage");
    let sparse = count_result(SVCountOptions::new(), SVColumnView::Sparse(sparse_view));

    assert_eq!(dense.categories(), sparse.categories());
    assert_eq!(dense.counts(), sparse.counts());
}

#[test]
fn every_representation_counts_the_same_categories() {
    let sorted_options = SVCountOptions::new().order(SVCountOrder::Ascending);
    let harness = SVRepresentationParityHarness::new(&DATASET);

    let dense = count_result(sorted_options, harness.dense_view());
    let sparse = count_result(
        sorted_options,
        SVColumnView::Sparse(harness.sparse_view().expect("valid sparse storage")),
    );

    assert_eq!(dense.categories(), sparse.categories());
    assert_eq!(dense.counts(), sparse.counts());
}

#[test]
fn a_sparse_count_never_densifies_its_input() {
    let indices = [2usize, 5];
    let values = [4.0, 6.0];
    let sparse = SVSparseView::new(&indices, &values, 10).expect("valid sparse storage");

    let result = count_result(SVCountOptions::new(), SVColumnView::Sparse(sparse));

    assert_eq!(result.categories(), &[0.0, 4.0, 6.0]);
    assert_eq!(result.counts(), &[8, 1, 1]);
}

const WEIGHTS: [f64; 9] = [2.0, 0.0, 1.0, 0.0, 3.0, 1.0, 0.0, 2.0, 1.0];

fn weighted_bin_counts(value: SVColumnView<'_>, weight: SVColumnView<'_>) -> Vec<f64> {
    match SVStat::bin(SVBinOptions::new().bin_edges(vec![0.0, 3.0, 6.0, 10.0]))
        .apply(SVStatInput::new(value).with_weights(weight))
        .expect("the weighted dataset is binnable")
    {
        SVStatResult::Bins(result) => result.counts().to_vec(),
        other => panic!("the bin stat must produce bins, received {other:?}"),
    }
}

#[test]
fn weighted_binning_agrees_across_the_three_representations() {
    let harness = SVRepresentationParityHarness::new(&DATASET);
    let weight_indices = [0usize, 2, 4, 5, 7, 8];
    let weight_values = [2.0, 1.0, 3.0, 1.0, 2.0, 1.0];
    let sparse_weights =
        SVSparseView::new(&weight_indices, &weight_values, WEIGHTS.len()).expect("valid weights");

    let dense = weighted_bin_counts(harness.dense_view(), SVColumnView::Dense(&WEIGHTS));
    let sparse = weighted_bin_counts(
        SVColumnView::Sparse(harness.sparse_view().expect("valid sparse storage")),
        SVColumnView::Sparse(sparse_weights),
    );
    let streaming = weighted_bin_counts(
        SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&DATASET, 9, 0)),
        SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&WEIGHTS, 9, 0)),
    );

    assert_eq!(dense, sparse);
    assert_eq!(dense, streaming);
}

fn weighted_density(value: SVColumnView<'_>, weight: SVColumnView<'_>) -> Vec<f64> {
    match SVStat::kernel_density(
        crate::sv_stat::SVKernelDensityOptions::new()
            .bandwidth(1.0)
            .grid_count(25)
            .cut(2.0),
    )
    .apply(SVStatInput::new(value).with_weights(weight))
    .expect("the weighted dataset has a density")
    {
        SVStatResult::Density(result) => result.density().to_vec(),
        other => panic!("the density stat must produce a density, received {other:?}"),
    }
}

#[test]
fn weighted_density_agrees_across_dense_and_sparse_representations() {
    let harness = SVRepresentationParityHarness::new(&DATASET);
    let weight_indices = [0usize, 2, 4, 5, 7, 8];
    let weight_values = [2.0, 1.0, 3.0, 1.0, 2.0, 1.0];
    let sparse_weights =
        SVSparseView::new(&weight_indices, &weight_values, WEIGHTS.len()).expect("valid weights");

    let dense = weighted_density(harness.dense_view(), SVColumnView::Dense(&WEIGHTS));
    let sparse = weighted_density(
        SVColumnView::Sparse(harness.sparse_view().expect("valid sparse storage")),
        SVColumnView::Sparse(sparse_weights),
    );
    let streaming = weighted_density(
        SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&DATASET, 9, 0)),
        SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&WEIGHTS, 9, 0)),
    );

    dense
        .iter()
        .zip(sparse)
        .for_each(|(dense, sparse)| assert!((dense - sparse).abs() < 1e-12));
    dense
        .iter()
        .zip(streaming)
        .for_each(|(dense, streaming)| assert!((dense - streaming).abs() < 1e-12));
}
