//! The headless canvas: a frame in, the same frame out.

use crate::SVCanvasSurface;
use crate::SVHeadlessCanvas;
use crate::SVNativeFrame;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;

fn sv_frame(width: usize, height: usize, value: u8) -> SVNativeFrame {
    SVNativeFrame::new(width, height, 1.0, vec![value; width * height * 4])
}

#[test]
fn a_blit_stores_the_frame_it_received() {
    let canvas = SVHeadlessCanvas::new(SVViewport::new(4.0, 3.0));
    let frame = sv_frame(4, 3, 17);

    canvas.blit(&frame).expect("the frame matches the canvas");

    assert_eq!(canvas.frame(), Some(frame));
    assert_eq!(canvas.presented_frames(), 1);
}

#[test]
fn a_frame_of_the_wrong_size_is_rejected() {
    let canvas = SVHeadlessCanvas::new(SVViewport::new(4.0, 3.0));

    let error = canvas
        .blit(&sv_frame(8, 6, 1))
        .expect_err("the frame does not match the canvas");

    assert!(matches!(
        error,
        SVError::BackendFailure {
            backend_kind: SVBackendKind::Wasm,
            operation: "show",
            ..
        }
    ));
    assert_eq!(canvas.frame(), None);
    assert_eq!(canvas.presented_frames(), 0);
}

#[test]
fn the_canvas_keeps_the_viewport_it_was_sized_for() {
    let viewport = SVViewport::new(320.0, 200.0).with_device_pixel_ratio(2.0);
    let canvas = SVHeadlessCanvas::new(viewport);

    assert_eq!(canvas.viewport(), viewport);
    assert_eq!(canvas.viewport().physical_width(), 640.0);
}
