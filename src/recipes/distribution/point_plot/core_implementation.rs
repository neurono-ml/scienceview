//! The built point plot and its render operation.

use std::path::Path;

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_observability::sv_run_operation;
use scienceview_core::sv_scale::sv_default_scale_palette;

use super::composition::sv_compose_geometries;
use super::configuration::SVPointPlotConfiguration;
use super::data_contract::{SVPointData, sv_resolve_point_data, sv_series_label};
use super::decoration::{sv_category_labels, sv_compose_decorations};
use super::output::SVPointRenderOutput;
use super::scales::{SVPointDomain, sv_build_scales, sv_plot_area};
use super::statistics::sv_summarize;

/// The point plot: the point estimate of each category and its uncertainty
/// (FDR-123).
///
/// Built only by [`SVPointPlotBuilder`](super::SVPointPlotBuilder), which
/// requires an explicit backend. The built value carries the backend kind, the
/// execution mode and the validated configuration; every operation records the
/// kind in its span.
pub struct SVPointPlot<'values> {
    configuration: SVPointPlotConfiguration<'values>,
    backend_kind: SVBackendKind,
    execution_mode: SVExecutionMode,
}

impl<'values> SVPointPlot<'values> {
    /// Transfers a validated configuration into a built plot.
    pub(crate) fn from_configuration(
        configuration: SVPointPlotConfiguration<'values>,
        backend_kind: SVBackendKind,
        execution_mode: SVExecutionMode,
    ) -> Self {
        Self {
            configuration,
            backend_kind,
            execution_mode,
        }
    }

    /// The configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// The configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// Renders the points, their error bars, the joining lines and the
    /// decorations in display space.
    ///
    /// The operation resolves the represented columns once, aggregates each
    /// category through the aggregate kernel, dodges the series through the
    /// move kernel, derives the domain, places every estimate through the
    /// scale kernel and asks the mark kernel for the geometry. Nothing is
    /// serialized yet: the result is backend-agnostic, so any backend can
    /// serialize the very same geometry. The geometry is transition-ready (a
    /// state change can interpolate it) but this change ships no animation
    /// driver.
    ///
    /// # Errors
    ///
    /// Returns the configuration and data errors of the composition and the
    /// kernel errors of the aggregate, the move, the scales and the marks.
    pub fn render(&self) -> Result<SVPointRenderOutput, SVError> {
        let data = sv_resolve_point_data(&self.configuration)?;
        let representation = data.representation;
        let number_of_points = data.number_of_points;
        sv_run_operation(
            "render",
            self.backend_kind,
            representation,
            number_of_points,
            || self.sv_render_resolved(&data),
        )
    }

    fn sv_render_resolved(&self, data: &SVPointData) -> Result<SVPointRenderOutput, SVError> {
        let summary = sv_summarize(data, &self.configuration)?;
        let domain = SVPointDomain::from_summary(&summary)?;
        let plot_area = sv_plot_area(self.configuration.viewport);
        let labels = sv_category_labels(&summary);
        let label_references: Vec<&str> = labels.iter().map(String::as_str).collect();
        let scales = sv_build_scales(
            &self.configuration,
            &summary,
            domain,
            &label_references,
            plot_area,
        )?;
        let geometries = sv_compose_geometries(&summary, &scales, &self.configuration, plot_area)?;
        let decorations = sv_compose_decorations(
            &self.configuration,
            &summary,
            &scales.nominal,
            &scales.value,
            plot_area,
            self.configuration.viewport,
        )?;
        let palette = sv_default_scale_palette();
        let series: Vec<(String, scienceview_core::sv_scale::SVColour)> = summary
            .series()
            .iter()
            .enumerate()
            .filter_map(|(index, series)| {
                series.map(|code| (sv_series_label(code), palette.colour_at_index(index)))
            })
            .collect();
        Ok(SVPointRenderOutput::new(
            self.backend_kind,
            data.representation,
            data.number_of_points,
            data.is_approximate,
            self.configuration.viewport,
            plot_area,
            self.configuration.title.map(str::to_string),
            summary,
            series,
            geometries,
            decorations,
        ))
    }

    /// Writes the rendered points through the configured backend.
    ///
    /// `Svg` writes the self-contained document. `Native` declares no export
    /// format, so asking it to save reports the missing capability instead of
    /// writing an unrelated file. Every other kind is not a serializer of this
    /// recipe and fails explicitly.
    ///
    /// # Errors
    ///
    /// Returns the render errors, [`SVError::BackendUnavailable`] when the SVG
    /// feature is not compiled, [`SVError::UnsupportedBackendCapability`] for
    /// `Native` and [`SVError::InvalidConfiguration`] for a kind without a
    /// point-plot serializer, plus the backend's own write error.
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
                    "the `{other}` backend has no point-plot serializer; select `Svg` or `Native`"
                ),
            }),
        }
    }
}

impl std::fmt::Debug for SVPointPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVPointPlot")
            .field("backend_kind", &self.backend_kind)
            .field("execution_mode", &self.execution_mode)
            .finish_non_exhaustive()
    }
}
