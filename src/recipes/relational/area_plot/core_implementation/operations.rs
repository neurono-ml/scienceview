//! The public operations: compose, render, save and show.

use std::path::Path;

use scienceview_core::sv_backend::{SVBackendCapability, SVRenderOutput};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_observability::sv_run_operation;

use super::super::preparation::sv_prepare_area;
use super::super::rendering::{SVAreaRendering, sv_compose_area};
use super::built_plot::SVAreaPlot;
use super::serialization::{sv_export, sv_serialize};

impl SVAreaPlot<'_> {
    /// Composes the figure into backend-agnostic primitives.
    ///
    /// This is the public surface a backend serializer consumes: the result
    /// carries the ordered primitives and needs no backend dispatch. The call
    /// emits the same `render` span as [`SVAreaPlot::render`], recording the
    /// configured backend kind, the representation and the number of points.
    ///
    /// # Errors
    ///
    /// Returns the data-boundary, move, scale, decoration or mark error that
    /// prevented the figure from being composed.
    pub fn rendering(&self) -> Result<SVAreaRendering, SVError> {
        let prepared = sv_prepare_area(&self.configuration)?;
        sv_run_operation(
            "render",
            self.backend_kind,
            prepared.representation,
            prepared.point_count,
            || sv_compose_area(&self.configuration, prepared),
        )
    }

    /// Renders the figure with the configured backend.
    ///
    /// The operation composes the scene once and hands it to the serializer
    /// selected by the configured kind: the SVG document or the native raster
    /// frame. The kind is never ignored, and a kind the recipe has no
    /// serializer for reports the missing capability instead of receiving
    /// bytes another backend produced.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::UnsupportedBackendCapability`]
    /// for a kind without a recipe serializer, and
    /// [`SVError::BackendUnavailable`] when the selected serializer's feature
    /// is not compiled.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let rendering = self.rendering()?;
        sv_serialize(self.backend_kind, &rendering)
    }

    /// Writes the figure to `destination`.
    ///
    /// Only the SVG backend exports a document. The native backend presents a
    /// frame instead, so it reports the missing export capability rather than
    /// writing raw pixels under a document extension.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::UnsupportedBackendCapability`]
    /// for a backend without an export format, and [`SVError::BackendFailure`]
    /// when the file cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let prepared = sv_prepare_area(&self.configuration)?;
        sv_run_operation(
            "save",
            self.backend_kind,
            prepared.representation,
            prepared.point_count,
            || {
                let rendering = sv_compose_area(&self.configuration, prepared)?;
                let bytes = sv_export(self.backend_kind, &rendering)?;
                std::fs::write(destination, bytes).map_err(|error| SVError::BackendFailure {
                    backend_kind: self.backend_kind,
                    operation: "save",
                    message: format!("cannot write {}: {error}", destination.display()),
                })
            },
        )
    }

    /// Reports that the recipe has no presentation surface.
    ///
    /// A native window is opened by the native backend's own present loop; the
    /// recipe exports documents and frames, it never owns a window.
    ///
    /// # Errors
    ///
    /// Always returns [`SVError::UnsupportedBackendCapability`].
    pub fn show(&self) -> Result<(), SVError> {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind: self.backend_kind,
            capability: SVBackendCapability::Presentation,
        })
    }
}
