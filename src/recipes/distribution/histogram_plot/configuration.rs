//! The validated configuration a built histogram carries.

use super::SVBarElement;
use super::SVHistogramMissingValuePolicy;
use super::SVHistogramMultiple;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVBarOrientation;
use crate::sv_stat::SVNormalisation;

/// The logical canvas the recipe solves its plot area inside.
pub(super) const SV_HISTOGRAM_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_HISTOGRAM_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_HISTOGRAM_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_HISTOGRAM_PLOT_TOP: f64 = 56.0;
pub(super) const SV_HISTOGRAM_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_HISTOGRAM_PLOT_HEIGHT: f64 = 272.0;

/// The pixels a histogram spends on one bin when no bin boundary is requested.
pub(super) const SV_HISTOGRAM_PIXELS_PER_BIN: f64 = 14.0;
pub(super) const SV_HISTOGRAM_MINIMUM_BINS: usize = 4;
pub(super) const SV_HISTOGRAM_MAXIMUM_BINS: usize = 40;

/// The validated configuration a built histogram carries.
#[derive(Clone)]
pub(super) struct SVHistogramConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) value: String,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
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
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_observations: usize,
    pub(super) representation: SVRepresentation,
}
