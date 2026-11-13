//! Representation parity, sparse storage and streaming windows.

use super::SVContourPlot;
use super::test_support::{
    SVTestSource, sv_builder, sv_compose_default, sv_points_builder, sv_points_source,
    sv_primitives, sv_sparse_points_source, sv_streaming_points_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

fn sv_points_field_values(source: &dyn crate::sv_data::SVDataSource) -> Vec<f64> {
    sv_points_builder(source)
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes")
        .field()
        .values()
        .to_vec()
}

#[test]
fn dense_and_sparse_fields_agree() {
    let dense = sv_points_field_values(&sv_points_source());
    let sparse = sv_points_field_values(&sv_sparse_points_source());

    assert_eq!(dense, sparse, "the sparse field matches the dense one");
}

#[test]
fn dense_and_streaming_contours_produce_the_same_primitives() {
    let dense = sv_primitives(
        &sv_points_builder(&sv_points_source())
            .build()
            .expect("the chart builds")
            .compose()
            .expect("composes"),
    );
    let streaming = sv_primitives(
        &sv_points_builder(&sv_streaming_points_source())
            .backend(SVBackendKind::Native)
            .build()
            .expect("the streaming chart builds")
            .compose()
            .expect("composes"),
    );

    assert_eq!(dense, streaming);
}

#[test]
fn a_structurally_sparse_matrix_is_never_densified() {
    // Only the central sample is stored; every other logical position is an
    // implicit zero. The field is equal to the dense zero-filled baseline
    // without ever materialising the twenty-five-value column.
    const INDICES: [usize; 1] = [12];
    const STORED: [f64; 1] = [4.0];
    const ZERO_FIELD: [f64; 25] = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    let sparse =
        crate::sv_data::SVSparseView::new(&INDICES, &STORED, 25).expect("the view is well formed");
    let sparse_source = SVTestSource {
        value: SVColumnView::Sparse(sparse),
        x: None,
        y: None,
    };
    let dense_source = SVTestSource::matrix(&ZERO_FIELD);

    assert_eq!(
        sv_compose_default(&sparse_source).field().values(),
        sv_compose_default(&dense_source).field().values()
    );
    assert_eq!(
        sv_primitives(&sv_compose_default(&sparse_source)),
        sv_primitives(&sv_compose_default(&dense_source))
    );
}

#[test]
fn a_streaming_chart_reads_its_current_window_without_blocking() {
    let short = SVTestSource {
        value: SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(
            &[1.0, 2.0, 3.0, 4.0],
            4,
            0,
        )),
        x: None,
        y: None,
    };
    let plot = SVContourPlot::builder()
        .data_source(&short)
        .value("value")
        .matrix(2, 2)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the streaming chart builds");

    assert_eq!(plot.value_count(), 4);
    assert_eq!(plot.representation().to_string(), "Streaming");
    assert_eq!(
        plot.compose()
            .expect("composes")
            .field()
            .number_of_samples(),
        4
    );
}

#[test]
fn the_representation_is_the_dominant_one_of_the_bound_columns() {
    let sparse = sv_points_builder(&sv_sparse_points_source())
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    assert_eq!(sparse.representation().to_string(), "Sparse");
    let dense = sv_points_builder(&sv_points_source())
        .build()
        .expect("the chart builds")
        .compose()
        .expect("composes");
    assert_eq!(dense.representation().to_string(), "Dense");
}

#[test]
fn an_export_only_backend_rejects_a_streaming_column_at_build() {
    let source = sv_streaming_points_source();
    let result = sv_points_builder(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(matches!(
        result,
        Err(SVError::UnsupportedBackendCapability { .. })
    ));
}

#[test]
fn the_matrix_builder_composes_the_same_field_from_a_sparse_value_column() {
    let source = super::test_support::sv_matrix_source();
    let plot = sv_builder(&source).build().expect("the chart builds");
    assert_eq!(
        plot.compose().expect("composes").field().number_of_rows(),
        5
    );
}
