//! `SVHexbinPlot`: the built figure, its getters and the composition.

use super::builder::SVHexbinPlotBuilder;
use super::composition::{SVHexbinComposition, sv_compose};
use super::configuration::SVHexbinConfiguration;
use super::{SVHexbinAggregation, SVHexbinMissingValuePolicy};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;
use crate::sv_scale::{SVColourMap, SVScaleTransform};
use crate::sv_stat::SVNormalisation;

/// A hexagonal density chart ready to compose and render.
///
/// Built only by [`SVHexbinPlotBuilder`]: the direct constructor is private, so
/// the explicit backend rule cannot be bypassed. The backend kind is carried,
/// never dispatched in `build`.
#[derive(Clone)]
pub struct SVHexbinPlot<'data> {
    pub(super) configuration: SVHexbinConfiguration<'data>,
}

impl std::fmt::Debug for SVHexbinPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVHexbinPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("grid_size", &self.configuration.grid_size)
            .field("min_count", &self.configuration.min_count)
            .finish_non_exhaustive()
    }
}

impl<'data> SVHexbinPlot<'data> {
    /// Starts building a hexbin plot.
    pub fn builder() -> SVHexbinPlotBuilder<'data> {
        SVHexbinPlotBuilder::new()
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

    /// Returns the configured aggregation.
    pub fn aggregation(&self) -> SVHexbinAggregation {
        self.configuration.aggregation
    }

    /// Returns the number of hexagons that span the horizontal extent.
    pub fn grid_size(&self) -> usize {
        self.configuration.grid_size
    }

    /// Returns the minimum number of observations a cell needs to be drawn.
    pub fn min_count(&self) -> usize {
        self.configuration.min_count
    }

    /// Returns the explicit extent, when one was configured.
    pub fn extent(&self) -> Option<(f64, f64, f64, f64)> {
        self.configuration.extent
    }

    /// Returns the glyph rotation in radians.
    pub fn orientation(&self) -> f64 {
        self.configuration.orientation
    }

    /// Returns the continuous colour map.
    pub fn colour_map(&self) -> &SVColourMap {
        &self.configuration.colour_map
    }

    /// Returns the transform applied before the colour map.
    pub fn colour_scale_kind(&self) -> SVScaleTransform {
        self.configuration.colour_transform
    }

    /// Returns the count normalisation.
    pub fn normalisation(&self) -> SVNormalisation {
        self.configuration.normalisation
    }

    /// Returns the missing-value policy.
    pub fn missing_value_policy(&self) -> SVHexbinMissingValuePolicy {
        self.configuration.missing_value_policy
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves every column once, deposits each observation in its
    /// hexagonal cell and paints the occupied cells through the continuous
    /// colour scale. It never densifies a sparse coordinate column and never
    /// blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale, stat or geometry error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVHexbinComposition, SVError> {
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

pub(super) fn sv_record_drawn_marks(composition: &SVHexbinComposition) {
    tracing::Span::current().record("points", composition.number_of_drawn_marks());
}
