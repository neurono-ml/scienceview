//! The `SVRenderBackend` implementation for a Angular component source.
//!
//! The backend produces text, never a package: the host application compiles
//! the source with its own Angular toolchain. It declares the interactive row of
//! the capability matrix because the component forwards events, accepts
//! streaming snapshots and swaps its markup in place; it does not export a
//! file, so `save` reports the missing capability. `show` cannot mount a
//! component by itself and reports a typed backend failure instead.

use super::super::component::sv_render_component_source;
use scienceview_core::sv_backend::SVBackendCapabilities;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_error::SVError;

/// Renders figures as self-contained Angular component source.
///
/// The value carries no state: every operation derives its source from the
/// request, so one instance can serve concurrent renders.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVAngularBackend;

impl SVAngularBackend {
    /// Creates the backend.
    pub const fn new() -> Self {
        Self
    }

    fn render_source(
        &self,
        request: &SVRenderRequest<'_>,
        appended_points: usize,
    ) -> Result<SVRenderOutput, SVError> {
        let source = sv_render_component_source(request, appended_points)?;

        Ok(SVRenderOutput {
            backend_kind: request.backend_kind,
            bytes: source.into_bytes(),
        })
    }
}

impl SVRenderBackend for SVAngularBackend {
    fn kind(&self) -> SVBackendKind {
        SVBackendKind::Angular
    }

    fn capabilities(&self) -> SVBackendCapabilities {
        sv_backend_capabilities(SVBackendKind::Angular)
    }

    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.render_source(request, 0)
    }

    fn update(&self, request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        self.render_source(
            &SVRenderRequest {
                backend_kind: request.backend_kind,
                execution_mode: request.execution_mode,
                view: request.view,
                title: None,
            },
            request.appended_points,
        )
    }

    fn save(&self, _request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Angular,
            capability: SVBackendCapability::Export,
        })
    }

    fn show(&self, _request: &SVShowRequest<'_>) -> Result<(), SVError> {
        Err(SVError::BackendFailure {
            backend_kind: SVBackendKind::Angular,
            operation: "show",
            message: "a Angular component is mounted by its host application; \
                      call render and pass the source to the bundler"
                .to_string(),
        })
    }
}
