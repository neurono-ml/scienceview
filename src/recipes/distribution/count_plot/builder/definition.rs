//! The builder value and its documented defaults.

use super::super::SVCountMissingValuePolicy;
use super::super::SVCountMultiple;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::SVColour;
use crate::sv_stat::{SVCountOrder, SVNormalisation};

/// Builds a [`SVCountPlot`](super::super::SVCountPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVCountPlotBuilder::build), so a count plot that exists can
/// render.
#[derive(Clone)]
pub struct SVCountPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) category: Option<String>,
    pub(super) series: Option<String>,
    pub(super) category_labels: Vec<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) orientation: SVBarOrientation,
    pub(super) width: f32,
    pub(super) normalisation: SVNormalisation,
    pub(super) order: SVCountOrder,
    pub(super) colour: Option<SVColour>,
    pub(super) multiple: SVCountMultiple,
    pub(super) missing_value_policy: SVCountMissingValuePolicy,
    pub(super) weight: Option<String>,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVCountPlotBuilder<'_> {
    /// Starts a count plot with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            category: None,
            series: None,
            category_labels: Vec::new(),
            series_labels: Vec::new(),
            orientation: SVBarOrientation::Vertical,
            width: 0.8,
            normalisation: SVNormalisation::Count,
            order: SVCountOrder::DataOrder,
            colour: None,
            multiple: SVCountMultiple::Layer,
            missing_value_policy: SVCountMissingValuePolicy::Omit,
            weight: None,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVCountPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
