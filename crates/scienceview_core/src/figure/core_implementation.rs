//! The built figure: the runtime value every operation acts on.
//!
//! The figure is not generic over the backend (design D1): it stores the
//! configured kind and resolves the concrete backend on each operation, so the
//! same type serves every backend and a dashboard can hold figures for
//! different targets in one collection.

use super::SVDataBinding;
use super::SVExecutionMode;
use super::representation::SVDataRepresentation;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::SVBackendRegistry;
use crate::sv_backend::SVRenderOutput;
use crate::sv_backend::SVRenderRequest;
use crate::sv_backend::SVSaveRequest;
use crate::sv_backend::SVShowRequest;
use crate::sv_backend::SVUpdateRequest;
use crate::sv_backend::sv_resolve_render_backend;
use crate::sv_error::SVError;
use crate::sv_observability::sv_run_operation;
use std::path::Path;
use std::sync::Arc;

/// A figure ready to be rendered.
///
/// Built only by [`SVFigureBuilder`](super::SVFigureBuilder): the direct
/// constructor is private, so the "backend is explicit" rule cannot be
/// bypassed.
pub struct SVFigure<'values> {
    backend_kind: SVBackendKind,
    execution_mode: SVExecutionMode,
    data: SVDataBinding<'values>,
    title: Option<&'values str>,
    registry: Arc<SVBackendRegistry>,
}

impl<'values> SVFigure<'values> {
    /// Transfers a validated configuration into a figure.
    ///
    /// Performs no backend dispatch: the kind is only recorded, so `build`
    /// cannot perform rendering work (spec `backend-selection`).
    pub(super) fn from_configuration(
        backend_kind: SVBackendKind,
        execution_mode: SVExecutionMode,
        data: SVDataBinding<'values>,
        title: Option<&'values str>,
        registry: Arc<SVBackendRegistry>,
    ) -> Self {
        Self {
            backend_kind,
            execution_mode,
            data,
            title,
            registry,
        }
    }

    /// The configured backend kind, as a runtime value.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Changes the backend kind used by subsequent operations.
    pub fn set_backend_kind(&mut self, backend_kind: SVBackendKind) {
        self.backend_kind = backend_kind;
    }

    /// The execution mode the builder selected.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// The representation the data arrived in.
    pub fn representation(&self) -> SVDataRepresentation {
        self.data.representation()
    }

    /// The logical number of marks the figure draws.
    pub fn point_count(&self) -> usize {
        self.data.point_count()
    }

    /// The values, borrowed from the caller.
    pub fn values(&self) -> &'values [f64] {
        self.data.values()
    }

    /// Renders the figure with the configured backend.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        sv_run_operation(
            "render",
            self.backend_kind,
            self.data.representation(),
            self.data.point_count(),
            || {
                let backend = sv_resolve_render_backend(&self.registry, self.backend_kind)?;
                backend.render(&SVRenderRequest {
                    backend_kind: self.backend_kind,
                    execution_mode: self.execution_mode,
                    representation: self.data.representation(),
                    values: self.data.values(),
                    title: self.title,
                })
            },
        )
    }

    /// Applies `appended_points` new points to an already rendered figure.
    pub fn update(&self, appended_points: usize) -> Result<SVRenderOutput, SVError> {
        sv_run_operation(
            "update",
            self.backend_kind,
            self.data.representation(),
            self.data.point_count(),
            || {
                let backend = sv_resolve_render_backend(&self.registry, self.backend_kind)?;
                backend.update(&SVUpdateRequest {
                    backend_kind: self.backend_kind,
                    execution_mode: self.execution_mode,
                    representation: self.data.representation(),
                    values: self.data.values(),
                    appended_points,
                })
            },
        )
    }

    /// Writes the figure to `destination`.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        sv_run_operation(
            "save",
            self.backend_kind,
            self.data.representation(),
            self.data.point_count(),
            || {
                let backend = sv_resolve_render_backend(&self.registry, self.backend_kind)?;
                backend.save(&SVSaveRequest {
                    backend_kind: self.backend_kind,
                    execution_mode: self.execution_mode,
                    representation: self.data.representation(),
                    values: self.data.values(),
                    destination,
                })
            },
        )
    }

    /// Presents the figure in a window or an embedded surface.
    pub fn show(&self) -> Result<(), SVError> {
        sv_run_operation(
            "show",
            self.backend_kind,
            self.data.representation(),
            self.data.point_count(),
            || {
                let backend = sv_resolve_render_backend(&self.registry, self.backend_kind)?;
                backend.show(&SVShowRequest {
                    backend_kind: self.backend_kind,
                    execution_mode: self.execution_mode,
                    representation: self.data.representation(),
                    values: self.data.values(),
                })
            },
        )
    }
}
