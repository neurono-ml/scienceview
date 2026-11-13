//! The browser canvas surface, compiled only for the WebAssembly target.
//!
//! A `web-sys` canvas is not `Send`: JavaScript objects belong to the single
//! browser thread. The backend contract, however, is `Send + Sync`. Rather than
//! reaching for `unsafe impl Send`, this module keeps every JavaScript value in
//! a thread-local registry and hands the backend a [`SVBrowserCanvasHandle`],
//! whose only field is a token. Blits hop through the registry on the thread
//! that created the canvas — the only thread the browser has.

use super::surface::SVCanvasSurface;
use crate::SVNativeFrame;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

thread_local! {
    static CANVASES: RefCell<Vec<(usize, SVBrowserCanvas)>> = const { RefCell::new(Vec::new()) };
}

static NEXT_TOKEN: AtomicUsize = AtomicUsize::new(1);

/// A `web-sys` canvas and its 2D context.
struct SVBrowserCanvas {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl SVBrowserCanvas {
    fn open(element_id: &str) -> Result<Self, SVError> {
        let window = web_sys::window().ok_or_else(|| sv_browser_error("no window is available"))?;
        let document = window
            .document()
            .ok_or_else(|| sv_browser_error("no document is available"))?;
        let element = document
            .get_element_by_id(element_id)
            .ok_or_else(|| sv_browser_error("the canvas element was not found"))?;
        let canvas = element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| sv_browser_error("the element is not a canvas"))?;
        let context = canvas
            .get_context("2d")
            .map_err(|_| sv_browser_error("the 2D context could not be requested"))?
            .ok_or_else(|| sv_browser_error("the canvas has no 2D context"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| sv_browser_error("the context is not a 2D canvas context"))?;

        Ok(Self { canvas, context })
    }

    fn viewport(&self) -> SVViewport {
        let ratio = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .filter(|ratio| ratio.is_finite() && *ratio > 0.0)
            .unwrap_or(1.0);
        let device_width = f64::from(self.canvas.width());
        let device_height = f64::from(self.canvas.height());

        SVViewport::new(device_width / ratio, device_height / ratio).with_device_pixel_ratio(ratio)
    }

    fn put_frame(&self, frame: &SVNativeFrame) -> Result<(), SVError> {
        let width = frame.width() as u32;
        let height = frame.height() as u32;
        if self.canvas.width() != width || self.canvas.height() != height {
            return Err(sv_browser_error(
                "the canvas size does not match the rendered frame",
            ));
        }

        let image = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(frame.pixels()),
            width,
            height,
        )
        .map_err(|_| sv_browser_error("the frame could not be copied into an image"))?;

        self.context
            .put_image_data(&image, 0.0, 0.0)
            .map_err(|_| sv_browser_error("the frame could not be blitted"))
    }
}

/// A `Send + Sync` handle to a canvas living on the browser thread.
pub struct SVBrowserCanvasHandle {
    token: usize,
    viewport: SVViewport,
}

impl SVBrowserCanvasHandle {
    /// Opens the canvas with `element_id` and registers it for this thread.
    pub fn open(element_id: &str) -> Result<Self, SVError> {
        let canvas = SVBrowserCanvas::open(element_id)?;
        let viewport = canvas.viewport();
        let token = NEXT_TOKEN.fetch_add(1, Ordering::Relaxed);

        CANVASES.with(|canvases| {
            canvases.borrow_mut().push((token, canvas));
        });

        Ok(Self { token, viewport })
    }

    /// Removes the canvas from the thread-local registry.
    pub fn close(self) {
        let token = self.token;
        CANVASES.with(|canvases| {
            canvases
                .borrow_mut()
                .retain(|(registered, _)| *registered != token);
        });
    }
}

impl SVCanvasSurface for SVBrowserCanvasHandle {
    fn viewport(&self) -> SVViewport {
        self.viewport
    }

    fn blit(&self, frame: &SVNativeFrame) -> Result<(), SVError> {
        let token = self.token;
        CANVASES.with(|canvases| {
            let canvases = canvases.borrow();
            let (_, canvas) = canvases
                .iter()
                .find(|(registered, _)| *registered == token)
                .ok_or_else(|| sv_browser_error("the canvas handle is no longer registered"))?;
            canvas.put_frame(frame)
        })
    }
}

fn sv_browser_error(message: &str) -> SVError {
    SVError::BackendFailure {
        backend_kind: SVBackendKind::Wasm,
        operation: "show",
        message: message.to_string(),
    }
}
