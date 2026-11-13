//! The built band plot and its render operation.

use std::path::Path;

use scienceview_core::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_mark::SVMarkKind;
use scienceview_core::sv_observability::sv_run_operation;
use scienceview_core::sv_scale::{SVColour, sv_default_scale_palette};

use super::composition::sv_compose_band_geometries;
use super::configuration::SVBandPlotConfiguration;
use super::data_contract::{SVBandData, sv_resolve_band_data};
use super::decoration::sv_compose_decorations;
use super::output::SVBandRenderOutput;
use super::scales::{
    sv_band_domain, sv_horizontal_axis_scale, sv_horizontal_placement_scale, sv_plot_area,
    sv_vertical_scale,
};

/// The band plot: the filled region between two boundary lines (FDR-105).
///
/// Built only by [`SVBandPlotBuilder`](super::SVBandPlotBuilder), which requires
/// an explicit backend. The built value carries the backend kind, the execution
/// mode and the validated configuration; every operation records the kind in
/// its span.
pub struct SVBandPlot<'values> {
    configuration: SVBandPlotConfiguration<'values>,
    backend_kind: SVBackendKind,
    execution_mode: SVExecutionMode,
}

impl<'values> SVBandPlot<'values> {
    /// Transfers a validated configuration into a built plot.
    pub(crate) fn from_configuration(
        configuration: SVBandPlotConfiguration<'values>,
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

    /// Renders the bands and their decorations in display space.
    ///
    /// The operation resolves the represented columns once, derives the
    /// domains, places every value through the scale kernel and asks the mark
    /// kernel for the paths. Nothing is serialized yet: the result is
    /// backend-agnostic, so any backend can serialize the very same geometry.
    ///
    /// # Errors
    ///
    /// Returns the configuration and data errors of the composition, and
    /// [`SVError::UnsupportedMark`] when the configured backend cannot host the
    /// ribbon mark.
    pub fn render(&self) -> Result<SVBandRenderOutput, SVError> {
        let mut data = sv_resolve_band_data(&self.configuration)?;
        let representation = data.representation;
        let number_of_points = data.number_of_points;
        sv_run_operation(
            "render",
            self.backend_kind,
            representation,
            number_of_points,
            || self.sv_render_resolved(&mut data),
        )
    }

    fn sv_render_resolved(&self, data: &mut SVBandData<'_>) -> Result<SVBandRenderOutput, SVError> {
        sv_verify_mark_supported(SVMarkKind::Ribbon, self.backend_kind)?;
        let domain = sv_band_domain(data)?;
        let plot_area = sv_plot_area(self.configuration.viewport);
        let horizontal = sv_horizontal_placement_scale(&self.configuration, &domain, plot_area)?;
        let axis = sv_horizontal_axis_scale(&self.configuration, &domain, plot_area)?;
        let vertical = sv_vertical_scale(&domain, plot_area)?;
        let geometries = sv_compose_band_geometries(
            data,
            &self.configuration,
            &horizontal,
            &vertical,
            plot_area,
        )?;
        let series = sv_series_entries(data);
        let decorations = sv_compose_decorations(
            &self.configuration,
            &axis,
            &vertical,
            plot_area,
            self.configuration.viewport,
            &series,
        )?;
        Ok(SVBandRenderOutput::new(
            self.backend_kind,
            data.representation,
            data.number_of_points,
            data.is_approximate,
            self.configuration.viewport,
            plot_area,
            self.configuration.title.map(str::to_string),
            geometries,
            decorations,
        ))
    }

    /// Serializes the rendered bands as one self-contained SVG document.
    ///
    /// # Errors
    ///
    /// Returns the render errors, [`SVError::InvalidConfiguration`] when the
    /// configured backend is not `Svg`, and the backend's own write error.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
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
}

fn sv_series_entries(data: &SVBandData<'_>) -> Vec<(String, SVColour)> {
    let palette = sv_default_scale_palette();
    data.bands
        .iter()
        .enumerate()
        .filter_map(|(index, band)| {
            band.name
                .as_ref()
                .map(|name| (name.clone(), palette.colour_at_index(index)))
        })
        .collect()
}
