//! The backend contract: headless frames, bytes, update and viewport sizing.

use crate::backend::SVNativeBackend;
use crate::backend::sv_native_default_viewport;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::SVColour;

fn sv_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("Native"),
    }
}

#[test]
fn rendering_a_figure_produces_a_comparable_frame() {
    let values = [1.0, 3.0, 2.0, 5.0];
    let frame = SVNativeBackend::new()
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");

    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 400);
    assert_eq!(frame.device_pixel_ratio(), 1.0);
    assert_eq!(frame.pixels().len(), 640 * 400 * 4);
    assert_eq!(frame.pixel(0, 0), Some(SVColour::new(255, 255, 255, 255)));
}

#[test]
fn two_renders_of_the_same_request_are_identical() {
    let values = [1.0, 3.0, 2.0, 5.0];
    let backend = SVNativeBackend::new();

    let first = backend
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");
    let second = backend
        .render_frame(&sv_request(&values))
        .expect("the rasterizer renders");

    assert_eq!(first, second);
}

#[test]
fn the_render_output_bytes_are_the_frame_pixels() {
    let values = [0.0, 1.0];
    let backend = SVNativeBackend::new();
    let output = backend
        .render(&sv_request(&values))
        .expect("the backend renders");

    assert_eq!(output.backend_kind, SVBackendKind::Native);
    assert_eq!(output.bytes.len(), 640 * 400 * 4);
    assert_eq!(
        output.bytes,
        backend
            .render_frame(&sv_request(&values))
            .expect("the frame is comparable")
            .into_pixels()
    );
}

#[test]
fn a_resized_viewport_sizes_the_frame() {
    let values = [1.0, 2.0];
    let viewport = SVViewport::new(320.0, 200.0).with_device_pixel_ratio(2.0);
    let frame = SVNativeBackend::new()
        .render_frame_with_viewport(&sv_request(&values), &viewport)
        .expect("the rasterizer renders");

    assert_eq!((frame.width(), frame.height()), (640, 400));
    assert_eq!(frame.device_pixel_ratio(), 2.0);
}

#[test]
fn updating_rerenders_the_same_window_in_place() {
    let values = [1.0, 2.0, 3.0];
    let backend = SVNativeBackend::new();
    let update = SVUpdateRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&values),
        appended_points: 1,
    };

    let output = backend.update(&update).expect("the backend updates");
    let frame = backend
        .render_frame(&SVRenderRequest {
            title: None,
            ..sv_request(&values)
        })
        .expect("the backend renders");

    assert_eq!(output.backend_kind, SVBackendKind::Native);
    assert_eq!(output.bytes, frame.into_pixels());
}

#[test]
fn the_default_viewport_is_the_documented_one() {
    let viewport = sv_native_default_viewport();

    assert_eq!(viewport.width(), 640.0);
    assert_eq!(viewport.height(), 400.0);
    assert_eq!(viewport.device_pixel_ratio(), 1.0);
}
