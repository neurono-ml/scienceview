//! The builder value and its documented defaults.

use super::super::SVBarElement;
use super::super::SVHistogramMissingValuePolicy;
use super::super::SVHistogramMultiple;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVBarOrientation;
use crate::sv_stat::SVNormalisation;

/// Builds a [`SVHistogramPlot`](super::super::SVHistogramPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVHistogramPlotBuilder::build), so a histogram that exists can
/// render.
#[derive(Clone)]
pub struct SVHistogramPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) value: Option<String>,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) weight: Option<String>,
    pub(super) bin_count: Option<usize>,
    pub(super) bin_width: Option<f64>,
    pub(super) bin_edges: Option<Vec<f64>>,
    pub(super) normalisation: SVNormalisation,
    pub(super) cumulative: bool,
    pub(super) common_normalisation: bool,
    pub(super) common_bins: bool,
    pub(super) shrink: f64,
    pub(super) element: SVBarElement,
    pub(super) orientation: SVBarOrientation,
    pub(super) multiple: SVHistogramMultiple,
    pub(super) missing_value_policy: SVHistogramMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVHistogramPlotBuilder<'_> {
    /// Starts a histogram with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            series: None,
            series_labels: Vec::new(),
            weight: None,
            bin_count: None,
            bin_width: None,
            bin_edges: None,
            normalisation: SVNormalisation::Count,
            cumulative: false,
            common_normalisation: false,
            common_bins: true,
            shrink: 1.0,
            element: SVBarElement::Bar,
            orientation: SVBarOrientation::Vertical,
            multiple: SVHistogramMultiple::Layer,
            missing_value_policy: SVHistogramMissingValuePolicy::Omit,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVHistogramPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
