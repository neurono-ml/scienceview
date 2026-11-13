//! The builder value, its backend typestate and its build step.

use std::marker::PhantomData;

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::{SVExecutionMode, SVWithBackend, SVWithoutBackend};
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};
use scienceview_core::sv_stat::SVAggregateErrorKind;

use super::super::configuration::{SVBandPlotConfiguration, sv_invalid};
use super::super::core_implementation::SVBandPlot;
use super::super::missing_value_policy::SVMissingValuePolicy;
use super::validation::sv_validate_mode;

/// Builds an [`SVBandPlot`].
///
/// The builder starts without a backend and gains a backend state only through
/// [`SVBandPlotBuilder::backend`], so a band plot cannot be built without an
/// explicit backend selection. Every other option is optional and has the
/// documented default.
pub struct SVBandPlotBuilder<'values, BackendState> {
    pub(super) data_source: Option<&'values dyn SVDataSource>,
    pub(super) x: Option<&'values str>,
    pub(super) lower: Option<&'values str>,
    pub(super) upper: Option<&'values str>,
    pub(super) values: Option<&'values str>,
    pub(super) series: Option<&'values str>,
    pub(super) derived: bool,
    pub(super) error: SVAggregateErrorKind,
    pub(super) opacity: Option<f32>,
    pub(super) fill: bool,
    pub(super) border: bool,
    pub(super) dashed_borders: bool,
    pub(super) centre_line: bool,
    pub(super) missing_value_policy: SVMissingValuePolicy,
    pub(super) title: Option<&'values str>,
    pub(super) x_title: Option<&'values str>,
    pub(super) y_title: Option<&'values str>,
    pub(super) x_scale_kind: SVScaleKind,
    pub(super) temporal_unit: SVTemporalUnit,
    pub(super) viewport: SVViewport,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<BackendState>,
}

impl SVBandPlotBuilder<'_, SVWithoutBackend> {
    /// Starts a band plot with no backend selected.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            lower: None,
            upper: None,
            values: None,
            series: None,
            derived: false,
            error: SVAggregateErrorKind::StandardDeviation,
            opacity: None,
            fill: true,
            border: false,
            dashed_borders: true,
            centre_line: false,
            missing_value_policy: SVMissingValuePolicy::default(),
            title: None,
            x_title: None,
            y_title: None,
            x_scale_kind: SVScaleKind::Continuous,
            temporal_unit: SVTemporalUnit::Day,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
            states: PhantomData,
        }
    }
}

impl Default for SVBandPlotBuilder<'_, SVWithoutBackend> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'values> SVBandPlotBuilder<'values, SVWithoutBackend> {
    /// Selects the rendering backend.
    ///
    /// Required: there is no automatic backend, so the rendering form is
    /// always a deliberate choice.
    #[must_use]
    pub fn backend(self, backend_kind: SVBackendKind) -> SVBandPlotBuilder<'values, SVWithBackend> {
        SVBandPlotBuilder {
            data_source: self.data_source,
            x: self.x,
            lower: self.lower,
            upper: self.upper,
            values: self.values,
            series: self.series,
            derived: self.derived,
            error: self.error,
            opacity: self.opacity,
            fill: self.fill,
            border: self.border,
            dashed_borders: self.dashed_borders,
            centre_line: self.centre_line,
            missing_value_policy: self.missing_value_policy,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            x_scale_kind: self.x_scale_kind,
            temporal_unit: self.temporal_unit,
            viewport: self.viewport,
            backend_kind: Some(backend_kind),
            execution_mode: self.execution_mode,
            states: PhantomData,
        }
    }
}

impl<'values> SVBandPlotBuilder<'values, SVWithBackend> {
    /// Validates the configuration and builds the band plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] when no source was attached,
    /// [`SVError::MissingBackendSelection`] when the backend state is somehow
    /// incomplete, and [`SVError::InvalidConfiguration`] for a missing column,
    /// a mixed explicit/derived mode or an out-of-range option.
    pub fn build(self) -> Result<SVBandPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = self
            .x
            .ok_or_else(|| sv_invalid("a band plot needs the `x` column"))?;
        sv_validate_mode(&self)?;
        if let Some(opacity) = self.opacity {
            if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
                return Err(sv_invalid(
                    "the band opacity must be finite and lie in 0.0..=1.0",
                ));
            }
        }
        if let SVAggregateErrorKind::Percentile(probability) = self.error {
            if !probability.is_finite() || !(0.0..1.0).contains(&probability) {
                return Err(sv_invalid(
                    "the band confidence must be finite and lie in (0, 1)",
                ));
            }
        }

        let configuration = SVBandPlotConfiguration {
            data_source,
            x,
            lower: self.lower,
            upper: self.upper,
            values: self.values,
            series: self.series,
            derived: self.derived,
            error: self.error,
            opacity: self.opacity,
            fill: self.fill,
            border: self.border,
            dashed_borders: self.dashed_borders,
            centre_line: self.centre_line,
            missing_value_policy: self.missing_value_policy,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            x_scale_kind: self.x_scale_kind,
            temporal_unit: self.temporal_unit,
            viewport: self.viewport,
        };

        Ok(SVBandPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}
