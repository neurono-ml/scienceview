//! The fluent option surface; validation happens in `build`.

use super::super::options::{SVStepDuplicatePolicy, SVStepPosition};
use super::definition::SVStepPlotBuilder;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_scale::SVScaleKind;
use scienceview_core::sv_stat::SVStat;

impl SVStepPlotBuilder<'_> {
    /// Names the ordering column.
    #[must_use]
    pub fn x(mut self, column: impl Into<String>) -> Self {
        self.x = Some(column.into());
        self
    }

    /// Names the value column, held constant until the next `x`.
    #[must_use]
    pub fn y(mut self, column: impl Into<String>) -> Self {
        self.y = Some(column.into());
        self
    }

    /// Names the column that splits the chart into one step line per series.
    #[must_use]
    pub fn series(mut self, column: impl Into<String>) -> Self {
        self.series = Some(column.into());
        self
    }

    /// Chooses where the riser sits between two samples.
    #[must_use]
    pub fn step(mut self, position: SVStepPosition) -> Self {
        self.step_position = position;
        self
    }

    /// Chooses which observation wins when two rows share an `x`.
    #[must_use]
    pub fn duplicate_policy(mut self, policy: SVStepDuplicatePolicy) -> Self {
        self.duplicate_policy = policy;
        self
    }

    /// Sets the stroke width; `None` keeps the `mark.line_width` token.
    #[must_use]
    pub fn line_width(mut self, width: SVLength) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Sets the dash pattern.
    #[must_use]
    pub fn dash(mut self, dash: SVDashPattern) -> Self {
        self.dash = Some(dash);
        self
    }

    /// Draws a marker at every sample (`true` by default, as in bokeh).
    #[must_use]
    pub fn markers(mut self, markers: bool) -> Self {
        self.markers = markers;
        self
    }

    /// Collapses duplicate `x` values per series with an aggregate stat.
    ///
    /// The value must be [`SVStat::Aggregate`](scienceview_core::sv_stat::SVStat::Aggregate);
    /// the aggregation groups by `x` within each series.
    #[must_use]
    pub fn aggregate(mut self, aggregate: SVStat) -> Self {
        self.aggregate = Some(aggregate);
        self
    }

    /// Sets the logical canvas size.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Chooses the kind of the horizontal scale (`Continuous` or `Temporal`).
    #[must_use]
    pub fn x_scale(mut self, x_scale_kind: SVScaleKind) -> Self {
        self.x_scale_kind = x_scale_kind;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Selects the rendering backend. Required: there is no automatic backend.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to [`SVExecutionMode::Automatic`].
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
