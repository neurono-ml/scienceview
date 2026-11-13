//! Representation parity, the data boundary and the streaming window.

use super::test_support::{
    SVTestRugSource, VALUES, sv_build_default, sv_dense_source, sv_scene_to_svg, sv_sparse_source,
    sv_streaming_source, sv_tick_segments,
};
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVRepresentation, SVSourceSchema, SVSparseView,
    SVStreamingView,
};
use crate::sv_error::SVError;

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_scene() {
    let dense_source = sv_dense_source();
    let sparse_source = sv_sparse_source();
    let streaming_source = sv_streaming_source();

    let dense = sv_scene_to_svg(
        &sv_build_default(&dense_source)
            .compose()
            .expect("dense composes"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_default(&sparse_source)
            .compose()
            .expect("sparse composes"),
    );
    let streaming = sv_scene_to_svg(
        &sv_build_default(&streaming_source)
            .compose()
            .expect("streaming composes"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn a_sparse_input_is_never_densified() {
    let source = sv_sparse_source();
    let plot = sv_build_default(&source);

    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .compose()
        .expect("a sparse view composes without being expanded");
    assert_eq!(scene.number_of_marks(), VALUES.len());
}

#[test]
fn a_sparse_input_marks_only_its_stored_entries() {
    const INDICES: [usize; 2] = [1, 4];
    const STORED: [f64; 2] = [2.0, 5.0];
    let sparse = SVSparseView::new(&INDICES, &STORED, 6).expect("the sparse view is valid");
    let source = SVTestRugSource {
        values: SVColumnView::Sparse(sparse),
        series: None,
    };
    let scene = sv_build_default(&source)
        .compose()
        .expect("only the stored entries are marked");

    assert_eq!(scene.number_of_marks(), 2);
    assert_eq!(sv_tick_segments(&scene).len(), 2);
}

#[test]
fn a_streaming_input_shows_its_current_window_without_blocking() {
    // Six samples were produced, the window holds the newest four.
    let window = &VALUES[2..];
    let source = SVTestRugSource {
        values: SVColumnView::Streaming(SVStreamingView::new(window, 6, 2)),
        series: None,
    };
    let plot = sv_build_default(&source);

    assert_eq!(plot.representation(), SVRepresentation::Streaming);
    let scene = plot.compose().expect("the current window composes");
    assert_eq!(scene.number_of_marks(), window.len());
    assert_eq!(sv_tick_segments(&scene).len(), window.len());
}

#[test]
fn a_missing_optional_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = super::SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .series("absent")
        .backend(crate::sv_backend::SVBackendKind::Svg)
        .build()
        .expect_err("a missing series column fails at build time");

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

struct SVMismatchedValues {
    values: SVColumnView<'static>,
    series: SVColumnView<'static>,
}

impl SVDataSource for SVMismatchedValues {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the rug resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "value" => Ok(SVColumnSnapshot::Borrowed(self.values)),
            "series" => Ok(SVColumnSnapshot::Borrowed(self.series)),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn columns_of_different_lengths_fail_at_build_time() {
    const SERIES: [f64; 2] = [0.0, 1.0];
    let source = SVMismatchedValues {
        values: SVColumnView::Dense(&VALUES),
        series: SVColumnView::Dense(&SERIES),
    };
    let error = super::SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .series("series")
        .backend(crate::sv_backend::SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_series_represented_sparsely_agrees_with_its_dense_twin() {
    const INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    const SERIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
    let sparse_series =
        SVSparseView::new(&INDICES, &SERIES, 6).expect("the sparse series view is valid");
    let sparse_source = SVTestRugSource {
        values: SVColumnView::Dense(&VALUES),
        series: Some(SVColumnView::Sparse(sparse_series)),
    };
    let dense_source = SVTestRugSource {
        values: SVColumnView::Dense(&VALUES),
        series: Some(SVColumnView::Dense(&SERIES)),
    };

    let sparse = sv_scene_to_svg(
        &super::SVRugPlot::builder()
            .data_source(&sparse_source)
            .value("value")
            .series("series")
            .backend(crate::sv_backend::SVBackendKind::Svg)
            .build()
            .expect("the sparse series configuration is complete")
            .compose()
            .expect("the sparse series composes"),
    );
    let dense = sv_scene_to_svg(
        &super::SVRugPlot::builder()
            .data_source(&dense_source)
            .value("value")
            .series("series")
            .backend(crate::sv_backend::SVBackendKind::Svg)
            .build()
            .expect("the dense series configuration is complete")
            .compose()
            .expect("the dense series composes"),
    );
    assert_eq!(
        dense, sparse,
        "the series representation never changes the ticks"
    );
}
