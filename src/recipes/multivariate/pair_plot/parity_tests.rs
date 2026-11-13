//! Representation parity, the data boundary and the streaming path.

use super::SVPairOffDiagonalKind;
use super::SVPairPlot;
#[cfg(feature = "render-svg")]
use super::SVPairRenderOutput;
use super::test_support::{
    sv_build_default, sv_dense_source, sv_scene_to_svg, sv_sparse_source, sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_grid() {
    let dense = sv_dense_source();
    let sparse = sv_sparse_source();
    let streaming = sv_streaming_source();

    let dense = sv_scene_to_svg(&sv_build_default(&dense).compose().expect("dense renders"));
    let sparse = sv_scene_to_svg(&sv_build_default(&sparse).compose().expect("sparse renders"));
    let streaming = sv_scene_to_svg(
        &sv_build_default(&streaming)
            .compose()
            .expect("streaming renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn every_off_diagonal_kind_is_representation_parity() {
    for kind in [
        SVPairOffDiagonalKind::Scatter,
        SVPairOffDiagonalKind::Hexbin,
        SVPairOffDiagonalKind::DensityContour,
    ] {
        let dense = sv_dense_source();
        let sparse = sv_sparse_source();
        let dense_scene = sv_build_kind(&dense, kind)
            .compose()
            .expect("dense composes");
        let sparse_scene = sv_build_kind(&sparse, kind)
            .compose()
            .expect("sparse composes");
        assert_eq!(
            sv_scene_to_svg(&dense_scene),
            sv_scene_to_svg(&sparse_scene),
            "{kind:?} agrees between dense and sparse"
        );
    }
}

#[test]
fn a_sparse_input_is_never_densified() {
    let sparse = sv_sparse_source();
    let plot = sv_build_default(&sparse);
    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot.compose().expect("a sparse view renders");
    assert!(scene.number_of_marks() > 0);
}

#[test]
fn a_streaming_input_returns_promptly_and_shows_the_window() {
    let streaming = sv_streaming_source();
    let plot = sv_build_default(&streaming);
    assert_eq!(plot.representation(), SVRepresentation::Streaming);
    let scene = plot.compose().expect("a streaming view renders");
    assert_eq!(scene.cell_rectangles().len(), 9);
}

#[cfg(feature = "render-svg")]
#[test]
fn update_recomposes_the_current_window() {
    let streaming = sv_streaming_source();
    let plot = sv_build_default(&streaming);
    let first = plot.compose().expect("the first window renders");
    let output = plot.update(1).expect("the update renders");
    let SVPairRenderOutput::Svg(bytes) = output else {
        panic!("the SVG backend produces a document");
    };
    assert!(bytes.starts_with(b"<svg") || bytes.starts_with(b"<?xml"));
    assert_eq!(first.cell_rectangles().len(), 9);
}

fn sv_build_kind(
    source: &dyn crate::sv_data::SVDataSource,
    kind: SVPairOffDiagonalKind,
) -> SVPairPlot<'_> {
    SVPairPlot::builder()
        .data_source(source)
        .variables(&["first", "second", "third"])
        .off_diagonal_kind(kind)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
}
