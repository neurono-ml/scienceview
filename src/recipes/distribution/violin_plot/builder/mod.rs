//! The `SVViolinPlot` builder and its validating state machine.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVBarOrientation;

use super::missing_value_policy::SVViolinMissingValuePolicy;
use super::scale_mode::SVViolinScale;

mod build;
mod options;

/// Builds an [`SVViolinPlot`](super::SVViolinPlot).
///
/// `backend(...)` is required — there is no automatic backend — while
/// `execution_mode(...)` defaults to [`SVExecutionMode::Automatic`]. Every
/// option is validated in [`build`](SVViolinPlotBuilder::build).
#[derive(Clone)]
pub struct SVViolinPlotBuilder<'values> {
    data_source: Option<&'values dyn SVDataSource>,
    value: Option<&'values str>,
    category: Option<&'values str>,
    series: Option<&'values str>,
    bandwidth: Option<f64>,
    bandwidth_adjustment: f64,
    width: f32,
    scale: SVViolinScale,
    show_box: bool,
    show_points: bool,
    split: bool,
    orientation: SVBarOrientation,
    missing_value_policy: SVViolinMissingValuePolicy,
    title: Option<&'values str>,
    x_title: Option<&'values str>,
    y_title: Option<&'values str>,
    viewport: SVViewport,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
}

impl SVViolinPlotBuilder<'_> {
    /// Starts a violin plot with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            category: None,
            series: None,
            bandwidth: None,
            bandwidth_adjustment: 1.0,
            width: 0.8,
            scale: SVViolinScale::Area,
            show_box: false,
            show_points: false,
            split: false,
            orientation: SVBarOrientation::Vertical,
            missing_value_policy: SVViolinMissingValuePolicy::default(),
            title: None,
            x_title: None,
            y_title: None,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVViolinPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
