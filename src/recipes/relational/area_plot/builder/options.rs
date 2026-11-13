//! Every optional builder setter.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkTheme};
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};

use super::super::configuration::SVAreaInterpolation;
use super::super::missing_value_policy::SVAreaMissingValuePolicy;
use super::builder_definition::SVAreaPlotBuilder;

impl<'values, BackendState> SVAreaPlotBuilder<'values, BackendState> {
    /// Attaches the multi-column data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the ordered horizontal column.
    #[must_use]
    pub fn x(mut self, x: &'values str) -> Self {
        self.x = Some(x);
        self
    }

    /// Names the vertical value column.
    #[must_use]
    pub fn y(mut self, y: &'values str) -> Self {
        self.y = Some(y);
        self
    }

    /// Names the optional grouping column: one area per distinct value.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Stacks the areas on each other instead of closing each against the
    /// baseline (default `false`).
    #[must_use]
    pub fn stack(mut self, stack: bool) -> Self {
        self.stack = stack;
        self
    }

    /// Chooses the baseline a non-stacked area closes against.
    #[must_use]
    pub fn baseline(mut self, baseline: SVBaselineMode) -> Self {
        self.baseline = baseline;
        self
    }

    /// Overrides the `mark.area_opacity` token.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Draws the boundary line of each area (default `true`).
    #[must_use]
    pub fn line(mut self, line: bool) -> Self {
        self.line = line;
        self
    }

    /// Chooses how the ordered boundary joins consecutive samples.
    #[must_use]
    pub fn interpolate(mut self, interpolation: SVAreaInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Chooses what happens to a non-finite sample.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVAreaMissingValuePolicy) -> Self {
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

    /// Chooses the horizontal scale kind (`Continuous` by default).
    ///
    /// Only `Continuous` and `Temporal` place an ordered axis; the other kinds
    /// are rejected at build time.
    #[must_use]
    pub fn x_scale_kind(mut self, x_scale_kind: SVScaleKind) -> Self {
        self.x_scale_kind = x_scale_kind;
        self
    }

    /// Sets the temporal unit used by a temporal horizontal scale.
    #[must_use]
    pub fn temporal_unit(mut self, temporal_unit: SVTemporalUnit) -> Self {
        self.temporal_unit = temporal_unit;
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
