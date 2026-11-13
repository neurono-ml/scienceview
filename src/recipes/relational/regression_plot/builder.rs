//! The builder of [`SVRegressionPlot`](super::SVRegressionPlot).
//!
//! The builder is the only public constructor. `build` exists only once a data
//! source and an explicit backend selection are present, and it validates the
//! configuration, the columns declared by the source and the mark capabilities
//! of the selected backend before the plot exists (patterns.md P2 and P7).

use super::builder_state::{
    SVWithBackend, SVWithDataSource, SVWithoutBackend, SVWithoutDataSource,
};
use super::builder_validation::{sv_validate_columns, sv_validate_marks};
use super::configuration::SVRegressionConfiguration;
use super::core_implementation::SVRegressionPlot;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
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
    pub(crate) source: Option<&'source dyn SVDataSource>,
    pub(crate) backend_kind: Option<SVBackendKind>,
    pub(crate) execution_mode: SVExecutionMode,
    pub(crate) title: Option<String>,
    pub(crate) configuration: SVRegressionConfiguration,
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
