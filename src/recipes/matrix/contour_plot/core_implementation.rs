//! `SVContourPlot`: the built figure, its getters and the composition.

use super::builder::SVContourPlotBuilder;
use super::composition::{SVContourComposition, sv_compose};
use super::configuration::SVContourConfiguration;
use super::field::SVContourField;
use super::options::{SVContourMissingPolicy, SVContourStyle};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;
use crate::sv_scale::{SVColourMap, SVScaleTransform};

/// A contour chart ready to render.
///
/// Built only by [`SVContourPlotBuilder`]: the direct constructor is private,
/// so the explicit backend rule cannot be bypassed. The backend kind is
/// carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVContourPlot<'data> {
    pub(super) configuration: SVContourConfiguration<'data>,
}

impl std::fmt::Debug for SVContourPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVContourPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("levels", &self.configuration.levels)
            .field("style", &self.configuration.style)
            .finish_non_exhaustive()
    }
}

impl<'data> SVContourPlot<'data> {
    /// Starts building a contour plot.
    pub fn builder() -> SVContourPlotBuilder<'data> {
        SVContourPlotBuilder::new()
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

    /// Returns the number of logical samples in the value column.
    pub fn value_count(&self) -> usize {
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

    /// Returns the explicit level spacing, when one was configured.
    pub fn level_size(&self) -> Option<f64> {
        self.configuration.level_size
    }

    /// Returns the contour style.
    pub fn style(&self) -> SVContourStyle {
        self.configuration.style
    }

    /// Returns the missing-value policy.
    pub fn missing_policy(&self) -> SVContourMissingPolicy {
        self.configuration.missing_policy
    }

    /// Returns `true` when threshold labels are drawn.
    pub fn labels(&self) -> bool {
        self.configuration.labels
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
    /// The call resolves the field once — folding a matrix layout or binning a
    /// point cloud through [`SVStat::Bin`](crate::sv_stat::SVStat::Bin) — and
    /// isolates the requested levels. It never densifies a sparse column and
    /// never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, stat, scale or contour error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVContourComposition, SVError> {
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

    /// Returns the sampled field the chart would draw, without composing the
    /// scene.
    ///
    /// # Errors
    ///
    /// Returns the column or interpolation error of the field.
    pub fn field(&self) -> Result<SVContourField, SVError> {
        super::preparation::sv_prepare_field(&self.configuration)
    }
}

pub(super) fn sv_record_drawn_marks(composition: &SVContourComposition) {
    tracing::Span::current().record("points", composition.number_of_drawn_marks());
}
