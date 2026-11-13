//! Representation parity, large data and the streaming window.

use super::super::SVStepPlot;
use super::test_sources::{SVTestColumn, SVTestSource};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVRepresentation, SVSparseView};

fn source<'a>(horizontal: SVTestColumn<'a>, vertical: SVTestColumn<'a>) -> SVTestSource<'a> {
    SVTestSource::new()
        .with_column("x", SVColumnRole::X, horizontal)
        .with_column("y", SVColumnRole::Y, vertical)
}

fn build<'a>(source: &'a SVTestSource<'a>) -> SVStepPlot<'a> {
    SVStepPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

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
fn a_sparse_column_keeps_its_stored_entries() {
    let indices = [0, 99_999];
    let values = [0.0, 1.0];
    let vertical_values = [1.0, 2.0];
    let sparse = SVSparseView::new(&indices, &values, 100_000).expect("the view is valid");

    assert_eq!(sparse.number_of_stored_values(), 2);
    assert_eq!(sparse.length(), 100_000);

    let table = source(
        SVTestColumn::Sparse(sparse),
        SVTestColumn::Sparse(
            SVSparseView::new(&indices, &vertical_values, 100_000).expect("the view is valid"),
        ),
    );
    let composition = build(&table).compose().expect("the sparse plot composes");
    assert_eq!(composition.representation(), SVRepresentation::Sparse);
    assert_eq!(composition.point_count(), 2);
    assert_eq!(composition.series()[0].sample_horizontal().len(), 2);
    assert_eq!(composition.series()[0].horizontal().len(), 3);
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

#[test]
fn a_streaming_window_extends_to_the_new_sample() {
    let first_horizontal = [1.0, 2.0, 3.0];
    let first_vertical = [1.0, 2.0, 3.0];
    let second_horizontal = [1.0, 2.0, 3.0, 4.0];
    let second_vertical = [1.0, 2.0, 3.0, 4.0];

    let first_table = source(
        SVTestColumn::Streaming(&first_horizontal, 3),
        SVTestColumn::Streaming(&first_vertical, 3),
    );
    let second_table = source(
        SVTestColumn::Streaming(&second_horizontal, 4),
        SVTestColumn::Streaming(&second_vertical, 4),
    );

    let first = build(&first_table).compose().expect("the window composes");
    let second = build(&second_table).compose().expect("the window composes");

    assert_eq!(first.point_count(), 3);
    assert_eq!(second.point_count(), 4);
    assert_eq!(second.series()[0].horizontal().last(), Some(&4.0));
}
