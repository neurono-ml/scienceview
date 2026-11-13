//! The `SVBarPlot` builder and its validating state machine.

use scienceview_core::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBarOrientation, SVFormatSpec, SVMarkKind};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use super::arrangement::SVBarArrangement;
use super::configuration::{SVBarPlotConfiguration, sv_invalid};
use super::core_implementation::SVBarPlot;
use super::data_contract::sv_validate_columns;
use super::missing_value_policy::SVBarMissingValuePolicy;
use super::ordering::SVBarOrdering;

/// Builds an [`SVBarPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVBarPlotBuilder::build), so a bar plot that exists can render.
#[derive(Clone)]
pub struct SVBarPlotBuilder<'values> {
    data_source: Option<&'values dyn SVDataSource>,
    category: Option<&'values str>,
    value: Option<&'values str>,
    series: Option<&'values str>,
    weight: Option<&'values str>,
    estimator: SVAggregateMethod,
    error: Option<SVAggregateErrorKind>,
    confidence_level: f64,
    arrangement: SVBarArrangement,
    ordering: SVBarOrdering,
    missing_value_policy: SVBarMissingValuePolicy,
    orientation: SVBarOrientation,
    width: f32,
    colour: Option<SVColour>,
    show_labels: bool,
    label_format: SVFormatSpec,
    title: Option<&'values str>,
    x_title: Option<&'values str>,
    y_title: Option<&'values str>,
    viewport: SVViewport,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
}

impl<'values> SVBarPlotBuilder<'values> {
    /// Starts a bar plot with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            category: None,
            value: None,
            series: None,
            weight: None,
            estimator: SVAggregateMethod::Mean,
            error: None,
            confidence_level: 0.95,
            arrangement: SVBarArrangement::Dodge,
            ordering: SVBarOrdering::Data,
            missing_value_policy: SVBarMissingValuePolicy::Omit,
            orientation: SVBarOrientation::Vertical,
            width: 0.8,
            colour: None,
            show_labels: false,
            label_format: SVFormatSpec::Automatic,
            title: None,
            x_title: None,
            y_title: None,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }

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

    /// Names the optional grouping column: one bar per series in a category.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Names the optional observation-weight column.
    #[must_use]
    pub fn weight(mut self, weight: &'values str) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Chooses the statistic reported per category (default
    /// [`SVAggregateMethod::Mean`]).
    #[must_use]
    pub fn estimator(mut self, estimator: SVAggregateMethod) -> Self {
        self.estimator = estimator;
        self
    }

    /// Requests an uncertainty interval next to every bar (default `None`).
    ///
    /// A `Percentile` kind uses the confidence level set by
    /// [`confidence_interval`](Self::confidence_interval).
    #[must_use]
    pub fn error(mut self, error: SVAggregateErrorKind) -> Self {
        self.error = Some(error);
        self
    }

    /// Sets the confidence level a percentile interval covers (default `0.95`).
    #[must_use]
    pub fn confidence_interval(mut self, confidence_level: f64) -> Self {
        self.confidence_level = confidence_level;
        self
    }

    /// Chooses how the series share a category slot (default
    /// [`SVBarArrangement::Dodge`]).
    #[must_use]
    pub fn arrangement(mut self, arrangement: SVBarArrangement) -> Self {
        self.arrangement = arrangement;
        self
    }

    /// Chooses the order of the categories (default [`SVBarOrdering::Data`]).
    #[must_use]
    pub fn ordering(mut self, ordering: SVBarOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    /// Chooses what a non-finite observation does (default
    /// [`SVBarMissingValuePolicy::Omit`]).
    #[must_use]
    pub fn missing_value_policy(mut self, policy: SVBarMissingValuePolicy) -> Self {
        self.missing_value_policy = policy;
        self
    }

    /// Chooses the value orientation (default [`SVBarOrientation::Vertical`]).
    #[must_use]
    pub fn orient(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the bar width as a fraction of the slot (default `0.8`).
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Overrides the series palette with one explicit colour.
    #[must_use]
    pub fn colour(mut self, colour: SVColour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Draws the aggregate value above every bar (default `false`).
    #[must_use]
    pub fn label(mut self, label: bool) -> Self {
        self.show_labels = label;
        self
    }

    /// Sets how a value label is formatted (default
    /// [`SVFormatSpec::Automatic`]).
    #[must_use]
    pub fn label_format(mut self, label_format: SVFormatSpec) -> Self {
        self.label_format = label_format;
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

    /// Validates the configuration and builds the bar plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`]
    /// or a configuration error naming the missing column, the invalid width,
    /// the invalid confidence level, the weighted estimator that cannot accept
    /// weights, or the backend that cannot host a required mark.
    pub fn build(self) -> Result<SVBarPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .ok_or_else(|| sv_invalid("a bar plot needs the `value` column"))?;
        let category = self
            .category
            .ok_or_else(|| sv_invalid("a bar plot needs the `category` column"))?;
        if !self.width.is_finite() || self.width <= 0.0 || self.width > 1.0 {
            return Err(sv_invalid(
                "the bar width must be finite and lie in 0.0..=1.0",
            ));
        }
        if !self.confidence_level.is_finite() || !(0.0..1.0).contains(&self.confidence_level) {
            return Err(sv_invalid(
                "the confidence level must be finite and lie strictly inside (0, 1)",
            ));
        }
        if self.weight.is_some()
            && !matches!(
                self.estimator,
                SVAggregateMethod::Mean | SVAggregateMethod::Sum
            )
        {
            return Err(sv_invalid(
                "observation weights are only valid for the mean and the sum estimators",
            ));
        }
        let mut required_marks = vec![SVMarkKind::Bar];
        if self.error.is_some() {
            required_marks.push(SVMarkKind::ErrorBar);
        }
        if self.show_labels {
            required_marks.push(SVMarkKind::Text);
        }
        for mark_kind in required_marks {
            sv_verify_mark_supported(mark_kind, backend_kind)?;
        }

        let configuration = SVBarPlotConfiguration {
            data_source,
            category,
            value,
            series: self.series,
            weight: self.weight,
            estimator: self.estimator,
            error: self.error,
            confidence_level: self.confidence_level,
            arrangement: self.arrangement,
            ordering: self.ordering,
            missing_value_policy: self.missing_value_policy,
            orientation: self.orientation,
            width: self.width,
            colour: self.colour,
            show_labels: self.show_labels,
            label_format: self.label_format,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
        };
        sv_validate_columns(&configuration)?;

        Ok(SVBarPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}

impl Default for SVBarPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
