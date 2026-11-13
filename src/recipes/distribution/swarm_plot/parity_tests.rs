//! Representation parity, the data boundary and the streaming path.

use super::SVSwarmPlot;
use super::test_support::{
    SVTestSource, assert_close, circle_centres, dense_source, scene, scene_svg, sparse_source,
    streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVColumnView, SVRepresentation, SVSparseView};

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_scene() {
    let dense = scene_svg(&scene(&dense_source()));
    let sparse = scene_svg(&scene(&sparse_source()));
    let streaming = scene_svg(&scene(&streaming_source()));

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn dense_sparse_and_streaming_inputs_place_the_same_dots() {
    assert_close(
        &circle_centres(&scene(&dense_source())),
        &circle_centres(&scene(&sparse_source())),
    );
    assert_close(
        &circle_centres(&scene(&dense_source())),
        &circle_centres(&scene(&streaming_source())),
    );
}

#[test]
fn a_sparse_input_is_never_densified() {
    const VALUES: [f64; 3] = [1.0, 2.0, 3.0];
    const CATEGORY: [f64; 3] = [0.0, 0.0, 0.0];
    const INDICES: [usize; 3] = [0, 2, 4];
    let value = SVSparseView::new(&INDICES, &VALUES, 1_000_000).expect("valid sparse storage");
    let category = SVSparseView::new(&INDICES, &CATEGORY, 1_000_000).expect("valid sparse storage");
    let source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Sparse(value)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(category),
        ),
    ]);

    let plot = super::test_support::default_builder(&source)
        .build()
        .expect("a huge sparse column still builds");
    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot.compose().expect("a sparse view renders unexpanded");
    assert_eq!(
        scene.number_of_marks(),
        INDICES.len(),
        "only the stored entries become observations"
    );
}

#[test]
fn a_sparse_column_matches_the_dense_column_storing_the_same_entries() {
    const THREE: [f64; 3] = [1.0, 2.0, 3.0];
    const ZERO: [f64; 3] = [0.0; 3];
    const INDICES: [usize; 3] = [0, 1, 2];
    let sparse = SVSparseView::new(&INDICES, &THREE, 3).expect("valid sparse storage");
    let sparse_category = SVSparseView::new(&INDICES, &ZERO, 3).expect("valid sparse storage");
    let sparse_source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Sparse(sparse)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Sparse(sparse_category),
        ),
    ]);
    let dense_source = SVTestSource::new(vec![
        ("value", SVColumnRole::Y, SVColumnView::Dense(&THREE)),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Dense(&ZERO),
        ),
    ]);

    assert_eq!(
        scene_svg(&scene(&dense_source)),
        scene_svg(&scene(&sparse_source))
    );
}

#[test]
fn every_series_representation_combination_is_accepted() {
    const VALUES: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
    const CATEGORY: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
    const SERIES: [f64; 4] = [0.0, 1.0, 0.0, 1.0];
    const INDICES: [usize; 4] = [0, 1, 2, 3];
    let sparse_series = SVSparseView::new(&INDICES, &SERIES, 4).expect("valid sparse storage");
    let combinations = [
        (SVColumnView::Dense(&VALUES), SVColumnView::Dense(&SERIES)),
        (
            SVColumnView::Dense(&VALUES),
            SVColumnView::Sparse(sparse_series),
        ),
    ];

    for (values, series) in combinations {
        let source = SVTestSource::new(vec![
            ("value", SVColumnRole::Y, values),
            (
                "category",
                SVColumnRole::Grouping,
                SVColumnView::Dense(&CATEGORY),
            ),
            ("series", SVColumnRole::Grouping, series),
        ]);
        let scene = SVSwarmPlot::builder()
            .data_source(&source)
            .value("value")
            .category("category")
            .series("series")
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete")
            .compose()
            .expect("every combination composes");
        assert_eq!(scene.number_of_marks(), 4);
        assert_eq!(scene.number_of_series(), 2);
    }
}
