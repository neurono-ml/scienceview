//! `scienceview_wasm`: WebAssembly canvas rendering backend for scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
//!
//! ## One renderer
//!
//! The browser does not get a second renderer. The crate depends on
//! [`scienceview_native`] with its `window` layer switched off and reuses
//! [`SVNativeBackend`](scienceview_native::SVNativeBackend) as its rasterizer:
//! the same anti-aliasing, text metrics, blending and layout rounding produce
//! the same frame on the desktop and in the browser. Extraction into a third
//! crate was rejected because it would have moved native's ownership surface
//! for no pixel gain; the dependency is the smaller contract.
//!
//! ## A canvas seam, not a browser dependency
//!
//! Rendering is headless on every target. [`SVCanvasSurface`] is the seam that
//! receives a finished frame; [`SVHeadlessCanvas`] captures it on the host, and
//! the `web-sys` canvas implements the same trait under
//! `cfg(target_arch = "wasm32")`. A host build therefore compiles the same API
//! and reports a typed [`SVError::BackendUnavailable`] only when presentation
//! is requested without a surface.
//!
//! ## The clock belongs to the model
//!
//! `requestAnimationFrame` drives redraws; its timestamp is handed to the model
//! clock, which returns the playback position to draw. A paused model answers
//! `None` and no animation work is performed. [`SVWasmFrameLoop`] is the seam,
//! and the browser driver is the only code that touches animation frames.
//!
//! ## Minimal bindings
//!
//! The JavaScript surface is construction, update and render. The figure model
//! stays in Rust; JavaScript pushes data and the model produces the output.

mod backend;
mod canvas;
mod frame;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod bindings;

pub use backend::SVWasmBackend;
pub use backend::SVWasmBackendProvider;
pub use backend::sv_register_wasm_backend;
pub use canvas::SVCanvasSurface;
pub use canvas::SVHeadlessCanvas;
pub use frame::SVPausedWasmClock;
pub use frame::SVWasmFrameLoop;
pub use frame::SVWasmFrameOutcome;
pub use frame::SVWasmModelClock;
pub use scienceview_native::SVNativeFrame;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use bindings::SVWasmFigure;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use canvas::SVBrowserCanvasHandle;
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use frame::SVBrowserFrameDriver;

#[cfg(test)]
#[path = "crate_tests/mod.rs"]
mod crate_tests;
