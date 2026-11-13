//! Dense, sparse and streaming input produce the same silhouette (FDR-117).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVSparseView, SVStreamingView};

use super::SVViolinPlotBuilder;
use super::violin_plot_test_support::{
    CATEGORY, SVTestSource, VALUES, assert_close, dense_source, points, render, sparse_source,
    streaming_source,
};

#[test]
fn dense_sparse_and_streaming_agree() {
    let dense = render(&dense_source());
    let sparse = render(&sparse_source());
    let streaming = render(&streaming_source());

    assert_eq!(dense.summaries().len(), 2);
    assert_close(&points(&dense), &points(&sparse));
    assert_close(&points(&dense), &points(&streaming));
    assert_eq!(dense.number_of_points(), sparse.number_of_points());
    assert_eq!(dense.number_of_points(), streaming.number_of_points());
    assert!(!dense.is_approximate());
    assert!(!sparse.is_approximate());
    assert!(streaming.is_approximate());
}

#[test]
fn a_sparse_column_is_never_densified() {
    let indices = [0usize, 2, 4, 6, 8];
    let values = [1.0, 3.0, 5.0, 7.0, 9.0];
    let categories = [0.0; 5];
    let logical_length = 1_000_000;
    let value_view =
        SVSparseView::new(&indices, &values, logical_length).expect("valid sparse storage");
    let category_view =
        SVSparseView::new(&indices, &categories, logical_length).expect("valid sparse storage");
    let source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Sparse(value_view)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(category_view),
        ),
    ]);

    let output = render(&source);
    assert_eq!(
        output.number_of_points(),
        indices.len(),
        "only the stored entries become observations"
    );
}

#[test]
fn a_streaming_window_renders_without_expanding_the_stream() {
    let value_view = SVStreamingView::new(&VALUES, 12, 0);
    let category_view = SVStreamingView::new(&CATEGORY, 12, 0);
    let source = SVTestSource::new(vec![
        (
            "value",
            SVColumnRole::Y,
            SVColumnView::Streaming(value_view),
        ),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(category_view),
        ),
    ]);

    let output = SVViolinPlotBuilder::new()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert_eq!(output.number_of_points(), VALUES.len());
    assert!(output.is_approximate());
}
