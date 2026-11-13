//! Parity of dense, sparse and streaming input (FDR-105 acceptance).

use std::sync::{Mutex, PoisonError};

use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVRepresentation, SVSparseView};

use super::band_plot_test_support::{
    SVCountingSource, SVTestSource, assert_close, dense_source, points, render, sparse_source,
    streaming_source,
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

#[test]
fn a_huge_sparse_column_with_few_stored_entries_draws_only_those_entries() {
    let indices = [0usize, 4_999_999, 9_999_999];
    let values = [0.0, 4.0, 8.0];
    let lower_values = [0.5, 3.5, 7.5];
    let upper_values = [1.5, 4.5, 8.5];
    let logical_length = 10_000_000;
    let x = SVSparseView::new(&indices, &values, logical_length).expect("valid sparse view");
    let lower =
        SVSparseView::new(&indices, &lower_values, logical_length).expect("valid sparse view");
    let upper =
        SVSparseView::new(&indices, &upper_values, logical_length).expect("valid sparse view");
    let source = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Sparse(x)),
        ("lower", SVColumnRole::Y, SVColumnView::Sparse(lower)),
        ("upper", SVColumnRole::Y, SVColumnView::Sparse(upper)),
    ]);

    let output = render(&source);

    assert_eq!(output.representation(), SVRepresentation::Sparse);
    assert_eq!(output.number_of_points(), indices.len());
    // The ribbon traces both boundaries, so three samples produce six path
    // vertices — never ten million implicit zeros.
    assert_eq!(points(&output).len(), indices.len() * 2);
}

#[test]
fn implicit_sparse_zeros_match_the_dense_equivalent() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let lower = [0.5, 0.0, 1.2, 0.0];
    let upper = [1.5, 2.0, 3.1, 4.0];
    let dense = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Dense(&x)),
        ("lower", SVColumnRole::Y, SVColumnView::Dense(&lower)),
        ("upper", SVColumnRole::Y, SVColumnView::Dense(&upper)),
    ]);
    let stored_indices = [0usize, 2];
    let stored_values = [0.5, 1.2];
    let sparse_lower =
        SVSparseView::new(&stored_indices, &stored_values, x.len()).expect("valid sparse view");
    let sparse = SVTestSource::new(vec![
        ("x", SVColumnRole::X, SVColumnView::Dense(&x)),
        ("lower", SVColumnRole::Y, SVColumnView::Sparse(sparse_lower)),
        ("upper", SVColumnRole::Y, SVColumnView::Dense(&upper)),
    ]);

    let dense_output = render(&dense);
    let sparse_output = render(&sparse);

    assert_eq!(sparse_output.number_of_points(), x.len());
    assert_close(&points(&dense_output), &points(&sparse_output));
}
