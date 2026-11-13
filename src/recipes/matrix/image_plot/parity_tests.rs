//! Representation parity, the data boundary and the streaming window.

use super::test_support::{COLUMNS, FIELD, ROWS, SVTestSource, sv_build_default};
use crate::sv_data::{SVRepresentation, SVSparseView};
use crate::sv_scale::SVColour;

const INDICES: [usize; ROWS * COLUMNS] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_raster() {
    let dense_source = SVTestSource::dense();
    let sparse_view = SVSparseView::new(&INDICES, &FIELD, FIELD.len()).expect("valid sparse view");
    let sparse_source = SVTestSource::sparse(sparse_view);
    let streaming_source = SVTestSource::streaming(&FIELD, 7);

    let dense = sv_build_default(&dense_source)
        .compose()
        .expect("dense composes");
    let sparse = sv_build_default(&sparse_source)
        .compose()
        .expect("sparse composes");
    let streaming = sv_build_default(&streaming_source)
        .compose()
        .expect("streaming composes");

    assert_eq!(dense.raster(), sparse.raster(), "dense and sparse agree");
    assert_eq!(
        dense.raster(),
        streaming.raster(),
        "dense and streaming agree"
    );
}

#[test]
fn a_sparse_input_is_never_densified() {
    let sparse_view = SVSparseView::new(&INDICES, &FIELD, FIELD.len()).expect("valid sparse view");
    let source = SVTestSource::sparse(sparse_view);
    let plot = sv_build_default(&source);

    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse raster composes without an expansion");
    assert_eq!(scene.dimensions().rows(), ROWS);
    assert_eq!(scene.dimensions().columns(), COLUMNS);
}

#[test]
fn a_streaming_window_renders_its_newest_frame() {
    const SECOND_FRAME: [f64; ROWS * COLUMNS] =
        [11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0];
    let mut window = [0.0_f64; ROWS * COLUMNS * 2];
    window[..ROWS * COLUMNS].copy_from_slice(&FIELD);
    window[ROWS * COLUMNS..].copy_from_slice(&SECOND_FRAME);

    let streaming_source = SVTestSource::streaming(&window, 2);
    let streamed = sv_build_default(&streaming_source)
        .compose()
        .expect("the window composes");

    assert_eq!(
        streamed.pixel_colour(0, 0),
        Some(SVColour::new(255, 255, 255, 255))
    );
    assert_eq!(streamed.number_of_pixels(), ROWS * COLUMNS);
}

#[test]
fn a_short_streaming_window_fills_the_frame_it_has() {
    const PARTIAL: [f64; 3] = [0.0, 5.0, 10.0];
    let source = SVTestSource::streaming(&PARTIAL, 1);
    let scene = sv_build_default(&source)
        .compose()
        .expect("a short window still composes");

    assert_eq!(scene.pixel_colour(0, 0), Some(SVColour::new(0, 0, 0, 255)));
    assert_eq!(
        scene.pixel_colour(ROWS - 1, COLUMNS - 1),
        Some(SVColour::new(0, 0, 0, 0)),
        "the cells the window does not hold yet stay transparent"
    );
}

#[test]
fn an_empty_streaming_window_composes_a_transparent_frame() {
    let values: [f64; 0] = [];
    let source = SVTestSource::streaming(&values, 0);
    let scene = sv_build_default(&source)
        .compose()
        .expect("an empty window composes");

    assert_eq!(scene.number_of_pixels(), ROWS * COLUMNS);
    assert_eq!(scene.pixel_colour(1, 1), Some(SVColour::new(0, 0, 0, 0)));
}

#[test]
fn a_streaming_column_is_reported_as_streaming() {
    let source = SVTestSource::streaming(&FIELD, 3);
    let plot = sv_build_default(&source);
    assert_eq!(plot.representation(), SVRepresentation::Streaming);
}
