//! `SVDensityHeatmapPlot`: the built figure, its getters and the composition.

use super::builder::SVDensityHeatmapPlotBuilder;
use super::composition::{SVDensityHeatmapComposition, sv_compose};
use super::configuration::SVDensityHeatmapConfiguration;
use super::options::SVEmptyBinPolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;
use crate::sv_scale::SVColourMap;
use crate::sv_stat::SVNormalisation;

/// A two-dimensional density chart ready to render.
///
/// Built only by [`SVDensityHeatmapPlotBuilder`]: the direct constructor is
/// private, so the explicit backend rule cannot be bypassed. The backend kind
/// is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVDensityHeatmapPlot<'data> {
    pub(super) configuration: SVDensityHeatmapConfiguration<'data>,
}

impl std::fmt::Debug for SVDensityHeatmapPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDensityHeatmapPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("bin_count_x", &self.configuration.bin_count_x)
            .field("bin_count_y", &self.configuration.bin_count_y)
            .finish_non_exhaustive()
    }
}

impl<'data> SVDensityHeatmapPlot<'data> {
    /// Starts building a density-heatmap plot.
    pub fn builder() -> SVDensityHeatmapPlotBuilder<'data> {
        SVDensityHeatmapPlotBuilder::new()
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

    /// Returns the resolved number of horizontal bins.
    pub fn bin_count_x(&self) -> usize {
        self.configuration.bin_count_x
    }

    /// Returns the resolved number of vertical bins.
    pub fn bin_count_y(&self) -> usize {
        self.configuration.bin_count_y
    }

    /// Returns the configured cell normalisation.
    pub fn normalisation(&self) -> SVNormalisation {
        self.configuration.normalise
    }

    /// Returns the continuous colour map.
    pub fn colour_map(&self) -> &SVColourMap {
        &self.configuration.colour_map
    }

    /// Returns `true` when the colour bar is drawn.
    pub fn colour_bar(&self) -> bool {
        self.configuration.colour_bar
    }

    /// Returns the configured cell opacity.
    pub fn opacity(&self) -> f32 {
        self.configuration.opacity
    }

    /// Returns the policy applied to empty bins.
    pub fn empty_bin_policy(&self) -> SVEmptyBinPolicy {
        self.configuration.empty_bin_policy
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves every column once, derives the two axes through the
    /// bin kernel, folds the cells through the aggregate kernel and paints them
    /// over a continuous colour scale. It never densifies a sparse column and
    /// never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, stat, scale or geometry error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVDensityHeatmapComposition, SVError> {
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

pub(super) fn sv_record_drawn_marks(composition: &SVDensityHeatmapComposition) {
    tracing::Span::current().record("points", composition.number_of_drawn_marks());
}
