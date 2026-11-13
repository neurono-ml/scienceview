//! The builder value and its documented defaults.

use super::super::options::{SVStepDuplicatePolicy, SVStepPosition};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_scale::SVScaleKind;
use scienceview_core::sv_stat::SVStat;

/// Builds an [`SVStepPlot`](super::super::SVStepPlot).
///
/// The builder is the only public constructor. `backend(...)` is required:
/// there is no automatic backend, so the rendering form is always a deliberate
/// choice. `execution_mode(...)` defaults to [`SVExecutionMode::Automatic`].
#[derive(Clone)]
pub struct SVStepPlotBuilder<'source> {
    pub(super) data_source: Option<&'source dyn SVDataSource>,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) series: Option<String>,
    pub(super) step_position: SVStepPosition,
    pub(super) duplicate_policy: SVStepDuplicatePolicy,
    pub(super) line_width: Option<SVLength>,
    pub(super) dash: Option<SVDashPattern>,
    pub(super) markers: bool,
    pub(super) aggregate: Option<SVStat>,
    pub(super) viewport: SVViewport,
    pub(super) x_scale_kind: SVScaleKind,
    pub(super) title: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVStepPlotBuilder<'_> {
    /// Starts a step plot with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            x: None,
            y: None,
            series: None,
            step_position: SVStepPosition::default(),
            duplicate_policy: SVStepDuplicatePolicy::default(),
            line_width: None,
            dash: None,
            markers: true,
            aggregate: None,
            viewport: SVViewport::new(640.0, 400.0),
            x_scale_kind: SVScaleKind::Continuous,
            title: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
        }
    }
}

impl Default for SVStepPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
