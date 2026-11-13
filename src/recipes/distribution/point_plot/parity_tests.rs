//! Dense, sparse and streaming input produce the same points (FDR-123).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVSparseView, SVStreamingView};

use super::SVPointPlotBuilder;
use super::test_support::{
    SVTestSource, assert_close, dense_source, dot_centres, render, render_series, sparse_source,
    streaming_source,
};

#[test]
fn dense_sparse_and_streaming_agree() {
    let dense = render_series(&dense_source());
    let sparse = render_series(&sparse_source());
    let streaming = render_series(&streaming_source());

    assert_eq!(dense.estimates().len(), 6);
    assert_eq!(sparse.estimates().len(), 6);
    assert_eq!(streaming.estimates().len(), 6);
    assert_close(&dot_centres(&dense), &dot_centres(&sparse));
    assert_close(&dot_centres(&dense), &dot_centres(&streaming));
    assert_eq!(dense.number_of_points(), sparse.number_of_points());
    assert_eq!(dense.number_of_points(), streaming.number_of_points());
    assert!(!dense.is_approximate());
    assert!(!sparse.is_approximate());
    assert!(streaming.is_approximate());
}

#[test]
fn the_point_estimates_agree_across_representations() {
    let dense = render_series(&dense_source());
    let sparse = render_series(&sparse_source());
    let streaming = render_series(&streaming_source());

    for ((dense, sparse), streaming) in dense
        .estimates()
        .iter()
        .zip(sparse.estimates())
        .zip(streaming.estimates())
    {
        assert!((dense.value() - sparse.value()).abs() < 1e-12);
        assert!((dense.value() - streaming.value()).abs() < 1e-12);
    }
}

#[test]
fn a_sparse_column_is_never_densified() {
    let indices = [0usize, 2, 4, 6, 8];
    let values = [1.0, 3.0, 5.0, 7.0, 9.0];
    let categories = [0.0, 0.0, 0.0, 0.0, 0.0];
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
    assert_eq!(output.estimates().len(), 1);
    assert_eq!(
        output.estimates()[0].number_of_observations(),
        indices.len()
    );
    assert!((output.estimates()[0].value() - 5.0).abs() < 1e-12);
}

#[test]
fn a_streaming_window_renders_without_expanding_the_stream() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let categories = [0.0, 0.0, 1.0, 1.0];
    let value_view = SVStreamingView::new(&values, 4, 0);
    let category_view = SVStreamingView::new(&categories, 4, 0);
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

    let output = SVPointPlotBuilder::new()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert_eq!(output.number_of_points(), values.len());
    assert_eq!(output.estimates().len(), 2);
    assert!(output.is_approximate());
}

#[test]
fn mixed_representations_agree_after_validating_lengths() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let categories = [0.0, 0.0, 1.0, 1.0];
    let series = [0.0, 1.0, 0.0, 1.0];
    let indices = [0usize, 1, 2, 3];
    let category_view =
        SVSparseView::new(&indices, &categories, categories.len()).expect("valid sparse storage");
    let source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&values)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(category_view),
        ),
        (
            "series",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&series, 4, 0)),
        ),
    ]);

    let mixed = SVPointPlotBuilder::new()
        .data_source(&source)
        .value("value")
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let dense_source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&values)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&categories),
        ),
        (
            "series",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&series),
        ),
    ]);
    let dense = render_series(&dense_source);
    assert_eq!(mixed.estimates().len(), 4);
    assert_eq!(mixed.number_of_points(), 4);
    assert!(mixed.is_approximate());
    assert_close(&dot_centres(&dense), &dot_centres(&mixed));
}
