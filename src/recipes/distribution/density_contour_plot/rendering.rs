//! The backend-aware operations of [`SVDensityContourPlot`](super::SVDensityContourPlot).

use super::SVDensityContourPlot;
use super::composition::{SVDensityContourComposition, sv_compose};
use super::core_implementation::sv_record_drawn_marks;
use crate::sv_backend::{SVBackendCapability, SVBackendKind, SVRenderOutput};
use crate::sv_error::SVError;
use crate::sv_observability::sv_run_operation;
use std::path::Path;

impl SVDensityContourPlot<'_> {
    /// Renders through the path the configured kind actually has.
    ///
    /// `Svg` serializes the SVG document, `Native` rasterizes the native frame,
    /// and every other kind reports [`SVError::UnsupportedBackendCapability`]
    /// because the recipe has no serializer for it yet. A compiled-out backend
    /// reports [`SVError::BackendUnavailable`] instead of degrading silently.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_points,
            || {
                let composition = sv_compose(configuration)?;
                sv_record_drawn_marks(&composition);
                sv_dispatch_render(&composition)
            },
        )
    }

    /// Writes the rendered figure to `destination`.
    ///
    /// Only the SVG backend exports a document; the native backend is a frame
    /// renderer without a file format, so it reports the missing capability.
    ///
    /// # Errors
    ///
    /// Returns the error of [`SVDensityContourPlot::render`] and
    /// [`SVError::BackendFailure`] when the destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let backend_kind = self.configuration.backend_kind;
        if backend_kind != SVBackendKind::Svg {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind,
                capability: SVBackendCapability::Export,
            });
        }
        let output = self.render()?;
        std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
            backend_kind,
            operation: "save",
            message: format!("cannot write {}: {error}", destination.display()),
        })
    }
}

fn sv_dispatch_render(
    composition: &SVDensityContourComposition,
) -> Result<SVRenderOutput, SVError> {
    match composition.backend_kind() {
        SVBackendKind::Svg => sv_render_svg(composition),
        SVBackendKind::Native => sv_render_native(composition),
        backend_kind => Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_render_svg(composition: &SVDensityContourComposition) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind: SVBackendKind::Svg,
        bytes: composition.to_svg()?.into_bytes(),
    })
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(composition: &SVDensityContourComposition) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: composition.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(composition: &SVDensityContourComposition) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind: SVBackendKind::Native,
        bytes: composition.to_native_frame()?.into_pixels(),
    })
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(composition: &SVDensityContourComposition) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: composition.backend_kind(),
    })
}
