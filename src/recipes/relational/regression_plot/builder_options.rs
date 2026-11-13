//! The optional setters of the regression builder.

use super::builder::SVRegressionPlotBuilder;
use super::extrapolation::SVExtrapolationPolicy;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDotShape;
use scienceview_core::sv_stat::SVFitModel;

impl<DataSourceState, BackendState> SVRegressionPlotBuilder<'_, DataSourceState, BackendState> {
    /// Selects the execution mode. Defaults to `Automatic`.
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Sets an optional chart title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Binds the predictor column.
    pub fn x(mut self, column: impl Into<String>) -> Self {
        self.configuration.x = Some(column.into());
        self
    }

    /// Binds the response column.
    pub fn y(mut self, column: impl Into<String>) -> Self {
        self.configuration.y = Some(column.into());
        self
    }

    /// Groups the observations and draws one fit per distinct value.
    pub fn series(mut self, column: impl Into<String>) -> Self {
        self.configuration.series = Some(column.into());
        self
    }

    /// Chooses the fitted model.
    pub fn model(mut self, model: SVFitModel) -> Self {
        self.configuration.model = model;
        self
    }

    /// Chooses the polynomial degree, used by [`SVFitModel::Polynomial`].
    pub fn order(mut self, order: usize) -> Self {
        self.configuration.order = order;
        self
    }

    /// Requests a two-sided confidence band covering the given probability.
    ///
    /// Only the linear and polynomial models carry a computed band; the stat
    /// kernel rejects the request for the other models.
    pub fn confidence(mut self, confidence: f64) -> Self {
        self.configuration.confidence = Some(confidence);
        self
    }

    /// Draws or hides the scatter points.
    pub fn show_points(mut self, show_points: bool) -> Self {
        self.configuration.show_points = show_points;
        self
    }

    /// Draws or hides the fitted line.
    pub fn show_fit(mut self, show_fit: bool) -> Self {
        self.configuration.show_fit = show_fit;
        self
    }

    /// Chooses the dot shape.
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.configuration.marker = marker;
        self
    }

    /// Sets the fit line width; the theme token applies when it is not set.
    pub fn line_width(mut self, line_width: SVLength) -> Self {
        self.configuration.line_width = line_width;
        self
    }

    /// Chooses how the curve relates to the visible horizontal domain.
    pub fn extrapolation(mut self, extrapolation: SVExtrapolationPolicy) -> Self {
        self.configuration.extrapolation = extrapolation;
        self
    }

    /// Sets the visible horizontal domain used by
    /// [`SVExtrapolationPolicy::ClipToVisibleDomain`].
    pub fn visible_x_domain(mut self, lower: f64, upper: f64) -> Self {
        self.configuration.visible_x_domain = Some((lower, upper));
        self
    }
}
