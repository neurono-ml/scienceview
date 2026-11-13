//! Representation parity, sparse storage and streaming windows.

use super::test_support::{
    SVTestSource, sv_builder, sv_compose_default, sv_dense_source, sv_primitives, sv_sparse_source,
    sv_streaming_source, sv_values,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnView, SVDataSource};

fn sv_streaming_composition(source: &dyn SVDataSource) -> super::SVDensityHeatmapComposition {
    sv_builder(source)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the streaming chart builds")
        .compose()
        .expect("the streaming chart composes")
}

#[test]
fn dense_sparse_and_streaming_agree_within_the_documented_tolerance() {
    let dense = sv_values(&sv_dense_source());
    let sparse = sv_values(&sv_sparse_source());
    let streaming = sv_streaming_composition(&sv_streaming_source()).grids()[0]
        .values()
        .to_vec();

    assert_eq!(dense, sparse, "the sparse grid matches the dense one");
    assert_eq!(dense, streaming, "the streaming grid matches the dense one");
}

#[test]
fn dense_sparse_and_streaming_produce_the_same_cells() {
    let dense = sv_primitives(&sv_compose_default(&sv_dense_source()));
    let sparse = sv_primitives(&sv_compose_default(&sv_sparse_source()));
    let streaming = sv_primitives(&sv_streaming_composition(&sv_streaming_source()));

    assert_eq!(dense, sparse);
    assert_eq!(dense, streaming);
}

#[test]
fn a_structurally_sparse_column_is_never_densified() {
    // Only two entries are stored; the other four logical positions are
    // implicit zeros. The grid must match the dense zero-filled baseline
    // without ever materialising the six-value column.
    const INDICES: [usize; 2] = [1, 4];
    const STORED_X: [f64; 2] = [1.0, 4.0];
    const STORED_Y: [f64; 2] = [2.0, 5.0];
    let sparse_x = crate::sv_data::SVSparseView::new(&INDICES, &STORED_X, 6)
        .expect("the sparse view is well formed");
    let sparse_y = crate::sv_data::SVSparseView::new(&INDICES, &STORED_Y, 6)
        .expect("the sparse view is well formed");
    let sparse_source = SVTestSource {
        horizontal: SVColumnView::Sparse(sparse_x),
        vertical: SVColumnView::Sparse(sparse_y),
        series: None,
        weight: None,
    };
    let dense_x = [0.0, 1.0, 0.0, 0.0, 4.0, 0.0];
    let dense_y = [0.0, 2.0, 0.0, 0.0, 5.0, 0.0];
    let dense_source = SVTestSource::dense(&dense_x, &dense_y);

    assert_eq!(sv_values(&sparse_source), sv_values(&dense_source));
    assert_eq!(
        sv_primitives(&sv_compose_default(&sparse_source)),
        sv_primitives(&sv_compose_default(&dense_source))
    );
}

#[test]
fn a_sparse_x_pairs_with_a_dense_y() {
    const INDICES: [usize; 3] = [0, 2, 5];
    const STORED_X: [f64; 3] = [0.0, 2.0, 5.0];
    let sparse_x = crate::sv_data::SVSparseView::new(&INDICES, &STORED_X, 6)
        .expect("the sparse view is well formed");
    let dense_y = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let mixed = SVTestSource {
        horizontal: SVColumnView::Sparse(sparse_x),
        vertical: SVColumnView::Dense(&dense_y),
        series: None,
        weight: None,
    };
    let baseline_x = [0.0, 0.0, 2.0, 0.0, 0.0, 5.0];
    let baseline = SVTestSource::dense(&baseline_x, &dense_y);

    assert_eq!(sv_values(&mixed), sv_values(&baseline));
}

#[test]
fn a_streaming_chart_reads_its_current_window_without_blocking() {
    let short_x = [0.0, 1.0];
    let short_y = [0.0, 1.0];
    let long_x = [0.0, 1.0, 2.0, 3.0];
    let long_y = [0.0, 1.0, 2.0, 3.0];
    let short_source = SVTestSource {
        horizontal: SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&short_x, 2, 0)),
        vertical: SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&short_y, 2, 0)),
        series: None,
        weight: None,
    };
    let long_source = SVTestSource {
        horizontal: SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&long_x, 4, 0)),
        vertical: SVColumnView::Streaming(crate::sv_data::SVStreamingView::new(&long_y, 4, 0)),
        series: None,
        weight: None,
    };

    let short = sv_builder(&short_source)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the short chart builds");
    let long = sv_builder(&long_source)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the long chart builds");

    assert_eq!(short.point_count(), 2);
    assert_eq!(long.point_count(), 4);
    assert_eq!(
        short.compose().expect("composes").grids()[0].values().len(),
        48
    );
}

#[test]
fn the_representation_is_the_dominant_one_of_the_bound_columns() {
    let sparse = sv_compose_default(&sv_sparse_source());
    assert_eq!(sparse.representation().to_string(), "Sparse");
    let streaming_source = sv_streaming_source();
    let streaming = sv_builder(&streaming_source)
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");
    assert_eq!(streaming.representation().to_string(), "Streaming");
}

#[test]
fn an_export_only_backend_rejects_a_streaming_column_at_build() {
    let source = sv_streaming_source();
    let result = sv_builder(&source)
        .backend(SVBackendKind::Svg)
        .build()
        .map(|_| ());

    assert!(result.is_err());
}
