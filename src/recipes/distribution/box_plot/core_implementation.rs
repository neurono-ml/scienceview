//! The built box plot and its render operation.

use std::path::Path;

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_observability::sv_run_operation;
use scienceview_core::sv_scale::sv_default_scale_palette;

use super::composition::sv_compose_box_geometries;
use super::configuration::SVBoxPlotConfiguration;
use super::data_contract::{SVBoxData, sv_resolve_box_data, sv_series_label};
use super::decoration::{sv_category_labels, sv_compose_decorations};
use super::output::SVBoxRenderOutput;
use super::scales::{SVBoxDomain, sv_build_scales, sv_plot_area};
use super::statistics::sv_summarize;

/// The box plot: the five-number summary of each category (FDR-116).
///
/// Built only by [`SVBoxPlotBuilder`](super::SVBoxPlotBuilder), which requires
/// an explicit backend. The built value carries the backend kind, the execution
/// mode and the validated configuration; every operation records the kind in
/// its span.
pub struct SVBoxPlot<'values> {
    configuration: SVBoxPlotConfiguration<'values>,
    backend_kind: SVBackendKind,
    execution_mode: SVExecutionMode,
}

impl<'values> SVBoxPlot<'values> {
    /// Transfers a validated configuration into a built plot.
    pub(crate) fn from_configuration(
        configuration: SVBoxPlotConfiguration<'values>,
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

    /// Renders the boxes and their decorations in display space.
    ///
    /// The operation resolves the represented columns once, summarises each
    /// group through the percentile kernel, derives the domains, places every
    /// value through the scale kernel and asks the mark kernel for the paths.
    /// Nothing is serialized yet: the result is backend-agnostic, so any backend
    /// can serialize the very same geometry.
    ///
    /// # Errors
    ///
    /// Returns the configuration and data errors of the composition and the
    /// kernel errors of the statistics, the dodge, the scales and the marks.
    pub fn render(&self) -> Result<SVBoxRenderOutput, SVError> {
        let data = sv_resolve_box_data(&self.configuration)?;
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

    fn sv_render_resolved(&self, data: &SVBoxData) -> Result<SVBoxRenderOutput, SVError> {
        let summaries = sv_summarize(data, &self.configuration)?;
        let domain = SVBoxDomain::from_data(data)?;
        let plot_area = sv_plot_area(self.configuration.viewport);
        let labels = sv_category_labels(&summaries.grouping);
        let label_references: Vec<&str> = labels.iter().map(String::as_str).collect();
        let scales = sv_build_scales(
            &self.configuration,
            &summaries.grouping,
            domain,
            &label_references,
            plot_area,
        )?;
        let geometries =
            sv_compose_box_geometries(&summaries, &scales, &self.configuration, plot_area)?;
        let decorations = sv_compose_decorations(
            &self.configuration,
            &summaries.grouping,
            &scales.nominal,
            &scales.value,
            plot_area,
            self.configuration.viewport,
        )?;
        let number_of_outliers = if self.configuration.show_outliers {
            summaries
                .summaries
                .iter()
                .map(|summary| summary.outliers().len())
                .sum()
        } else {
            0
        };
        let palette = sv_default_scale_palette();
        let series: Vec<(String, scienceview_core::sv_scale::SVColour)> = summaries
            .grouping
            .series
            .iter()
            .enumerate()
            .filter_map(|(index, series)| {
                series.map(|code| (sv_series_label(code), palette.colour_at_index(index)))
            })
            .collect();
        Ok(SVBoxRenderOutput::new(
            self.backend_kind,
            data.representation,
            data.number_of_points,
            number_of_outliers,
            data.is_approximate,
            self.configuration.viewport,
            plot_area,
            self.configuration.title.map(str::to_string),
            summaries.summaries,
            series,
            geometries,
            decorations,
        ))
    }

    /// Writes the rendered boxes through the configured backend.
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
    /// box-plot serializer, plus the backend's own write error.
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
                    "the `{other}` backend has no box-plot serializer; select `Svg` or `Native`"
                ),
            }),
        }
    }
}

impl std::fmt::Debug for SVBoxPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVBoxPlot")
            .field("backend_kind", &self.backend_kind)
            .field("execution_mode", &self.execution_mode)
            .finish_non_exhaustive()
    }
}
