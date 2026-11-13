//! Every optional builder setter.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVMarkTheme;
use scienceview_core::sv_stat::{SVKernelShape, SVNormalisation};

use super::super::missing_value_policy::SVDensityMissingValuePolicy;
use super::builder_definition::SVDensityPlotBuilder;

impl<'values, BackendState> SVDensityPlotBuilder<'values, BackendState> {
    /// Attaches the multi-column data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the continuous column whose density is estimated.
    #[must_use]
    pub fn value(mut self, value: &'values str) -> Self {
        self.value = Some(value);
        self
    }

    /// Names the optional grouping column: one density per distinct code.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Names the optional observation weight column.
    ///
    /// The weights reach `SVStat::KernelDensity` unchanged: they are resolved
    /// once per series and the estimate is normalised by their total. Negative
    /// or non-finite weights are rejected with a typed error naming the
    /// position.
    #[must_use]
    pub fn weight(mut self, weight: &'values str) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Fixes the bandwidth instead of deriving it with Silverman's rule.
    #[must_use]
    pub fn bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Scales the bandwidth: the estimate uses `bandwidth * adjustment`
    /// (default `1.0`).
    ///
    /// The adjustment also scales a bandwidth derived by the rule of thumb, so
    /// `2.0` doubles the smoothing whether or not the bandwidth was fixed.
    #[must_use]
    pub fn bandwidth_adjustment(mut self, bandwidth_adjustment: f64) -> Self {
        self.bandwidth_adjustment = bandwidth_adjustment;
        self
    }

    /// Chooses the kernel shape (Gaussian by default).
    #[must_use]
    pub fn kernel(mut self, kernel: SVKernelShape) -> Self {
        self.kernel = kernel;
        self
    }

    /// Chooses how many samples the evaluation grid carries (default `256`).
    #[must_use]
    pub fn grid_count(mut self, grid_count: usize) -> Self {
        self.grid_count = grid_count;
        self
    }

    /// Chooses how many bandwidths the grid extends beyond the data
    /// (default `3.0`).
    #[must_use]
    pub fn cut(mut self, cut: f64) -> Self {
        self.cut = cut;
        self
    }

    /// Fills the curve down to the zero baseline (default `false`).
    #[must_use]
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    /// Overrides the `mark.line_width` token of the curve.
    #[must_use]
    pub fn line_width(mut self, line_width: SVLength) -> Self {
        self.line_width = Some(line_width);
        self
    }

    /// Chooses how the density is normalised before it reaches the vertical
    /// scale (`Density` by default).
    #[must_use]
    pub fn normalise(mut self, normalisation: SVNormalisation) -> Self {
        self.normalisation = normalisation;
        self
    }

    /// Chooses what happens to a non-finite observation.
    #[must_use]
    pub fn missing_value_policy(
        mut self,
        missing_value_policy: SVDensityMissingValuePolicy,
    ) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Replaces the theme defaults of the composed marks.
    #[must_use]
    pub fn theme(mut self, theme: SVMarkTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: &'values str) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_title(mut self, x_title: &'values str) -> Self {
        self.x_title = Some(x_title);
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_title(mut self, y_title: &'values str) -> Self {
        self.y_title = Some(y_title);
        self
    }

    /// Sets the logical size of the rendered figure.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Selects the execution mode (defaults to `Automatic`).
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
