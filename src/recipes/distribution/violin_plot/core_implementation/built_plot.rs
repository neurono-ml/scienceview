//! The built value and the configuration accessors it exposes.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::SVExecutionMode;

use super::super::configuration::SVViolinPlotConfiguration;
use super::super::missing_value_policy::SVViolinMissingValuePolicy;
use super::super::scale_mode::SVViolinScale;

/// A violin plot: the smoothed distribution of each category (FDR-117).
///
/// Built only by [`SVViolinPlotBuilder`](super::super::SVViolinPlotBuilder),
/// which requires an explicit backend. The built value carries the backend
/// kind, the execution mode and the validated configuration.
pub struct SVViolinPlot<'values> {
    pub(super) configuration: SVViolinPlotConfiguration<'values>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'values> SVViolinPlot<'values> {
    pub(crate) fn from_configuration(
        configuration: SVViolinPlotConfiguration<'values>,
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

    /// The observed value column name.
    pub fn value_column_name(&self) -> &str {
        self.configuration.value
    }

    /// The categorical column name.
    pub fn category_column_name(&self) -> &str {
        self.configuration.category
    }

    /// The optional series column name.
    pub fn series_column_name(&self) -> Option<&str> {
        self.configuration.series
    }

    /// The explicit bandwidth, or `None` when the rule of thumb applies.
    pub fn bandwidth_value(&self) -> Option<f64> {
        self.configuration.bandwidth
    }

    /// The bandwidth adjustment factor.
    pub fn bandwidth_adjustment_value(&self) -> f64 {
        self.configuration.bandwidth_adjustment
    }

    /// The violin width as a fraction of the category slot.
    pub fn width_value(&self) -> f32 {
        self.configuration.width
    }

    /// The violin area scale.
    pub fn scale_value(&self) -> SVViolinScale {
        self.configuration.scale
    }

    /// Whether the inner box and median are drawn.
    pub fn show_box_enabled(&self) -> bool {
        self.configuration.show_box
    }

    /// Whether the sample points are drawn.
    pub fn show_points_enabled(&self) -> bool {
        self.configuration.show_points
    }

    /// Whether two series share a category in split halves.
    pub fn split_enabled(&self) -> bool {
        self.configuration.split
    }

    /// The missing-value policy in force.
    pub fn missing_value_policy_value(&self) -> SVViolinMissingValuePolicy {
        self.configuration.missing_value_policy
    }
}

impl std::fmt::Debug for SVViolinPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVViolinPlot")
            .field("backend_kind", &self.backend_kind)
            .field("execution_mode", &self.execution_mode)
            .finish_non_exhaustive()
    }
}
