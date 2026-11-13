//! Parity of dense, sparse and streaming input (FDR-105 acceptance).

use std::sync::{Mutex, PoisonError};

use scienceview_core::sv_data::SVRepresentation;

use super::band_plot_test_support::{
    SVCountingSource, assert_close, dense_source, points, render, sparse_source, streaming_source,
};

#[test]
fn dense_sparse_and_streaming_agree() {
    let dense = dense_source();
    let sparse = sparse_source();
    let streaming = streaming_source();
    let dense_output = render(&dense);
    let sparse_output = render(&sparse);
    let streaming_output = render(&streaming);

    assert_eq!(dense_output.representation(), SVRepresentation::Dense);
    assert_eq!(sparse_output.representation(), SVRepresentation::Sparse);
    assert_eq!(
        streaming_output.representation(),
        SVRepresentation::Streaming
    );
    assert!(!dense_output.is_approximate());
    assert!(streaming_output.is_approximate());
    assert_close(&points(&dense_output), &points(&sparse_output));
    assert_close(&points(&dense_output), &points(&streaming_output));
}

#[test]
fn every_column_is_resolved_once_per_render() {
    let sparse = sparse_source();
    let counting = SVCountingSource {
        inner: &sparse,
        reads: Mutex::new(Vec::new()),
    };
    let _ = render(&counting);
    let reads = counting
        .reads
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    assert_eq!(reads, vec!["x", "lower", "upper"]);
}
