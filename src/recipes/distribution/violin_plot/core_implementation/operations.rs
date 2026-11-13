//! The public operations: render, save and show.

use std::path::Path;

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_observability::sv_run_operation;
use scienceview_core::sv_scale::sv_default_scale_palette;

use super::super::composition::sv_compose_violin_geometries;
use super::super::data_contract::{SVViolinData, sv_resolve_violin_data, sv_series_label};
use super::super::decoration::{sv_compose_violin_decorations, sv_violin_category_labels};
use super::super::output::SVViolinRenderOutput;
use super::super::scales::{SVViolinDomain, sv_build_scales, sv_plot_area};
use super::super::statistics::sv_summarize_violins;
use super::built_plot::SVViolinPlot;

impl SVViolinPlot<'_> {
    /// Renders the violins and their decorations in display space.
    ///
    /// # Errors
    ///
    /// Returns the configuration and data errors of the composition and the
    /// kernel errors of the density, percentiles, dodge, scales and marks.
    pub fn render(&self) -> Result<SVViolinRenderOutput, SVError> {
        let data = sv_resolve_violin_data(&self.configuration)?;
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

    fn sv_render_resolved(&self, data: &SVViolinData) -> Result<SVViolinRenderOutput, SVError> {
        let summaries = sv_summarize_violins(data, &self.configuration)?;
        let domain = SVViolinDomain::from_summaries(&summaries.summaries)?;
        let plot_area = sv_plot_area(self.configuration.viewport);
        let labels = sv_violin_category_labels(&summaries.grouping);
        let label_references: Vec<&str> = labels.iter().map(String::as_str).collect();
        let scales = sv_build_scales(
            &self.configuration,
            summaries.grouping.categories.len(),
            domain,
            &label_references,
            plot_area,
        )?;
        let geometries =
            sv_compose_violin_geometries(&summaries, &scales, &self.configuration, plot_area)?;
        let decorations = sv_compose_violin_decorations(
            &self.configuration,
            &summaries.grouping,
            &scales.nominal,
            &scales.value,
            plot_area,
            self.configuration.viewport,
        )?;
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
        Ok(SVViolinRenderOutput::new(
            self.backend_kind,
            data.representation,
            data.number_of_points,
            summaries.grouping.categories.len(),
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

    /// Writes the rendered violins through the configured backend.
    ///
    /// # Errors
    ///
    /// Returns the render errors, [`SVError::BackendUnavailable`] when the SVG
    /// feature is not compiled, [`SVError::UnsupportedBackendCapability`] for
    /// `Native`, and the backend's own write error.
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
                    "the `{other}` backend has no violin-plot serializer; select `Svg` or `Native`"
                ),
            }),
        }
    }

    /// Reports that the recipe has no presentation surface.
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
