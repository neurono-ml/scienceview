//! The data boundary: sparse is read in place, streaming is rejected up front.

use super::SVClusteredHeatmapPlot;
use super::test_support::{
    sv_build_default, sv_dense_source, sv_sparse_source, sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;

#[test]
fn a_sparse_value_column_is_read_without_being_densified() {
    let source = sv_sparse_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot.compose().expect("the sparse source composes");

    assert_eq!(
        scene.number_of_marks(),
        3,
        "only the stored cells are drawn"
    );
    let values: Vec<f64> = scene
        .cells()
        .iter()
        .filter_map(|cell| cell.value())
        .collect();
    assert_eq!(values, [1.0, 5.0, 9.0]);
}

#[test]
fn a_dense_value_column_reports_its_own_representation() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Dense);
    assert_eq!(
        plot.compose()
            .expect("the dense source composes")
            .number_of_marks(),
        9
    );
}

#[test]
fn a_streaming_source_is_rejected_before_composition() {
    let source = sv_streaming_source();
    let error = SVClusteredHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("value")
        .row_order(&super::test_support::ROW_ORDER)
        .column_order(&super::test_support::COLUMN_ORDER)
        .dendrogram(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a stream is a static-only exception");

    match error {
        SVError::UnsupportedRepresentation {
            operation,
            representation,
            alternative,
        } => {
            assert_eq!(operation, "SVClusteredHeatmapPlot");
            assert_eq!(representation, SVRepresentation::Streaming);
            assert!(alternative.contains("snapshot"));
        }
        other => panic!("expected the documented static-only rejection, received {other:?}"),
    }
}

#[test]
fn a_missing_value_column_is_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVClusteredHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .row_order(&super::test_support::ROW_ORDER)
        .column_order(&super::test_support::COLUMN_ORDER)
        .dendrogram(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the value column is required");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_missing_key_column_is_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVClusteredHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("absent")
        .value("value")
        .row_order(&super::test_support::ROW_ORDER)
        .column_order(&super::test_support::COLUMN_ORDER)
        .dendrogram(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the key column must exist");
    assert!(matches!(error, SVError::ColumnNotFound { .. }));
}
