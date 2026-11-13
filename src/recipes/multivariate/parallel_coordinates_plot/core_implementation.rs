//! The built parallel-coordinates plot and its render operation.

use super::builder::SVParallelCoordinatesPlotBuilder;
use super::composition::sv_compose_geometries;
use super::configuration::SVParallelCoordinatesConfiguration;
use super::decorations::sv_build_decorations;
use super::options::{SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization};
use super::output::SVParallelCoordinatesRenderOutput;
use super::preparation::{SVParallelCoordinatesPrepared, sv_prepare};
use super::rendering::sv_dispatch_render;
use crate::sv_backend::{SVBackendCapability, SVBackendKind, SVRenderOutput};
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_observability::sv_run_operation;
use std::path::Path;

/// Many observations drawn as polylines through parallel axes.
///
/// Built only by [`SVParallelCoordinatesPlotBuilder`], which requires an
/// explicit backend. Every operation resolves the source once and records the
/// configured backend kind in its span.
#[derive(Clone)]
pub struct SVParallelCoordinatesPlot<'values> {
    configuration: SVParallelCoordinatesConfiguration<'values>,
}

impl std::fmt::Debug for SVParallelCoordinatesPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVParallelCoordinatesPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("observations", &self.configuration.number_of_observations)
            .finish_non_exhaustive()
    }
}

impl<'values> SVParallelCoordinatesPlot<'values> {
    /// Starts building a parallel-coordinates plot.
    pub fn builder() -> SVParallelCoordinatesPlotBuilder<'values> {
        SVParallelCoordinatesPlotBuilder::new()
    }

    pub(super) fn from_configuration(
        configuration: SVParallelCoordinatesConfiguration<'values>,
    ) -> Self {
        Self { configuration }
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the dominant representation of the bound columns.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of logical observations.
    pub fn number_of_observations(&self) -> usize {
        self.configuration.number_of_observations
    }

    /// Returns the configured line width.
    pub fn line_width(&self) -> SVLength {
        self.configuration.line_width
    }

    /// Returns the configured line opacity.
    pub fn opacity(&self) -> f32 {
        self.configuration.opacity
    }

    /// Returns the configured normalisation rule.
    pub fn normalization(&self) -> SVParallelCoordinatesNormalization {
        self.configuration.normalization
    }

    /// Returns the configured missing-value policy.
    pub fn missing_policy(&self) -> SVParallelCoordinatesMissingPolicy {
        self.configuration.missing
    }

    /// Composes the backend-agnostic scene.
    ///
    /// # Errors
    ///
    /// Returns the data, scale, mark or missing-value error of the composition.
    pub fn compose(&self) -> Result<SVParallelCoordinatesRenderOutput, SVError> {
        let prepared = sv_prepare(&self.configuration)?;
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            prepared.number_of_vertices,
            || sv_build_output(configuration, prepared),
        )
    }

    /// Renders through the path the configured kind actually has.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let prepared = sv_prepare(&self.configuration)?;
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            prepared.number_of_vertices,
            || {
                let output = sv_build_output(configuration, prepared)?;
                sv_dispatch_render(configuration.backend_kind, &output)
            },
        )
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// # Errors
    ///
    /// Returns the render error and [`SVError::BackendFailure`] when the
    /// destination cannot be written. Only the SVG backend exports a document.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        if self.configuration.backend_kind != SVBackendKind::Svg {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.configuration.backend_kind,
                capability: SVBackendCapability::Export,
            });
        }
        let bytes = self.render()?.bytes;
        std::fs::write(destination, bytes).map_err(|error| SVError::BackendFailure {
            backend_kind: self.configuration.backend_kind,
            operation: "save",
            message: format!("cannot write {}: {error}", destination.display()),
        })
    }
}

fn sv_build_output(
    configuration: &SVParallelCoordinatesConfiguration<'_>,
    prepared: SVParallelCoordinatesPrepared,
) -> Result<SVParallelCoordinatesRenderOutput, SVError> {
    let geometries = sv_compose_geometries(configuration, &prepared)?;
    let decorations = sv_build_decorations(configuration, &prepared)?;
    Ok(SVParallelCoordinatesRenderOutput::new(
        configuration.backend_kind,
        configuration.representation,
        configuration.number_of_observations,
        prepared.viewport,
        prepared.plot_area,
        configuration.title.clone(),
        prepared.axes,
        prepared.lines,
        prepared.colour_groups,
        geometries,
        decorations,
        crate::sv_decoration::SVDecorationStyle::built_in().background(),
        configuration.normalization,
        configuration.missing,
    ))
}
