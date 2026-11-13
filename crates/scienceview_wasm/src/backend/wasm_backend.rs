//! The `SVRenderBackend` implementation: the native CPU rasterizer, presented
//! on a canvas.
//!
//! Rendering is headless on every target: `render` and `update` rasterize with
//! [`SVNativeBackend`] and return the RGBA frame, so a host build and a test
//! compare pixels without a browser. `show` solves the layout for the surface's
//! viewport and blits the finished frame; without a surface it reports a typed
//! [`SVError::BackendUnavailable`] instead of pretending to present.

use crate::SVNativeFrame;
use crate::canvas::SVCanvasSurface;
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
use scienceview_native::SVNativeBackend;
use scienceview_native::sv_native_default_viewport;
use std::sync::Arc;

/// Renders figures through the shared CPU rasterizer onto a canvas surface.
///
/// Render and update carry no surface state, so one instance serves concurrent
/// renders. Presentation is the only operation that needs a surface.
#[derive(Clone, Default)]
pub struct SVWasmBackend {
    surface: Option<Arc<dyn SVCanvasSurface>>,
}

impl SVWasmBackend {
    /// Creates a backend without a presentation surface.
    pub const fn new() -> Self {
        Self { surface: None }
    }

    /// Attaches the surface `show` blits into.
    pub fn with_surface(mut self, surface: Arc<dyn SVCanvasSurface>) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Returns the attached surface, if any.
    pub fn surface(&self) -> Option<&Arc<dyn SVCanvasSurface>> {
        self.surface.as_ref()
    }

    /// Rasterizes a request at the default viewport and returns the frame.
    pub fn render_frame(&self, request: &SVRenderRequest<'_>) -> Result<SVNativeFrame, SVError> {
        self.render_frame_with_viewport(request, &sv_native_default_viewport())
    }

    /// Rasterizes a request at an explicit viewport and returns the frame.
    pub fn render_frame_with_viewport(
        &self,
        request: &SVRenderRequest<'_>,
        viewport: &SVViewport,
    ) -> Result<SVNativeFrame, SVError> {
        SVNativeBackend::new().render_frame_with_viewport(request, viewport)
    }
}

impl core::fmt::Debug for SVWasmBackend {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SVWasmBackend")
            .field("has_surface", &self.surface.is_some())
            .finish()
    }
}

impl SVRenderBackend for SVWasmBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Wasm
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        sv_backend_capabilities(SVBackendKind::Wasm)
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
            backend_kind: SVBackendKind::Wasm,
            capability: scienceview_core::sv_backend::SVBackendCapability::Export,
        })
    }

    fn show(&self, request: &SVShowRequest<'_>) -> Result<(), SVError> {
        let surface = self.surface.as_ref().ok_or(SVError::BackendUnavailable {
            backend_kind: SVBackendKind::Wasm,
        })?;
        let render_request = SVRenderRequest {
            backend_kind: request.backend_kind,
            execution_mode: request.execution_mode,
            view: request.view,
            title: None,
        };
        let frame = self.render_frame_with_viewport(&render_request, &surface.viewport())?;
        surface.blit(&frame)
    }
}
