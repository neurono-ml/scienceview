//! A headless surface: the acceptance canvas of the host target.
//!
//! The canvas owns the last frame it accepted, which is what lets a test
//! compare a WebAssembly frame against the native frame without a browser. It
//! is also the proof that presentation is a blit and not a second render.

use super::surface::SVCanvasSurface;
use crate::SVNativeFrame;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// A canvas that keeps the frames blitted into it in memory.
pub struct SVHeadlessCanvas {
    viewport: SVViewport,
    frame: Mutex<Option<SVNativeFrame>>,
    presented_frames: AtomicUsize,
}

impl SVHeadlessCanvas {
    /// Creates a canvas sized from a viewport.
    pub fn new(viewport: SVViewport) -> Self {
        Self {
            viewport,
            frame: Mutex::new(None),
            presented_frames: AtomicUsize::new(0),
        }
    }

    /// Returns the viewport the canvas was sized for.
    pub const fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the last frame presented, if any.
    pub fn frame(&self) -> Option<SVNativeFrame> {
        self.frame
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Returns how many frames were presented.
    pub fn presented_frames(&self) -> usize {
        self.presented_frames.load(Ordering::Relaxed)
    }
}

impl SVCanvasSurface for SVHeadlessCanvas {
    fn viewport(&self) -> SVViewport {
        self.viewport
    }

    fn blit(&self, frame: &SVNativeFrame) -> Result<(), SVError> {
        let expected_width = self.viewport.physical_width().round() as usize;
        let expected_height = self.viewport.physical_height().round() as usize;

        if frame.width() != expected_width || frame.height() != expected_height {
            return Err(SVError::BackendFailure {
                backend_kind: SVBackendKind::Wasm,
                operation: "show",
                message: format!(
                    "the canvas is {expected_width}x{expected_height} device pixels but the frame is {}x{}",
                    frame.width(),
                    frame.height()
                ),
            });
        }

        let mut stored = self
            .frame
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *stored = Some(frame.clone());
        self.presented_frames.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
