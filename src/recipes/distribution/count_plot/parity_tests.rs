//! Representation parity, the data boundary and the streaming path.

use super::SVCountPlot;
use super::test_support::{
    SVTestSource, sv_build_default, sv_build_series, sv_dense_source, sv_scene_to_svg,
    sv_series_source, sv_sparse_series_source, sv_sparse_source, sv_streaming_series_source,
    sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnView, SVRepresentation, SVSparseView};
use crate::sv_error::SVError;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_scene() {
    let dense = sv_scene_to_svg(
        &sv_build_default(&sv_dense_source())
            .compose()
            .expect("dense renders"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_default(&sv_sparse_source())
            .compose()
            .expect("sparse renders"),
    );
    let streaming = sv_scene_to_svg(
        &sv_build_default(&sv_streaming_source())
            .compose()
            .expect("streaming renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn dense_sparse_and_streaming_series_agree() {
    let dense = sv_scene_to_svg(
        &sv_build_series(&sv_series_source())
            .compose()
            .expect("the dense series renders"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_series(&sv_sparse_series_source())
            .compose()
            .expect("the sparse series renders"),
    );
    let streaming = sv_scene_to_svg(
        &sv_build_series(&sv_streaming_series_source())
            .compose()
            .expect("the streaming series renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse series agree");
    assert_eq!(dense, streaming, "dense and streaming series agree");
}

#[test]
fn a_sparse_input_is_never_densified() {
    let source = sv_sparse_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse view counts without being expanded");
    assert_eq!(scene.counts(), &[1.0, 2.0, 3.0]);
}

#[test]
fn a_sparse_input_with_implicit_zeros_counts_them_as_category_zero() {
    const DENSE: [f64; 5] = [0.0, 0.0, 2.0, 0.0, 2.0];
    const INDICES: [usize; 2] = [2, 4];
    const STORED: [f64; 2] = [2.0, 2.0];
    let sparse = SVSparseView::new(&INDICES, &STORED, 5).expect("the sparse view is valid");
    let sparse_source = SVTestSource {
        category: SVColumnView::Sparse(sparse),
        series: None,
    };
    let dense_source = SVTestSource::dense(&DENSE);

    let dense = sv_build_default(&dense_source)
        .compose()
        .expect("dense renders");
    let sparse = sv_build_default(&sparse_source)
        .compose()
        .expect("sparse renders");

    assert_eq!(dense.counts(), &[3.0, 2.0]);
    assert_eq!(sparse.counts(), dense.counts());
    assert_eq!(
        sv_scene_to_svg(&dense),
        sv_scene_to_svg(&sparse),
        "an implicit zero is a category-zero observation, not a gap"
    );
}

#[test]
fn a_sparse_non_finite_category_is_omitted_like_its_dense_equivalent() {
    const DENSE: [f64; 4] = [0.0, f64::NAN, 2.0, 0.0];
    const INDICES: [usize; 2] = [1, 2];
    const STORED: [f64; 2] = [f64::NAN, 2.0];
    let sparse = SVSparseView::new(&INDICES, &STORED, 4).expect("the sparse view is valid");
    let sparse_source = SVTestSource {
        category: SVColumnView::Sparse(sparse),
        series: None,
    };
    let dense_source = SVTestSource::dense(&DENSE);

    let dense = sv_scene_to_svg(
        &sv_build_default(&dense_source)
            .compose()
            .expect("dense renders"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_default(&sparse_source)
            .compose()
            .expect("sparse renders"),
    );
    assert_eq!(
        dense, sparse,
        "a dropped sparse observation is not an implicit zero"
    );
}

#[test]
fn a_streaming_input_returns_promptly_and_shows_the_window() {
    let source = sv_streaming_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Streaming);

    let (sender, receiver) = mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let scene = plot.compose().expect("the streaming window renders");
            let _ = sender.send(scene.number_of_bars());
        });
    });

    let bars = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("the render must not block on the stream");
    assert_eq!(bars, 3);
}

#[test]
fn a_streaming_update_recomposes_the_current_window_promptly() {
    let source = sv_streaming_source();
    let plot = sv_build_default(&source);
    let output = plot.update(4).expect("the streaming update renders");

    let document = std::str::from_utf8(output.as_svg().expect("the update produces SVG"))
        .expect("the SVG document is UTF-8");
    assert!(document.contains("<rect"));
}

#[test]
fn a_large_dense_column_counts_without_losing_observations() {
    let categories: Vec<f64> = (0..10_000).map(|index| (index % 500) as f64).collect();
    let table = crate::sv_data::SVDataTableBuilder::new()
        .with_column(
            "category",
            crate::sv_data::SVColumnRole::Grouping,
            &categories,
        )
        .build()
        .expect("the large table is well formed");
    let scene = SVCountPlot::builder()
        .data_source(&table)
        .category("category")
        .order(crate::sv_stat::SVCountOrder::Ascending)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the large configuration is complete")
        .compose()
        .expect("the large count plot composes");

    assert_eq!(scene.number_of_categories(), 500);
    let total: f64 = scene.counts().iter().sum();
    assert_eq!(total, 10_000.0);
}

#[test]
fn a_missing_optional_series_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVCountPlot::builder()
        .data_source(&source)
        .category("category")
        .series("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing series column fails at build time");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn the_number_of_observations_is_the_probed_column_length() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.number_of_observations(), 6);
}
