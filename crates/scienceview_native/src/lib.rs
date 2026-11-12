//! `scienceview_native`: from-scratch native window rendering backend for
//! scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
//!
//! ## Two layers
//!
//! - A **pure rasterizer** ([`SVRasterCanvas`]) owns an RGBA buffer and draws
//!   paths, rectangles, circles, text boxes and images with anti-aliasing. It
//!   needs no window, so every acceptance test compares pixels in CI.
//! - A **present layer** ([`SVNativeWindow`], behind the `window` feature)
//!   opens a winit window, blits the buffer with softbuffer, and re-solves the
//!   viewport on resize or device-pixel-ratio change.
//!
//! The frame loop takes its clock from the model through [`SVFrameClock`];
//! `scienceview_native` depends on no animation or transition type, so the
//! animation and transition crates plug in without this crate changing.
//!
//! ## Capabilities
//!
//! The backend declares events, scripting, streaming, in-place update and
//! presentation, and no export format: `render` and `update` return the frame,
//! `show` presents it, and `save` reports the missing capability.
//!
//! ## Rendering through the registry
//!
//! ```ignore
//! sv_register_native_backend();
//! let frame = figure.render()?;
//! ```
//!
//! A headless caller that wants the frame with its dimensions uses the
//! rasterizer surface directly:
//!
//! ```ignore
//! let frame = SVNativeBackend::new().render_frame(&request)?;
//! assert_eq!(frame.pixel(0, 0), Some(SVColour::new(250, 250, 250, 255)));
//! ```

mod backend;
mod canvas;
mod figure;

#[cfg(feature = "window")]
mod window;

pub use backend::SVFrameClock;
pub use backend::SVFrameLoop;
pub use backend::SVFrameOutcome;
pub use backend::SVNativeBackend;
pub use backend::SVNativeBackendProvider;
pub use backend::SVPausedClock;
pub use backend::sv_native_default_viewport;
pub use backend::sv_register_native_backend;
pub use canvas::SVDirtyRegion;
pub use canvas::SVNativeFrame;
pub use canvas::SVRasterBounds;
pub use canvas::SVRasterCanvas;

#[cfg(feature = "window")]
pub use window::SVNativeWindow;

#[cfg(test)]
#[path = "crate_tests/mod.rs"]
mod crate_tests;
