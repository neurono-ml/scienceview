//! The builder of [`SVRegressionPlot`](super::SVRegressionPlot).
//!
//! The builder is the only public constructor. `build` exists only once a data
//! source and an explicit backend selection are present, and it validates the
//! configuration, the columns declared by the source and the mark capabilities
//! of the selected backend before the plot exists (patterns.md P2 and P7).

use super::builder_state::{
    SVWithBackend, SVWithDataSource, SVWithoutBackend, SVWithoutDataSource,
};
use super::configuration::{SVExtrapolationPolicy, SVRegressionConfiguration};
use super::core_implementation::SVRegressionPlot;
use scienceview_core::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::{SVDotShape, SVMarkKind};
use scienceview_core::sv_stat::SVFitModel;
use std::marker::PhantomData;

/// Builds a [`SVRegressionPlot`].
///
/// ```
/// use scienceview_core::sv_backend::SVBackendKind;
/// use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
/// use scienceview::recipes::relational::{SVRegressionPlot, SVRegressionPlotBuilder};
///
/// let x = [0.0, 1.0, 2.0, 3.0];
/// let y = [1.0, 3.0, 5.0, 7.0];
/// let table = SVDataTable::builder()
///     .with_column("x", SVColumnRole::X, &x)
///     .with_column("y", SVColumnRole::Y, &y)
///     .build()
///     .expect("the table is well formed");
///
/// let plot = SVRegressionPlotBuilder::new()
///     .data_source(&table)
///     .x("x")
///     .y("y")
///     .backend(SVBackendKind::Svg)
///     .build()
///     .expect("the configuration is complete");
///
/// assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
/// ```
pub struct SVRegressionPlotBuilder<'source, DataSourceState, BackendState> {
    source: Option<&'source dyn SVDataSource>,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
    title: Option<String>,
    configuration: SVRegressionConfiguration,
    states: PhantomData<(DataSourceState, BackendState)>,
}

impl SVRegressionPlotBuilder<'_, SVWithoutDataSource, SVWithoutBackend> {
    /// Starts a regression chart with neither data nor a backend.
    pub fn new() -> Self {
        Self {
            source: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
            title: None,
            configuration: SVRegressionConfiguration::new(),
            states: PhantomData,
        }
    }
}

impl Default for SVRegressionPlotBuilder<'_, SVWithoutDataSource, SVWithoutBackend> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'source, BackendState> SVRegressionPlotBuilder<'source, SVWithoutDataSource, BackendState> {
    /// Attaches the data source the chart reads.
    pub fn data_source(
        self,
        source: &'source dyn SVDataSource,
    ) -> SVRegressionPlotBuilder<'source, SVWithDataSource, BackendState> {
        SVRegressionPlotBuilder {
            source: Some(source),
            backend_kind: self.backend_kind,
            execution_mode: self.execution_mode,
            title: self.title,
            configuration: self.configuration,
            states: PhantomData,
        }
    }
}

impl<'source, DataSourceState> SVRegressionPlotBuilder<'source, DataSourceState, SVWithoutBackend> {
    /// Selects the rendering backend.
    ///
    /// Required: there is no automatic backend, so the rendering form is always
    /// a deliberate choice.
    pub fn backend(
        self,
        backend_kind: SVBackendKind,
    ) -> SVRegressionPlotBuilder<'source, DataSourceState, SVWithBackend> {
        SVRegressionPlotBuilder {
            source: self.source,
            backend_kind: Some(backend_kind),
            execution_mode: self.execution_mode,
            title: self.title,
            configuration: self.configuration,
            states: PhantomData,
        }
    }
}

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

impl<'source> SVRegressionPlotBuilder<'source, SVWithDataSource, SVWithBackend> {
    /// Validates the configuration and builds the chart.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] for an incomplete option set,
    /// [`SVError::ColumnNotFound`] when a bound column is not declared by the
    /// source, and [`SVError::UnsupportedMark`] when the selected backend
    /// cannot draw one of the composed marks.
    pub fn build(self) -> Result<SVRegressionPlot<'source>, SVError> {
        let source = self.source.ok_or_else(|| SVError::InvalidConfiguration {
            message: "a regression plot needs a data source".to_string(),
        })?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;

        self.configuration.validate()?;
        sv_validate_columns(source, &self.configuration)?;
        sv_validate_marks(backend_kind, &self.configuration)?;

        Ok(SVRegressionPlot::from_configuration(
            source,
            backend_kind,
            self.execution_mode,
            self.title,
            self.configuration,
        ))
    }
}

/// Checks that every bound column is declared by the source.
fn sv_validate_columns(
    source: &dyn SVDataSource,
    configuration: &SVRegressionConfiguration,
) -> Result<(), SVError> {
    let schema = source.schema();
    let bound = [
        configuration.x.as_deref(),
        configuration.y.as_deref(),
        configuration.series.as_deref(),
    ];
    for name in bound.into_iter().flatten() {
        if schema.column(name).is_none() {
            return Err(SVError::ColumnNotFound {
                name: name.to_string(),
            });
        }
    }
    Ok(())
}

/// Checks the selected backend can draw every composed mark.
fn sv_validate_marks(
    backend_kind: SVBackendKind,
    configuration: &SVRegressionConfiguration,
) -> Result<(), SVError> {
    if configuration.show_points {
        sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;
    }
    if configuration.show_fit {
        sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;
    }
    if configuration.confidence.is_some() {
        sv_verify_mark_supported(SVMarkKind::Ribbon, backend_kind)?;
    }
    Ok(())
}
