//! Dense, sparse and streaming inputs produce the same curve.

use super::super::SVECdfPlot;
use super::test_support::{SVLiveSource, SVTestColumn, SVTestSource};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVRepresentation, SVSparseView};

const VALUES: [f64; 4] = [3.0, 1.0, 2.0, 3.0];

fn build<'a>(source: &'a SVTestSource<'a>) -> SVECdfPlot<'a> {
    SVECdfPlot::builder()
        .data_source(source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn dense_sparse_and_streaming_agree() {
    let indices = [0, 1, 2, 3];
    let sparse =
        SVSparseView::new(&indices, &VALUES, VALUES.len()).expect("the sparse view is valid");
    let dense_table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &VALUES);
    let sparse_table =
        SVTestSource::new().with_column("value", SVColumnRole::Y, SVTestColumn::Sparse(sparse));
    let streaming_table = SVTestSource::new().with_column(
        "value",
        SVColumnRole::Y,
        SVTestColumn::Streaming(&VALUES, 7),
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
fn a_sparse_column_is_never_expanded() {
    let indices = [0, 10, 999_999];
    let stored = [2.0, 4.0, 6.0];
    let length = 1_000_000;
    let sparse = SVSparseView::new(&indices, &stored, length).expect("the sparse view is valid");
    let table =
        SVTestSource::new().with_column("value", SVColumnRole::Y, SVTestColumn::Sparse(sparse));
    let composition = build(&table).compose().expect("the sparse column composes");
    let series = &composition.series()[0];

    assert_eq!(composition.point_count(), length);
    assert_eq!(
        series.sample_horizontal(),
        &[0.0, 2.0, 4.0, 6.0],
        "only the implicit zero and the three stored values are breakpoints"
    );
    assert_eq!(series.horizontal().len(), 8);
    assert_eq!(series.vertical().last(), Some(&1.0));
}

#[test]
fn large_dense_and_streaming_inputs_produce_the_same_vertices() {
    let values: Vec<f64> = (0..10_000)
        .map(|index| (index as f64 / 13.0).sin())
        .collect();
    let dense_table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let streaming_table = SVTestSource::new().with_column(
        "value",
        SVColumnRole::Y,
        SVTestColumn::Streaming(&values, 10_000),
    );

    let dense = build(&dense_table).compose().expect("dense composes");
    let streaming = build(&streaming_table)
        .compose()
        .expect("streaming composes");

    assert_eq!(dense.point_count(), 10_000);
    assert_eq!(dense.series(), streaming.series());
}

#[test]
fn a_streaming_window_extends_with_the_append() {
    let values = [1.0, 2.0, 3.0];
    let series = [1.0, 1.0, 1.0];
    let table = SVLiveSource::new(&values, &series);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .series("scenario")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let before = plot.render().expect("the window composes");
    assert_eq!(before.point_count(), 3);
    table.append(4.0, 1.0);
    let after = plot.render().expect("the advanced window composes");
    assert_eq!(after.point_count(), 4);
    assert_eq!(
        plot.update(1).expect("the update composes"),
        after,
        "update and render agree on the same window"
    );
}
