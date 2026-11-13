//! The validated configuration a built swarm carries.

use super::missing_value_policy::SVSwarmMissingValuePolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};

pub(super) const SV_SWARM_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_SWARM_VIEWPORT_HEIGHT: f64 = 430.0;
pub(super) const SV_SWARM_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_SWARM_PLOT_TOP: f64 = 56.0;
pub(super) const SV_SWARM_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_SWARM_PLOT_HEIGHT: f64 = 272.0;

/// The validated configuration a built swarm carries.
#[derive(Clone)]
pub(super) struct SVSwarmConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) value: String,
    pub(super) category: String,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) marker: SVDotShape,
    pub(super) size: Option<SVLength>,
    pub(super) orientation: SVBarOrientation,
    pub(super) swarm_width: Option<f32>,
    pub(super) alpha: f32,
    pub(super) missing_value_policy: SVSwarmMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
}
