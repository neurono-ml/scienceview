//! The canvas seam: what receives the rasterizer's frame.

mod headless_canvas;
mod surface;

pub use headless_canvas::SVHeadlessCanvas;
pub use surface::SVCanvasSurface;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod browser_canvas;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use browser_canvas::SVBrowserCanvasHandle;

#[cfg(test)]
#[path = "canvas_tests.rs"]
mod canvas_tests;
