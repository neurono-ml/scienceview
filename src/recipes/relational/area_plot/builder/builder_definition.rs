//! The builder struct and the backend-free constructor.

use std::marker::PhantomData;

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::{SVExecutionMode, SVWithoutBackend};
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkTheme};
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};

use super::super::configuration::SVAreaInterpolation;
use super::super::missing_value_policy::SVAreaMissingValuePolicy;

/// Builds an [`SVAreaPlot`].
///
/// The builder starts without a backend and gains a backend state only through
/// [`SVAreaPlotBuilder::backend`], so an area plot cannot be built without an
/// explicit backend selection. Every other option is optional and has the
/// documented default.
///
/// ```
/// use scienceview_core::sv_backend::SVBackendKind;
/// use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
/// use scienceview::sv_recipes::sv_relational::sv_area_plot::SVAreaPlot;
///
/// let x = [0.0, 1.0, 2.0];
/// let y = [1.0, 3.0, 2.0];
/// let table = SVDataTable::builder()
///     .with_column("x", SVColumnRole::X, &x)
///     .with_column("y", SVColumnRole::Y, &y)
///     .build()
///     .expect("the table is well formed");
///
/// let plot = SVAreaPlot::new()
///     .data_source(&table)
///     .x("x")
///     .y("y")
///     .backend(SVBackendKind::Svg)
///     .build()
///     .expect("the configuration is complete");
///
/// assert_eq!(plot.baseline_value(), scienceview_core::sv_mark::SVBaselineMode::Zero);
/// ```
pub struct SVAreaPlotBuilder<'values, BackendState> {
    pub(super) data_source: Option<&'values dyn SVDataSource>,
    pub(super) x: Option<&'values str>,
    pub(super) y: Option<&'values str>,
    pub(super) series: Option<&'values str>,
    pub(super) stack: bool,
    pub(super) baseline: SVBaselineMode,
    pub(super) opacity: Option<f32>,
    pub(super) line: bool,
    pub(super) interpolation: SVAreaInterpolation,
    pub(super) missing_value_policy: SVAreaMissingValuePolicy,
    pub(super) theme: Option<SVMarkTheme>,
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

impl SVAreaPlotBuilder<'_, SVWithoutBackend> {
    /// Starts an area plot with no backend selected.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            stack: false,
            baseline: SVBaselineMode::Zero,
            opacity: None,
            line: true,
            interpolation: SVAreaInterpolation::default(),
            missing_value_policy: SVAreaMissingValuePolicy::default(),
            theme: None,
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

impl Default for SVAreaPlotBuilder<'_, SVWithoutBackend> {
    fn default() -> Self {
        Self::new()
    }
}
