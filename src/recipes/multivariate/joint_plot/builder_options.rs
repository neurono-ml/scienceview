//! The option setters of [`SVJointPlotBuilder`](super::SVJointPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::{
    SVJointKind, SVJointMarginal, SVJointMissingValuePolicy, SVJointPlotBuilder,
    SVJointRegressionModel,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVDotShape;

impl<'data> SVJointPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required horizontal column.
    #[must_use]
    pub fn x(mut self, x: impl Into<String>) -> Self {
        self.x = Some(x.into());
        self
    }

    /// Names the required vertical column.
    #[must_use]
    pub fn y(mut self, y: impl Into<String>) -> Self {
        self.y = Some(y.into());
        self
    }

    /// Names the optional grouping column.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Selects the form of the central panel.
    #[must_use]
    pub fn kind(mut self, kind: SVJointKind) -> Self {
        self.kind = kind;
        self
    }

    /// Selects the form of the two marginal panels.
    #[must_use]
    pub fn marginal_kind(mut self, marginal_kind: SVJointMarginal) -> Self {
        self.marginal_kind = marginal_kind;
        self
    }

    /// Selects the marker shape of the scatter central panel.
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Sets the constant opacity, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Selects the model of the regression central panel.
    #[must_use]
    pub fn regression_model(mut self, regression_model: SVJointRegressionModel) -> Self {
        self.regression_model = regression_model;
        self
    }

    /// Selects how a non-finite observation is treated.
    #[must_use]
    pub fn missing_value_policy(mut self, missing_value_policy: SVJointMissingValuePolicy) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
        self
    }

    /// Selects the rendering backend. Required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to `Automatic`.
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
