//! The validated configuration a built scatter carries.

use super::{SVScatterLegendPlacement, SVScatterSizeScale};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVDotShape;
use crate::sv_move::SVMove;
use crate::sv_scale::SVColour;

pub(super) const SV_SCATTER_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_SCATTER_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_SCATTER_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_SCATTER_PLOT_TOP: f64 = 56.0;
pub(super) const SV_SCATTER_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_SCATTER_PLOT_HEIGHT: f64 = 272.0;

/// The validated configuration a built scatter carries.
#[derive(Clone)]
pub(super) struct SVScatterConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x: String,
    pub(super) y: String,
    pub(super) series: Option<String>,
    pub(super) size: Option<String>,
    pub(super) colour: Option<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) marker: SVDotShape,
    pub(super) size_scale: SVScatterSizeScale,
    pub(super) alpha: f32,
    pub(super) edge_colour: Option<SVColour>,
    pub(super) edge_width: SVLength,
    pub(super) jitter: Option<SVMove>,
    pub(super) legend: SVScatterLegendPlacement,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
    pub(super) base_marker_size: f64,
}
