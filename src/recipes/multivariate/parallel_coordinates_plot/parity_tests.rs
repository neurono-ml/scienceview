//! Representation parity and the data boundary.

use super::SVParallelCoordinatesPlot;
use super::test_support::{
    sv_build_default, sv_dense_source, sv_output_svg, sv_partially_sparse_source, sv_sparse_source,
    sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_polylines() {
    let dense_source = sv_dense_source();
    let sparse_source = sv_sparse_source();
    let streaming_source = sv_streaming_source();

    let dense = sv_output_svg(
        &sv_build_default(&dense_source)
            .compose()
            .expect("the dense source renders"),
    );
    let sparse = sv_output_svg(
        &sv_build_default(&sparse_source)
            .compose()
            .expect("the sparse source renders"),
    );
    let streaming = sv_output_svg(
        &sv_build_default(&streaming_source)
            .compose()
            .expect("the streaming source renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree byte for byte");
    assert_eq!(dense, streaming, "dense and streaming agree byte for byte");
}

#[test]
fn a_partially_sparse_column_is_read_through_its_stored_entries() {
    let source = sv_partially_sparse_source();
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Sparse);

    let output = plot
        .compose()
        .expect("the sparse source renders without being expanded");
    assert_eq!(output.representation(), SVRepresentation::Sparse);
    assert_eq!(output.lines().len(), 4);
    let second = output
        .lines()
        .iter()
        .map(|line| line.vertices()[1].y())
        .collect::<Vec<_>>();
    assert_eq!(
        second.iter().filter(|value| **value == second[1]).count(),
        2,
        "the implicit zeros share one display position"
    );
}

#[test]
fn a_streaming_input_draws_its_current_window() {
    let source = sv_streaming_source();
    let output = SVParallelCoordinatesPlot::builder()
        .data_source(&source)
        .variables(&["first", "second", "third"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the streaming window renders without blocking");

    assert_eq!(output.representation(), SVRepresentation::Streaming);
    assert_eq!(output.number_of_observations(), 4);
    assert_eq!(output.lines().len(), 4);
}
