//! The built value and the configuration accessors it exposes.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkTheme};

use super::super::builder::SVAreaPlotBuilder;
use super::super::configuration::{SVAreaInterpolation, SVAreaPlotConfiguration};
use super::super::missing_value_policy::SVAreaMissingValuePolicy;

/// A relational area chart: the region between a line and a baseline.
///
/// Built only by [`SVAreaPlotBuilder`]; the direct constructor is private, so
/// the explicit backend rule cannot be bypassed.
pub struct SVAreaPlot<'values> {
    pub(super) configuration: SVAreaPlotConfiguration<'values>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'values> SVAreaPlot<'values> {
    /// Starts building an area plot.
    ///
    /// The builder type is the only constructor; the method is named `new`
    /// because it starts the chart's own builder, not because it returns the
    /// built chart.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> SVAreaPlotBuilder<'values, scienceview_core::sv_figure::SVWithoutBackend> {
        SVAreaPlotBuilder::new()
    }

    pub(crate) fn from_configuration(
        configuration: SVAreaPlotConfiguration<'values>,
        backend_kind: SVBackendKind,
        execution_mode: SVExecutionMode,
    ) -> Self {
        Self {
            configuration,
            backend_kind,
            execution_mode,
        }
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the selected execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// Returns the horizontal column name.
    pub fn x_column_name(&self) -> &str {
        self.configuration.x
    }

    /// Returns the vertical column name.
    pub fn y_column_name(&self) -> &str {
        self.configuration.y
    }

    /// Returns the optional series column name.
    pub fn series_column_name(&self) -> Option<&str> {
        self.configuration.series
    }

    /// Returns whether the areas are stacked.
    pub fn stack_enabled(&self) -> bool {
        self.configuration.stack
    }

    /// Returns the configured baseline of a non-stacked area.
    pub fn baseline_value(&self) -> SVBaselineMode {
        self.configuration.baseline
    }

    /// Returns the explicit opacity, or `None` when the token applies.
    pub fn opacity_value(&self) -> Option<f32> {
        self.configuration.opacity
    }

    /// Returns whether the boundary line is drawn.
    pub fn line_enabled(&self) -> bool {
        self.configuration.line
    }

    /// Returns the interpolation of the ordered boundary.
    pub fn interpolation_value(&self) -> SVAreaInterpolation {
        self.configuration.interpolation
    }

    /// Returns the missing-value policy in force.
    pub fn missing_value_policy_value(&self) -> SVAreaMissingValuePolicy {
        self.configuration.missing_value_policy
    }

    /// Returns the theme the composed marks use, when one was set.
    pub fn theme_value(&self) -> Option<&SVMarkTheme> {
        self.configuration.theme.as_ref()
    }
}
