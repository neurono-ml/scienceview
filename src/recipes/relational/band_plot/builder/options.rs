//! The option surface: one method per setting, no validation.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};
use scienceview_core::sv_stat::SVAggregateErrorKind;

use super::super::missing_value_policy::SVBandMissingValuePolicy;
use super::builder_definition::SVBandPlotBuilder;

impl<'values, BackendState> SVBandPlotBuilder<'values, BackendState> {
    /// Attaches the multi-column data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the ordering column.
    #[must_use]
    pub fn x(mut self, x: &'values str) -> Self {
        self.x = Some(x);
        self
    }

    /// Names the lower boundary column of the explicit mode.
    #[must_use]
    pub fn lower(mut self, lower: &'values str) -> Self {
        self.lower = Some(lower);
        self
    }

    /// Names the upper boundary column of the explicit mode.
    #[must_use]
    pub fn upper(mut self, upper: &'values str) -> Self {
        self.upper = Some(upper);
        self
    }

    /// Selects the derived mode.
    #[must_use]
    pub fn derive(mut self) -> Self {
        self.derived = true;
        self
    }

    /// Names the value column the derived mode aggregates.
    #[must_use]
    pub fn values(mut self, values: &'values str) -> Self {
        self.values = Some(values);
        self
    }

    /// Names the optional grouping column: one band per series.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Chooses the uncertainty band of the derived mode.
    #[must_use]
    pub fn error(mut self, error: SVAggregateErrorKind) -> Self {
        self.error = error;
        self
    }

    /// Selects a centred percentile interval at `confidence` (for example 0.95).
    #[must_use]
    pub fn confidence(mut self, confidence: f64) -> Self {
        self.error = SVAggregateErrorKind::Percentile(confidence);
        self
    }

    /// Overrides the `mark.area_opacity` token.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Draws the filled region (default `true`).
    #[must_use]
    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    /// Draws the two boundary lines (default `false`).
    #[must_use]
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    /// Dashes the boundary lines when the border is enabled (default `true`).
    #[must_use]
    pub fn dashed_borders(mut self, dashed_borders: bool) -> Self {
        self.dashed_borders = dashed_borders;
        self
    }

    /// Composes a solid line through the centre of the band (default `false`).
    #[must_use]
    pub fn centre_line(mut self, centre_line: bool) -> Self {
        self.centre_line = centre_line;
        self
    }

    /// Chooses what happens to a non-finite sample.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVBandMissingValuePolicy) -> Self {
        self.missing_value_policy = missing_value_policy;
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
