//! The present layer: a winit window and a softbuffer surface.
//!
//! This is the only module that depends on a GUI stack, and it is compiled only
//! with the `window` feature. Everything it draws comes from the headless
//! [`SVRasterCanvas`](crate::SVRasterCanvas), so the window has no geometry
//! logic of its own.

use crate::canvas::SVNativeFrame;
use crate::present::SVPresenter;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_error::SVError;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::window::Window;

/// A window and its presented pixel surface.
pub(crate) struct SVPresentSurface {
    window: Arc<Window>,
    surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
}

impl SVPresentSurface {
    /// Wraps a window created by the active event loop.
    pub(crate) fn new(window: Arc<Window>) -> Result<Self, SVError> {
        let context = softbuffer::Context::new(Arc::clone(&window)).map_err(sv_window_failure)?;
        let surface =
            softbuffer::Surface::new(&context, Arc::clone(&window)).map_err(sv_window_failure)?;

        Ok(Self { window, surface })
    }

    /// Returns the window, so the event loop can request a redraw.
    pub(crate) fn window(&self) -> &Arc<Window> {
        &self.window
    }

    /// Blits one rendered frame into the window.
    pub(crate) fn blit(&mut self, frame: &SVNativeFrame) -> Result<(), SVError> {
        let Some(width) = NonZeroU32::new(frame.width() as u32) else {
            return Ok(());
        };
        let Some(height) = NonZeroU32::new(frame.height() as u32) else {
            return Ok(());
        };
        self.surface
            .resize(width, height)
            .map_err(sv_window_failure)?;
        let mut buffer = self.surface.buffer_mut().map_err(sv_window_failure)?;

        for (destination, source) in buffer.iter_mut().zip(frame.pixels().chunks_exact(4)) {
            *destination =
                (u32::from(source[0]) << 16) | (u32::from(source[1]) << 8) | u32::from(source[2]);
        }

        buffer.present().map_err(sv_window_failure)
    }
}

impl SVPresenter for SVPresentSurface {
    fn present(&mut self, frame: &SVNativeFrame) -> Result<(), SVError> {
        self.blit(frame)
    }
}

/// Maps a window-stack failure to the crate's single error type.
pub(crate) fn sv_window_failure(error: impl std::fmt::Display) -> SVError {
    SVError::BackendFailure {
        backend_kind: SVBackendKind::Native,
        operation: "show",
        message: format!("the native window failed: {error}"),
    }
}
