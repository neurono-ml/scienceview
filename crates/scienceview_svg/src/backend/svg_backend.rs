//! The `SVRenderBackend` implementation for a static SVG document.
//!
//! The backend is export-only, exactly as the capability matrix declares: it
//! renders bytes and writes them to a file, and it reports the missing
//! capability for `update` and `show` instead of pretending to succeed.

use crate::figure::sv_render_default_figure;
use scienceview_core::sv_backend::SVBackendCapabilities;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_error::SVError;
use std::fs;

/// Renders figures as self-contained SVG documents.
///
/// The value carries no state: every operation derives its output from the
/// request, so one instance can serve concurrent renders.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVSvgBackend;

impl SVSvgBackend {
    /// Creates the backend.
    pub const fn new() -> Self {
        Self
    }
}

impl SVRenderBackend for SVSvgBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Svg
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        sv_backend_capabilities(SVBackendKind::Svg)
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        let document = sv_render_default_figure(request)?;

        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: document.into_bytes(),
        })
    }

    fn save(&self, request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        let document = sv_render_default_figure(&SVRenderRequest {
            backend_kind: request.backend_kind,
            execution_mode: request.execution_mode,
            view: request.view,
            title: None,
        })?;

        fs::write(request.destination, document).map_err(|error| SVError::BackendFailure {
            backend_kind: SVBackendKind::Svg,
            operation: "save",
            message: format!("cannot write {}: {error}", request.destination.display()),
        })
    }
}
