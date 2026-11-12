//! The `SVRenderBackend` implementation: a from-scratch CPU renderer.
//!
//! Rendering is headless on purpose. `render` and `update` rasterize into a
//! buffer and return it, so every acceptance test compares pixels without a
//! display; `show` presents that buffer in a window when the `window` feature is
//! compiled. The backend declares presentation, events, scripting, streaming
//! and in-place update, and no export format — `save` reports the missing
//! capability instead of writing an unrelated file.

use crate::canvas::SVNativeFrame;
use crate::figure::sv_rasterize_default_figure;
use scienceview_core::sv_backend::SVBackendCapabilities;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;

/// The logical width of the frame a bare figure renders at.
const NATIVE_DEFAULT_WIDTH: f64 = 640.0;

/// The logical height of the frame a bare figure renders at.
const NATIVE_DEFAULT_HEIGHT: f64 = 400.0;

/// Returns the logical viewport a bare figure renders at.
pub fn sv_native_default_viewport() -> SVViewport {
    SVViewport::new(NATIVE_DEFAULT_WIDTH, NATIVE_DEFAULT_HEIGHT)
}

/// Renders figures as CPU pixels in a native window.
///
/// The value carries no state: every operation derives its frame from the
/// request, so one instance can serve concurrent renders.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVNativeBackend;

impl SVNativeBackend {
    /// Creates the backend.
    pub const fn new() -> Self {
        Self
    }

    /// Rasterizes a request at the default viewport and returns the frame.
    ///
    /// This is the comparable-frame surface of the backend (design D3).
    pub fn render_frame(&self, request: &SVRenderRequest<'_>) -> Result<SVNativeFrame, SVError> {
        self.render_frame_with_viewport(request, &sv_native_default_viewport())
    }

    /// Rasterizes a request at an explicit viewport and returns the frame.
    ///
    /// A resized window calls this with its new logical viewport, which is what
    /// makes the layout re-solve before the frame is redrawn.
    pub fn render_frame_with_viewport(
        &self,
        request: &SVRenderRequest<'_>,
        viewport: &SVViewport,
    ) -> Result<SVNativeFrame, SVError> {
        sv_rasterize_default_figure(request, viewport)
    }
}

impl SVRenderBackend for SVNativeBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Native
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        sv_backend_capabilities(SVBackendKind::Native)
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let frame = self.render_frame(request)?;
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: frame.into_pixels(),
        })
    }

    fn update(&self, request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let frame = self.render_frame(&SVRenderRequest {
            backend_kind: request.backend_kind,
            execution_mode: request.execution_mode,
            view: request.view,
            title: None,
        })?;
        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: frame.into_pixels(),
        })
    }

    fn save(&self, _request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: scienceview_core::sv_backend::SVBackendCapability::Export,
        })
    }

    fn show(&self, request: &SVShowRequest<'_>) -> Result<(), SVError> {
        let render_request = SVRenderRequest {
            backend_kind: request.backend_kind,
            execution_mode: request.execution_mode,
            view: request.view,
            title: None,
        };
        sv_show_native_request(&render_request)
    }
}

#[cfg(feature = "window")]
fn sv_show_native_request(request: &SVRenderRequest<'_>) -> Result<(), SVError> {
    crate::window::sv_present_native_request(request)
}

#[cfg(not(feature = "window"))]
fn sv_show_native_request(_request: &SVRenderRequest<'_>) -> Result<(), SVError> {
    Err(SVError::BackendFailure {
        backend_kind: SVBackendKind::Native,
        operation: "show",
        message: "the window layer is not compiled; enable the `window` feature of \
                  `scienceview_native` (the root `render-native` feature enables it)"
            .to_string(),
    })
}
