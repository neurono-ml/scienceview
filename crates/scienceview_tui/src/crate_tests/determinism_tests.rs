//! Two renders of the same request are byte-identical.

use crate::backend::SVTuiBackend;
use crate::cell::SVAnsiColourDepth;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

fn sv_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Tui,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("Deterministic"),
    }
}

#[test]
fn two_renders_of_the_same_request_are_identical() {
    let values = [0.0, 1.0, 0.5, 2.0, 1.5];
    let backend = SVTuiBackend::new();

    let first = backend
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");
    let second = backend
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");

    assert_eq!(first, second);
    assert_eq!(
        first.to_ansi(SVAnsiColourDepth::Ansi256),
        second.to_ansi(SVAnsiColourDepth::Ansi256)
    );
}

#[test]
fn the_same_frame_repeats_at_every_colour_depth() {
    let values = [1.0, 2.0];
    let frame = SVTuiBackend::new()
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");

    for depth in [
        SVAnsiColourDepth::TrueColor,
        SVAnsiColourDepth::Ansi256,
        SVAnsiColourDepth::Ansi16,
    ] {
        assert_eq!(frame.to_ansi(depth), frame.clone().to_ansi(depth));
    }
}
