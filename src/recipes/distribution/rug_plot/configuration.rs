//! The validated configuration a built rug plot carries.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVBarOrientation;

pub(super) const SV_RUG_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_RUG_VIEWPORT_HEIGHT: f64 = 430.0;
pub(super) const SV_RUG_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_RUG_PLOT_TOP: f64 = 56.0;
pub(super) const SV_RUG_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_RUG_PLOT_HEIGHT: f64 = 272.0;
pub(super) const SV_RUG_MARGIN: f64 = 6.0;

/// The validated configuration a built rug plot carries.
#[derive(Clone)]
pub(super) struct SVRugConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) value: String,
    pub(super) series: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) orientation: SVBarOrientation,
    pub(super) height: f32,
    pub(super) alpha: f32,
    pub(super) line_width: Option<SVLength>,
    pub(super) title: Option<String>,
    pub(super) value_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_observations: usize,
    pub(super) representation: SVRepresentation,
}
