//! The builder struct and the backend-free constructor.

use std::marker::PhantomData;

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::{SVExecutionMode, SVWithoutBackend};
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVMarkTheme;
use scienceview_core::sv_stat::{SVKernelShape, SVNormalisation};

use super::super::missing_value_policy::SVDensityMissingValuePolicy;

/// Builds an [`SVDensityPlot`](super::SVDensityPlot).
///
/// The builder starts without a backend and gains a backend state only through
/// [`SVDensityPlotBuilder::backend`], so a density plot cannot be built without
/// an explicit backend selection. Every other option is optional and has the
/// documented default.
///
/// ```
/// use scienceview_core::sv_backend::SVBackendKind;
/// use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
/// use scienceview::sv_recipes::sv_distribution::sv_density_plot::SVDensityPlot;
///
/// let values = [1.0, 2.0, 2.0, 3.0, 4.0];
/// let table = SVDataTable::builder()
///     .with_column("value", SVColumnRole::Y, &values)
///     .build()
///     .expect("the table is well formed");
///
/// let plot = SVDensityPlot::new()
///     .data_source(&table)
///     .value("value")
///     .backend(SVBackendKind::Svg)
///     .build()
///     .expect("the configuration is complete");
///
/// assert_eq!(plot.grid_count_value(), 256);
/// ```
pub struct SVDensityPlotBuilder<'values, BackendState> {
    pub(super) data_source: Option<&'values dyn SVDataSource>,
    pub(super) value: Option<&'values str>,
    pub(super) series: Option<&'values str>,
    pub(super) bandwidth: Option<f64>,
    pub(super) bandwidth_adjustment: f64,
    pub(super) kernel: SVKernelShape,
    pub(super) grid_count: usize,
    pub(super) cut: f64,
    pub(super) fill: bool,
    pub(super) line_width: Option<SVLength>,
    pub(super) normalisation: SVNormalisation,
    pub(super) missing_value_policy: SVDensityMissingValuePolicy,
    pub(super) theme: Option<SVMarkTheme>,
    pub(super) title: Option<&'values str>,
    pub(super) x_title: Option<&'values str>,
    pub(super) y_title: Option<&'values str>,
    pub(super) viewport: SVViewport,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) states: PhantomData<BackendState>,
}

impl SVDensityPlotBuilder<'_, SVWithoutBackend> {
    /// Starts a density plot with no backend selected.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            series: None,
            bandwidth: None,
            bandwidth_adjustment: 1.0,
            kernel: SVKernelShape::Gaussian,
            grid_count: 256,
            cut: 3.0,
            fill: false,
            line_width: None,
            normalisation: SVNormalisation::Density,
            missing_value_policy: SVDensityMissingValuePolicy::default(),
            theme: None,
            title: None,
            x_title: None,
            y_title: None,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
            states: PhantomData,
        }
    }
}

impl Default for SVDensityPlotBuilder<'_, SVWithoutBackend> {
    fn default() -> Self {
        Self::new()
    }
}
