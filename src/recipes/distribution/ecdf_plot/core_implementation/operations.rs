//! The public operations and their observability spans.

use super::super::composition::SVEcdfPlotComposition;
use super::super::data_contract::sv_resolve_ecdf_data;
use super::definition::SVECdfPlot;
use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVMarkKind;
use scienceview_core::sv_observability::sv_run_operation;
use std::path::Path;

impl SVECdfPlot<'_> {
    /// Renders the chart in display space through the composed geometry.
    ///
    /// The operation resolves the source once, records the representation and
    /// the observation count in its span and returns the backend-neutral
    /// composition. Serializing it is a separate, explicit step: `Svg` writes
    /// the self-contained document, `Native` rasterizes the same primitives.
    ///
    /// # Errors
    ///
    /// Returns the data and composition errors, and [`SVError::UnsupportedMark`]
    /// when the configured backend cannot host the line mark.
    pub fn render(&self) -> Result<SVEcdfPlotComposition, SVError> {
        let data = sv_resolve_ecdf_data(self)?;
        sv_run_operation(
            "render",
            self.backend_kind,
            data.representation,
            data.point_count,
            || {
                sv_verify_mark_supported(SVMarkKind::Line, self.backend_kind)?;
                self.sv_compose_data(&data)
            },
        )
    }

    /// Re-renders the current window after `appended_points` new observations.
    ///
    /// The recipe owns no incremental buffer: the operation resolves the source
    /// again, so the curve moves to the advanced window. A backend that can
    /// append without recomposing the history consumes the count; the
    /// composition is rebuilt for the current window.
    ///
    /// # Errors
    ///
    /// Same as [`SVECdfPlot::render`].
    pub fn update(&self, appended_points: usize) -> Result<SVEcdfPlotComposition, SVError> {
        let data = sv_resolve_ecdf_data(self)?;
        sv_run_operation(
            "update",
            self.backend_kind,
            data.representation,
            data.point_count,
            || {
                let _ = appended_points;
                sv_verify_mark_supported(SVMarkKind::Line, self.backend_kind)?;
                self.sv_compose_data(&data)
            },
        )
    }

    /// Writes the rendered chart to `destination` through the configured
    /// backend.
    ///
    /// `Svg` writes the self-contained document. `Native` declares no export
    /// format, so asking it to save reports the missing capability instead of
    /// writing an unrelated file. Every other kind is not a serializer of this
    /// recipe and fails explicitly.
    ///
    /// # Errors
    ///
    /// Returns the render errors, [`SVError::UnsupportedBackendCapability`] for
    /// `Native`, [`SVError::InvalidConfiguration`] for a kind without an ECDF
    /// serializer, and the backend's own write error.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        match self.backend_kind {
            SVBackendKind::Svg => {
                #[cfg(feature = "render-svg")]
                {
                    let document = self.render()?.to_svg()?;
                    std::fs::write(destination, document).map_err(|error| SVError::BackendFailure {
                        backend_kind: self.backend_kind,
                        operation: "save",
                        message: error.to_string(),
                    })
                }
                #[cfg(not(feature = "render-svg"))]
                {
                    let _ = destination;
                    Err(SVError::BackendUnavailable {
                        backend_kind: self.backend_kind,
                    })
                }
            }
            SVBackendKind::Native => Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: SVBackendCapability::Export,
            }),
            other => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the `{other}` backend has no ECDF serializer; select `Svg` or `Native`"
                ),
            }),
        }
    }
}
