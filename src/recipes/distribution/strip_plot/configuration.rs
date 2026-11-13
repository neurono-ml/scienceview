//! The validated configuration a built strip plot carries.

use super::SVStripMissingValuePolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};
use crate::sv_move::SVJitterMode;

pub(super) const SV_STRIP_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_STRIP_VIEWPORT_HEIGHT: f64 = 430.0;
pub(super) const SV_STRIP_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_STRIP_PLOT_TOP: f64 = 56.0;
pub(super) const SV_STRIP_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_STRIP_PLOT_HEIGHT: f64 = 272.0;

/// The validated configuration a built strip plot carries.
#[derive(Clone)]
pub(super) struct SVStripConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) value: String,
    pub(super) category: String,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) jitter: f32,
    pub(super) jitter_mode: SVJitterMode,
    pub(super) seed: Option<u64>,
    pub(super) marker: SVDotShape,
    pub(super) alpha: f32,
    pub(super) size: Option<SVLength>,
    pub(super) orientation: SVBarOrientation,
    pub(super) missing_value_policy: SVStripMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) value_label: Option<String>,
    pub(super) category_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_observations: usize,
    pub(super) representation: SVRepresentation,
}
