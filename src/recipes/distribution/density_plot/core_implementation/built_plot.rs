//! The built value and the configuration accessors it exposes.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVMarkTheme;
use scienceview_core::sv_stat::{SVKernelShape, SVNormalisation};

use super::super::builder::SVDensityPlotBuilder;
use super::super::configuration::SVDensityPlotConfiguration;
use super::super::missing_value_policy::SVDensityMissingValuePolicy;

/// A distribution density chart: the smoothed shape of one variable.
///
/// Built only by [`SVDensityPlotBuilder`]; the direct constructor is private,
/// so the explicit backend rule cannot be bypassed.
pub struct SVDensityPlot<'values> {
    pub(super) configuration: SVDensityPlotConfiguration<'values>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
}

impl<'values> SVDensityPlot<'values> {
    /// Starts building a density plot.
    ///
    /// The builder type is the only constructor; the method is named `new`
    /// because it starts the chart's own builder, not because it returns the
    /// built chart.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> SVDensityPlotBuilder<'values, scienceview_core::sv_figure::SVWithoutBackend> {
        SVDensityPlotBuilder::new()
    }

    pub(crate) fn from_configuration(
        configuration: SVDensityPlotConfiguration<'values>,
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

    /// Returns the value column name.
    pub fn value_column_name(&self) -> &str {
        self.configuration.value
    }

    /// Returns the optional series column name.
    pub fn series_column_name(&self) -> Option<&str> {
        self.configuration.series
    }

    /// Returns the explicit bandwidth, or `None` when the rule of thumb applies.
    pub fn bandwidth_value(&self) -> Option<f64> {
        self.configuration.bandwidth
    }

    /// Returns the bandwidth adjustment factor.
    pub fn bandwidth_adjustment_value(&self) -> f64 {
        self.configuration.bandwidth_adjustment
    }

    /// Returns the kernel shape.
    pub fn kernel_value(&self) -> SVKernelShape {
        self.configuration.kernel
    }

    /// Returns the requested number of grid samples.
    pub fn grid_count_value(&self) -> usize {
        self.configuration.grid_count
    }

    /// Returns the cut factor.
    pub fn cut_value(&self) -> f64 {
        self.configuration.cut
    }

    /// Returns whether the curve is filled to the baseline.
    pub fn fill_enabled(&self) -> bool {
        self.configuration.fill
    }

    /// Returns the explicit line width, or `None` when the token applies.
    pub fn line_width_value(&self) -> Option<SVLength> {
        self.configuration.line_width
    }

    /// Returns the normalisation of the density.
    pub fn normalisation_value(&self) -> SVNormalisation {
        self.configuration.normalisation
    }

    /// Returns the missing-value policy in force.
    pub fn missing_value_policy_value(&self) -> SVDensityMissingValuePolicy {
        self.configuration.missing_value_policy
    }

    /// Returns the theme the composed marks use, when one was set.
    pub fn theme_value(&self) -> Option<&SVMarkTheme> {
        self.configuration.theme.as_ref()
    }
}
