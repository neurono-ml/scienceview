//! The public rendering contract (PRD §4.2).
//!
//! Every backend — the ten workspace crates and any third-party crate — is a
//! value implementing [`SVRenderBackend`]. The core never names a backend
//! crate: it holds the trait object, so a new backend plugs in without
//! touching the core.

use super::capability::SVBackendCapability;
use super::kind::SVBackendKind;
use super::matrix::SVBackendCapabilities;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use std::path::Path;
use std::sync::Arc;

/// What a backend receives for a `render` call.
///
/// Borrowed on purpose: the backend reads the figure's values without copying
/// them, and the figure stays the owner.
#[derive(Debug, Clone, Copy)]
pub struct SVRenderRequest<'figure> {
    /// The configured backend kind.
    pub backend_kind: SVBackendKind,
    /// The execution mode the figure requested.
    pub execution_mode: SVExecutionMode,
    /// The figure's data, in the representation it arrived in.
    pub view: SVColumnView<'figure>,
    /// The optional figure title.
    pub title: Option<&'figure str>,
}

/// What a backend produces for a `render` or `update` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVRenderOutput {
    /// The backend that produced the bytes, as a self-description.
    pub backend_kind: SVBackendKind,
    /// The rendered document or frame.
    pub bytes: Vec<u8>,
}

/// What a backend receives for an `update` call.
#[derive(Debug, Clone, Copy)]
pub struct SVUpdateRequest<'figure> {
    /// The configured backend kind.
    pub backend_kind: SVBackendKind,
    /// The execution mode the figure requested.
    pub execution_mode: SVExecutionMode,
    /// The figure's data, in the representation it arrived in.
    pub view: SVColumnView<'figure>,
    /// How many points the update appended.
    pub appended_points: usize,
}

/// What a backend receives for a `save` call.
#[derive(Debug, Clone, Copy)]
pub struct SVSaveRequest<'figure> {
    /// The configured backend kind.
    pub backend_kind: SVBackendKind,
    /// The execution mode the figure requested.
    pub execution_mode: SVExecutionMode,
    /// The figure's data, in the representation it arrived in.
    pub view: SVColumnView<'figure>,
    /// Where the backend must write.
    pub destination: &'figure Path,
}

/// What a backend receives for a `show` call.
#[derive(Debug, Clone, Copy)]
pub struct SVShowRequest<'figure> {
    /// The configured backend kind.
    pub backend_kind: SVBackendKind,
    /// The execution mode the figure requested.
    pub execution_mode: SVExecutionMode,
    /// The figure's data, in the representation it arrived in.
    pub view: SVColumnView<'figure>,
}

/// The rendering contract every backend implements.
///
/// `render` is mandatory; the other operations have defaults that report the
/// missing capability as an error instead of doing nothing silently. A backend
/// that updates in place overrides `update`, and so on.
pub trait SVRenderBackend: Send + Sync {
    /// The kind this implementation renders.
    fn kind(&self) -> SVBackendKind;

    /// What this implementation declares it can do.
    fn capabilities(&self) -> SVBackendCapabilities;

    /// Renders the figure.
    fn render(&self, request: &SVRenderRequest<'_>) -> Result<SVRenderOutput, SVError>;

    /// Updates an already rendered figure in place.
    fn update(&self, _request: &SVUpdateRequest<'_>) -> Result<SVRenderOutput, SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.kind(),
            capability: SVBackendCapability::InPlaceUpdate,
        })
    }

    /// Writes the figure to `request.destination`.
    fn save(&self, _request: &SVSaveRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.kind(),
            capability: SVBackendCapability::Export,
        })
    }

    /// Presents the figure in a window or an embedded surface.
    fn show(&self, _request: &SVShowRequest<'_>) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.kind(),
            capability: SVBackendCapability::Presentation,
        })
    }
}

/// Supplies concrete backend implementations to the dispatch seam.
///
/// One provider may serve one kind or several. The registry asks each provider
/// in registration order and uses the first answer, so a test can register a
/// recording backend while a real backend keeps its own provider.
pub trait SVRenderBackendProvider: Send + Sync {
    /// Returns an implementation for `backend_kind`, or `None` if this provider
    /// does not serve that kind.
    fn provide(&self, backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>>;
}
