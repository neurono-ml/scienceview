//! The backend contract: the same rasterizer, a canvas blit, and typed refusals.

use crate::SVHeadlessCanvas;
use crate::SVWasmBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::SVColour;
use scienceview_native::SVNativeBackend;
use scienceview_native::sv_native_default_viewport;
use std::path::Path;
use std::sync::Arc;

fn sv_wasm_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Wasm,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("WebAssembly"),
    }
}

fn sv_native_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: Some("WebAssembly"),
    }
}

#[test]
fn a_headless_wasm_frame_matches_the_native_frame() {
    let values = [1.0, 3.0, 2.0, 5.0];
    let wasm_frame = SVWasmBackend::new()
        .render_frame(&sv_wasm_request(&values))
        .expect("the shared rasterizer renders");
    let native_frame = SVNativeBackend::new()
        .render_frame(&sv_native_request(&values))
        .expect("the native rasterizer renders");

    assert_eq!(wasm_frame, native_frame);
    assert_eq!(wasm_frame.width(), 640);
    assert_eq!(wasm_frame.height(), 400);
    assert_eq!(
        wasm_frame.pixel(0, 0),
        Some(SVColour::new(255, 255, 255, 255))
    );
}

#[test]
fn the_render_output_bytes_are_the_shared_frame_pixels() {
    let values = [0.0, 1.0];
    let backend = SVWasmBackend::new();
    let output = backend
        .render(&sv_wasm_request(&values))
        .expect("the backend renders");

    assert_eq!(output.backend_kind, SVBackendKind::Wasm);
    assert_eq!(
        output.bytes,
        backend
            .render_frame(&sv_wasm_request(&values))
            .expect("the frame is comparable")
            .into_pixels()
    );
}

#[test]
fn a_resized_viewport_sizes_the_frame() {
    let values = [1.0, 2.0];
    let viewport = SVViewport::new(320.0, 200.0).with_device_pixel_ratio(2.0);
    let frame = SVWasmBackend::new()
        .render_frame_with_viewport(&sv_wasm_request(&values), &viewport)
        .expect("the rasterizer renders");

    assert_eq!((frame.width(), frame.height()), (640, 400));
    assert_eq!(frame.device_pixel_ratio(), 2.0);
}

#[test]
fn showing_blits_the_frame_into_the_canvas() {
    let values = [1.0, 2.0, 4.0];
    let viewport = sv_native_default_viewport();
    let canvas = Arc::new(SVHeadlessCanvas::new(viewport));
    let backend = SVWasmBackend::new().with_surface(canvas.clone());

    backend
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Wasm,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
        })
        .expect("the frame is blitted");

    assert_eq!(canvas.presented_frames(), 1);
    assert_eq!(
        canvas.frame(),
        Some(
            backend
                .render_frame(&SVRenderRequest {
                    title: None,
                    ..sv_wasm_request(&values)
                })
                .expect("the frame is comparable")
        )
    );
    assert_eq!(canvas.viewport(), viewport);
}

#[test]
fn showing_without_a_surface_reports_the_unavailable_backend() {
    let values = [1.0, 2.0];
    let error = SVWasmBackend::new()
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Wasm,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
        })
        .expect_err("no canvas is attached");

    assert_eq!(
        error,
        SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Wasm,
        }
    );
}

#[test]
fn saving_reports_the_missing_export_capability() {
    let values = [1.0, 2.0];
    let error = SVWasmBackend::new()
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::Wasm,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            destination: Path::new("figure.png"),
        })
        .expect_err("a canvas backend declares no export format");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Wasm,
            capability: scienceview_core::sv_backend::SVBackendCapability::Export,
        }
    );
}

#[test]
fn updating_rerenders_the_same_canvas_in_place() {
    let first_values = [1.0, 2.0, 3.0];
    let updated_values = [1.0, 2.0, 3.0, 4.0];
    let canvas = Arc::new(SVHeadlessCanvas::new(sv_native_default_viewport()));
    let backend = SVWasmBackend::new().with_surface(canvas.clone());

    backend
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Wasm,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&first_values),
        })
        .expect("the first frame is presented");
    let update = backend
        .update(&SVUpdateRequest {
            backend_kind: SVBackendKind::Wasm,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&updated_values),
            appended_points: 1,
        })
        .expect("the backend updates");
    backend
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Wasm,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&updated_values),
        })
        .expect("the updated frame is presented");

    assert_eq!(canvas.presented_frames(), 2);
    assert_eq!(
        update.bytes,
        canvas
            .frame()
            .expect("the canvas holds the updated frame")
            .into_pixels()
    );
}

#[test]
fn the_wasm_backend_serves_its_own_kind_in_the_registry() {
    use crate::SVWasmBackendProvider;
    use scienceview_core::sv_backend::SVRenderBackendProvider;
    use scienceview_core::sv_backend::sv_backend_registry;

    sv_register_once();
    let backend = sv_backend_registry()
        .resolve(SVBackendKind::Wasm)
        .expect("the provider is registered");

    assert_eq!(backend.kind(), SVBackendKind::Wasm);
    assert!(
        SVWasmBackendProvider::new()
            .provide(SVBackendKind::Native)
            .is_none()
    );
}

fn sv_register_once() {
    crate::sv_register_wasm_backend();
}
