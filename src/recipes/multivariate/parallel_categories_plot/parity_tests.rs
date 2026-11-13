//! Representation parity, the data boundary and the streaming path.

use super::test_support::{
    sv_build_default, sv_dense_source, sv_mixed_source, sv_scene_to_svg, sv_sparse_source,
    sv_streaming_source,
};
use crate::sv_data::SVRepresentation;

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

    assert_eq!(dense, sparse, "dense and sparse agree byte for byte");
    assert_eq!(dense, streaming, "dense and streaming agree byte for byte");
}

#[test]
fn mixed_representations_agree_after_validating_lengths() {
    let dense = sv_scene_to_svg(
        &sv_build_default(&sv_dense_source())
            .compose()
            .expect("dense renders"),
    );
    let mixed = sv_scene_to_svg(
        &sv_build_default(&sv_mixed_source())
            .compose()
            .expect("the mixed representations render"),
    );
    assert_eq!(dense, mixed);
}

#[test]
fn a_sparse_input_is_reported_as_sparse_and_never_densified() {
    let source = sv_sparse_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse view composes without being expanded");
    assert_eq!(scene.number_of_observations(), 12);
    assert_eq!(scene.number_of_axes(), 3);
}

#[test]
fn a_sparse_input_with_implicit_zeros_matches_the_dense_input() {
    const DENSE: [f64; 6] = [0.0, 0.0, 2.0, 0.0, 2.0, 1.0];
    const STORED_INDICES: [usize; 3] = [2, 4, 5];
    const STORED_VALUES: [f64; 3] = [2.0, 2.0, 1.0];
    let sparse =
        crate::sv_data::SVSparseView::new(&STORED_INDICES, &STORED_VALUES, 6).expect("valid");
    let sparse_source = super::test_support::SVTestSource {
        first: crate::sv_data::SVColumnView::Sparse(sparse),
        second: crate::sv_data::SVColumnView::Dense(&DENSE),
        third: crate::sv_data::SVColumnView::Dense(&DENSE),
        colour: None,
    };
    let dense_source = super::test_support::SVTestSource {
        first: crate::sv_data::SVColumnView::Dense(&DENSE),
        second: crate::sv_data::SVColumnView::Dense(&DENSE),
        third: crate::sv_data::SVColumnView::Dense(&DENSE),
        colour: None,
    };

    let dense = sv_build_default(&dense_source)
        .compose()
        .expect("dense renders");
    let sparse = sv_build_default(&sparse_source)
        .compose()
        .expect("sparse renders");
    assert_eq!(dense.number_of_ribbons(), sparse.number_of_ribbons());
    assert_eq!(sv_scene_to_svg(&dense), sv_scene_to_svg(&sparse));
}

#[test]
fn a_streaming_input_composes_from_the_current_window_without_blocking() {
    let source = sv_streaming_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Streaming);
    let scene = plot.compose().expect("the window composes promptly");
    assert_eq!(scene.number_of_observations(), 12);
}
