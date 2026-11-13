//! The builder struct and its starting state.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::{SVBarOrientation, SVDashPattern, SVDotShape};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use crate::sv_recipes::sv_distribution::sv_point_plot::missing_value_policy::SVPointMissingValuePolicy;
use crate::sv_recipes::sv_distribution::sv_point_plot::ordering::SVPointOrdering;

/// Builds an [`SVPointPlot`](crate::sv_recipes::sv_distribution::sv_point_plot::SVPointPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVPointPlotBuilder::build), so a point plot that exists can render.
#[derive(Clone)]
pub struct SVPointPlotBuilder<'values> {
    pub(super) data_source: Option<&'values dyn SVDataSource>,
    pub(super) category: Option<&'values str>,
    pub(super) value: Option<&'values str>,
    pub(super) series: Option<&'values str>,
    pub(super) estimator: SVAggregateMethod,
    pub(super) error: SVAggregateErrorKind,
    pub(super) confidence_level: f64,
    pub(super) ordering: SVPointOrdering,
    pub(super) missing_value_policy: SVPointMissingValuePolicy,
    pub(super) orientation: SVBarOrientation,
    pub(super) marker: SVDotShape,
    pub(super) caps: bool,
    pub(super) cap_width: f32,
    pub(super) join: bool,
    pub(super) line_width: Option<SVLength>,
    pub(super) dash: Option<SVDashPattern>,
    pub(super) colour: Option<SVColour>,
    pub(super) title: Option<&'values str>,
    pub(super) x_title: Option<&'values str>,
    pub(super) y_title: Option<&'values str>,
    pub(super) viewport: SVViewport,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVPointPlotBuilder<'_> {
    /// Starts a point plot with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            category: None,
            value: None,
            series: None,
            estimator: SVAggregateMethod::Mean,
            error: SVAggregateErrorKind::StandardDeviation,
            confidence_level: 0.95,
            ordering: SVPointOrdering::Data,
            missing_value_policy: SVPointMissingValuePolicy::Omit,
            orientation: SVBarOrientation::Vertical,
            marker: SVDotShape::Circle,
            caps: true,
            cap_width: 0.1,
            join: false,
            line_width: None,
            dash: None,
            colour: None,
            title: None,
            x_title: None,
            y_title: None,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVPointPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
