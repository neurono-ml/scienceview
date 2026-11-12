//! Representation parity, the data boundary and the streaming path.

use super::SVScatterPlot;
use super::scatter_plot_tests::{
    SVTestSource, sv_build_default, sv_dense_source, sv_scene_to_svg, sv_sparse_source,
    sv_streaming_source,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{
    SVColumnSnapshot, SVColumnView, SVDataSource, SVDataTableBuilder, SVRepresentation,
    SVSourceSchema, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_mark::SVDotShape;
use std::sync::mpsc;
use std::time::Duration;

const HORIZONTAL: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const VERTICAL: [f64; 5] = [1.0, 4.0, 2.0, 5.0, 3.0];

#[test]
fn dense_sparse_and_streaming_inputs_draw_the_same_scene() {
    let dense_source = sv_dense_source();
    let sparse_source = sv_sparse_source();
    let streaming_source = sv_streaming_source();

    let dense = sv_scene_to_svg(
        &sv_build_default(&dense_source)
            .render()
            .expect("dense renders"),
    );
    let sparse = sv_scene_to_svg(
        &sv_build_default(&sparse_source)
            .render()
            .expect("sparse renders"),
    );
    let streaming = sv_scene_to_svg(
        &sv_build_default(&streaming_source)
            .render()
            .expect("streaming renders"),
    );

    assert_eq!(dense, sparse, "dense and sparse agree");
    assert_eq!(dense, streaming, "dense and streaming agree");
}

#[test]
fn a_sparse_input_is_never_densified() {
    let sparse_source = sv_sparse_source();
    let plot = sv_build_default(&sparse_source);

    assert_eq!(plot.representation(), SVRepresentation::Sparse);
    let scene = plot
        .render()
        .expect("a sparse view renders without being expanded");
    assert_eq!(scene.number_of_marks(), HORIZONTAL.len());
}

#[test]
fn a_sparse_input_with_implicit_zeros_keeps_its_logical_points() {
    const DENSE_X: [f64; 5] = [0.0, 0.0, 2.0, 0.0, 4.0];
    const INDICES: [usize; 2] = [2, 4];
    const STORED_X: [f64; 2] = [2.0, 4.0];
    let sparse = SVSparseView::new(&INDICES, &STORED_X, 5).expect("the sparse view is valid");
    let source = SVTestSource {
        horizontal: SVColumnView::Sparse(sparse),
        vertical: SVColumnView::Dense(&VERTICAL),
        series: None,
        sizes: None,
        colours: None,
    };
    let scene = sv_build_default(&source)
        .render()
        .expect("implicit zeros stay logical points");
    assert_eq!(scene.number_of_marks(), DENSE_X.len());
}

#[test]
fn a_streaming_input_returns_promptly_and_shows_the_window() {
    let streaming_source = sv_streaming_source();
    let plot = sv_build_default(&streaming_source);
    assert_eq!(plot.representation(), SVRepresentation::Streaming);

    let (sender, receiver) = mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let scene = plot.render().expect("the streaming window renders");
            let _ = sender.send(scene.number_of_marks());
        });
    });

    let points = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("the render must not block on the stream");
    assert_eq!(points, HORIZONTAL.len());
}

#[test]
fn a_missing_optional_column_fails_with_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
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

struct SVMismatchedSource {
    horizontal: SVColumnView<'static>,
    vertical: SVColumnView<'static>,
}

impl SVDataSource for SVMismatchedSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        unreachable!("the scatter resolves columns by name")
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "x" => Ok(SVColumnSnapshot::Borrowed(self.horizontal)),
            "y" => Ok(SVColumnSnapshot::Borrowed(self.vertical)),
            _ => Err(SVError::ColumnNotFound {
                name: name.to_string(),
            }),
        }
    }
}

#[test]
fn columns_of_different_lengths_fail_at_build_time() {
    let source = SVMismatchedSource {
        horizontal: SVColumnView::Dense(&HORIZONTAL),
        vertical: SVColumnView::Dense(&HORIZONTAL[..3]),
    };
    let error = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("mismatched columns fail before rendering");

    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn an_empty_source_renders_an_empty_scene() {
    let horizontal: [f64; 0] = [];
    let vertical: [f64; 0] = [];
    let table = SVDataTableBuilder::new()
        .with_column("x", crate::sv_data::SVColumnRole::X, &horizontal)
        .with_column("y", crate::sv_data::SVColumnRole::Y, &vertical)
        .build()
        .expect("the empty table is well formed");
    let scene = SVScatterPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("an empty scatter renders");

    assert_eq!(scene.number_of_marks(), 0);
    assert_eq!(scene.number_of_mark_primitives(), 0);
}

#[test]
fn jitter_displaces_points_deterministically() {
    let source = sv_dense_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .jitter(crate::sv_move::SVMove::jitter().seed(7).into())
        .marker(SVDotShape::Circle)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the jitter configuration is complete");

    let first = sv_scene_to_svg(&plot.render().expect("the jittered scatter renders"));
    let second = sv_scene_to_svg(&plot.render().expect("the jittered scatter renders again"));
    assert_eq!(first, second, "the seeded jitter is deterministic");

    let plain_source = sv_dense_source();
    let plain = sv_scene_to_svg(
        &sv_build_default(&plain_source)
            .render()
            .expect("the plain scatter renders"),
    );
    assert_ne!(first, plain, "jitter moves the marks");
}

#[test]
fn a_streaming_scene_can_be_serialized_as_svg() {
    let streaming_source = SVTestSource {
        horizontal: SVColumnView::Streaming(SVStreamingView::new(&HORIZONTAL, 11, 3)),
        vertical: SVColumnView::Streaming(SVStreamingView::new(&VERTICAL, 11, 3)),
        series: None,
        sizes: None,
        colours: None,
    };
    let scene = sv_build_default(&streaming_source)
        .render()
        .expect("the streaming scene renders");
    let svg = sv_scene_to_svg(&scene);
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("<path"), "the marks are serialized");
}
