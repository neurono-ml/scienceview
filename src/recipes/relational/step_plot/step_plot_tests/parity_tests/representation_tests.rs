//! Dense, sparse and streaming inputs produce the same steps.

use super::super::test_sources::SVTestColumn;
use super::helpers::{build, source};
use scienceview_core::sv_data::{SVRepresentation, SVSparseView};

#[test]
fn dense_sparse_and_streaming_agree() {
    let horizontal = [1.0, 2.0, 3.0, 4.0];
    let vertical = [10.0, 20.0, 30.0, 40.0];
    let indices = [0, 1, 2, 3];

    let dense_table = source(
        SVTestColumn::Dense(&horizontal),
        SVTestColumn::Dense(&vertical),
    );
    let sparse_vertical =
        SVSparseView::new(&indices, &vertical, vertical.len()).expect("the sparse view is valid");
    let sparse_table = source(
        SVTestColumn::Dense(&horizontal),
        SVTestColumn::Sparse(sparse_vertical),
    );
    let streaming_table = source(
        SVTestColumn::Streaming(&horizontal, 9),
        SVTestColumn::Streaming(&vertical, 9),
    );

    let dense = build(&dense_table).compose().expect("dense composes");
    let sparse = build(&sparse_table).compose().expect("sparse composes");
    let streaming = build(&streaming_table)
        .compose()
        .expect("streaming composes");

    assert_eq!(dense.representation(), SVRepresentation::Dense);
    assert_eq!(sparse.representation(), SVRepresentation::Sparse);
    assert_eq!(streaming.representation(), SVRepresentation::Streaming);
    assert_eq!(dense.series(), sparse.series());
    assert_eq!(dense.series(), streaming.series());
    assert_eq!(dense.point_count(), sparse.point_count());
    assert_eq!(dense.point_count(), streaming.point_count());
}

#[test]
fn large_dense_and_streaming_inputs_produce_the_same_vertices() {
    let horizontal: Vec<f64> = (0..10_000).map(|index| index as f64).collect();
    let vertical: Vec<f64> = (0..10_000).map(|index| (index as f64).sin()).collect();

    let dense_table = source(
        SVTestColumn::Dense(&horizontal),
        SVTestColumn::Dense(&vertical),
    );
    let streaming_table = source(
        SVTestColumn::Streaming(&horizontal, 10_000),
        SVTestColumn::Streaming(&vertical, 10_000),
    );

    let dense = build(&dense_table).compose().expect("dense composes");
    let streaming = build(&streaming_table)
        .compose()
        .expect("streaming composes");

    assert_eq!(dense.point_count(), 10_000);
    assert_eq!(dense.series()[0].horizontal().len(), 19_999);
    assert_eq!(dense.series(), streaming.series());
}
