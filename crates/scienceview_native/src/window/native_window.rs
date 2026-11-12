//! The winit present layer and its event loop.
//!
//! The window never owns geometry: it asks a caller-supplied painter for a
//! frame, blits it, and re-solves the logical viewport on resize or scale
//! change. The model clock reaches the window through [`SVFrameLoop`], so a
//! paused timeline performs no animation work.

use super::native_application::SVNativeApplication;
use super::present_surface::sv_window_failure;
use crate::backend::SVFrameClock;
use crate::backend::SVFrameLoop;
use crate::backend::SVNativeBackend;
use crate::canvas::SVNativeFrame;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

/// A native window that presents the backend's frames.
pub struct SVNativeWindow {
    viewport: SVViewport,
}

impl SVNativeWindow {
    /// Describes the window's initial logical viewport.
    pub fn new(viewport: SVViewport) -> Self {
        Self { viewport }
    }

    /// Returns the initial logical viewport.
    pub const fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Opens the window and runs its event loop until it closes.
    ///
    /// `clock` supplies playback positions; `painter` turns a position into a
    /// frame. A failed paint closes the window and is returned.
    pub fn run<C, F>(&mut self, clock: C, painter: F) -> Result<(), SVError>
    where
        C: SVFrameClock + 'static,
        F: FnMut(f64) -> Result<SVNativeFrame, SVError>,
    {
        let event_loop = EventLoop::new().map_err(sv_window_failure)?;
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut application = SVNativeApplication {
            viewport: self.viewport,
            frame_loop: SVFrameLoop::new(clock),
            painter,
            position: 0.0,
            surface: None,
            failure: None,
        };
        event_loop
            .run_app(&mut application)
            .map_err(sv_window_failure)?;

        application.failure.map_or(Ok(()), Err)
    }

    /// Presents a static figure request in a window.
    pub fn show(&mut self, request: &SVRenderRequest<'_>) -> Result<(), SVError> {
        let request = *request;
        self.run(crate::backend::SVPausedClock, move |_position| {
            SVNativeBackend::new().render_frame(&request)
        })
    }
}

/// Presents a request in a new window, used by `SVRenderBackend::show`.
pub(crate) fn sv_present_native_request(request: &SVRenderRequest<'_>) -> Result<(), SVError> {
    SVNativeWindow::new(crate::backend::sv_native_default_viewport()).show(request)
}
