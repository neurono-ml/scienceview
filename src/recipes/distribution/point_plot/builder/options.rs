//! The option surface of the point-plot builder.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::{SVBarOrientation, SVDashPattern, SVDotShape};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use crate::sv_recipes::sv_distribution::sv_point_plot::missing_value_policy::SVPointMissingValuePolicy;
use crate::sv_recipes::sv_distribution::sv_point_plot::ordering::SVPointOrdering;

use super::definition::SVPointPlotBuilder;

impl<'values> SVPointPlotBuilder<'values> {
    /// Attaches the data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the categorical axis column.
    #[must_use]
    pub fn category(mut self, category: &'values str) -> Self {
        self.category = Some(category);
        self
    }

    /// Names the measured numeric column.
    #[must_use]
    pub fn value(mut self, value: &'values str) -> Self {
        self.value = Some(value);
        self
    }

    /// Names the optional grouping column: one point per series in a category.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Chooses the statistic reported per category (default
    /// [`SVAggregateMethod::Mean`]).
    #[must_use]
    pub fn estimator(mut self, estimator: SVAggregateMethod) -> Self {
        self.estimator = estimator;
        self
    }

    /// Chooses the uncertainty band drawn around every point (default
    /// [`SVAggregateErrorKind::StandardDeviation`]).
    ///
    /// A `Percentile` kind uses the confidence level set by
    /// [`confidence_interval`](Self::confidence_interval).
    #[must_use]
    pub fn error(mut self, error: SVAggregateErrorKind) -> Self {
        self.error = error;
        self
    }

    /// Sets the confidence level a percentile interval covers (default `0.95`).
    #[must_use]
    pub fn confidence_interval(mut self, confidence_level: f64) -> Self {
        self.confidence_level = confidence_level;
        self
    }

    /// Chooses the order of the categories (default [`SVPointOrdering::Data`]).
    #[must_use]
    pub fn ordering(mut self, ordering: SVPointOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    /// Chooses what a non-finite observation does (default
    /// [`SVPointMissingValuePolicy::Omit`]).
    #[must_use]
    pub fn missing_value_policy(mut self, policy: SVPointMissingValuePolicy) -> Self {
        self.missing_value_policy = policy;
        self
    }

    /// Chooses the value orientation (default [`SVBarOrientation::Vertical`]).
    #[must_use]
    pub fn orient(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Chooses the marker shape (default [`SVDotShape::Circle`]).
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Draws the caps of every error bar (default `true`).
    #[must_use]
    pub fn caps(mut self, caps: bool) -> Self {
        self.caps = caps;
        self
    }

    /// Sets the cap extent as a fraction of the category slot (default `0.1`).
    #[must_use]
    pub fn cap_width(mut self, cap_width: f32) -> Self {
        self.cap_width = cap_width;
        self
    }

    /// Joins the points of one series with a line (default `false`).
    #[must_use]
    pub fn join(mut self, join: bool) -> Self {
        self.join = join;
        self
    }

    /// Sets the width of the joining line; `None` keeps the
    /// `mark.line_width` theme token.
    #[must_use]
    pub fn line_width(mut self, line_width: SVLength) -> Self {
        self.line_width = Some(line_width);
        self
    }

    /// Sets the dash pattern of the joining line (default solid).
    #[must_use]
    pub fn dash(mut self, dash: SVDashPattern) -> Self {
        self.dash = Some(dash);
        self
    }

    /// Overrides the series palette with one explicit colour.
    #[must_use]
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: &'values str) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the title of the horizontal axis.
    #[must_use]
    pub fn x_title(mut self, x_title: &'values str) -> Self {
        self.x_title = Some(x_title);
        self
    }

    /// Sets the title of the vertical axis.
    #[must_use]
    pub fn y_title(mut self, y_title: &'values str) -> Self {
        self.y_title = Some(y_title);
        self
    }

    /// Sets the logical viewport.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Selects the rendering backend, which is required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Sets the execution mode (default `Automatic`).
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
