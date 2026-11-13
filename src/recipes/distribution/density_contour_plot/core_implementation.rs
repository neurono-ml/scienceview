//! `SVDensityContourPlot`: the built figure, its getters and the composition.

use super::builder::SVDensityContourPlotBuilder;
use super::composition::{SVDensityContourComposition, sv_compose};
use super::configuration::SVDensityContourConfiguration;
use super::options::SVContourStyle;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;
use crate::sv_scale::{SVColourMap, SVScaleTransform};

/// A two-dimensional density chart ready to render.
///
/// Built only by [`SVDensityContourPlotBuilder`]: the direct constructor is
/// private, so the explicit backend rule cannot be bypassed. The backend kind
/// is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVDensityContourPlot<'data> {
    pub(super) configuration: SVDensityContourConfiguration<'data>,
}

impl std::fmt::Debug for SVDensityContourPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDensityContourPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("levels", &self.configuration.levels)
            .field("grid_count", &self.configuration.grid_count)
            .finish_non_exhaustive()
    }
}

impl<'data> SVDensityContourPlot<'data> {
    /// Starts building a density-contour plot.
    pub fn builder() -> SVDensityContourPlotBuilder<'data> {
        SVDensityContourPlotBuilder::new()
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

    /// Returns the number of logical observations in the `x` column.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_points
    }

    /// Returns the configured number of contour levels.
    pub fn levels(&self) -> usize {
        self.configuration.levels
    }

    /// Returns the explicit level range, when one was configured.
    pub fn level_range(&self) -> Option<(f64, f64)> {
        self.configuration.level_range
    }

    /// Returns the explicit bandwidth, when one was configured.
    pub fn bandwidth(&self) -> Option<f64> {
        self.configuration.bandwidth
    }

    /// Returns the bandwidth adjustment.
    pub fn bandwidth_adjustment(&self) -> f64 {
        self.configuration.bandwidth_adjustment
    }

    /// Returns the number of grid cells per axis.
    pub fn grid_count(&self) -> usize {
        self.configuration.grid_count
    }

    /// Returns the contour style.
    pub fn style(&self) -> SVContourStyle {
        self.configuration.style
    }

    /// Returns the continuous colour map.
    pub fn colour_map(&self) -> &SVColourMap {
        &self.configuration.colour_map
    }

    /// Returns the transform applied before the colour map.
    pub fn colour_scale_kind(&self) -> SVScaleTransform {
        self.configuration.colour_scale
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves every column once, bins the joint density and isolates
    /// the requested contour levels. It never densifies a sparse column and
    /// never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, stat, scale or contour error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVDensityContourComposition, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_points,
            || {
                let composition = sv_compose(configuration)?;
                sv_record_drawn_marks(&composition);
                Ok(composition)
            },
        )
    }
}

pub(super) fn sv_record_drawn_marks(composition: &SVDensityContourComposition) {
    tracing::Span::current().record("points", composition.number_of_drawn_marks());
}
