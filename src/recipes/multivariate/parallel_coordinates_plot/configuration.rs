//! The validated configuration a built parallel-coordinates plot carries.

use super::options::{
    SVDimension, SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;

/// The logical canvas the recipe solves its plot area inside.
pub(super) const SV_PARALLEL_COORDINATES_VIEWPORT_WIDTH: f64 = 720.0;
pub(super) const SV_PARALLEL_COORDINATES_VIEWPORT_HEIGHT: f64 = 420.0;
pub(super) const SV_PARALLEL_COORDINATES_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_PARALLEL_COORDINATES_PLOT_TOP: f64 = 88.0;
pub(super) const SV_PARALLEL_COORDINATES_PLOT_WIDTH: f64 = 608.0;
pub(super) const SV_PARALLEL_COORDINATES_PLOT_HEIGHT: f64 = 280.0;

/// The validated configuration a built plot carries.
#[derive(Clone)]
pub(super) struct SVParallelCoordinatesConfiguration<'values> {
    pub(super) data_source: &'values dyn SVDataSource,
    pub(super) variables: Vec<&'values str>,
    pub(super) colour: Option<&'values str>,
    pub(super) line_width: SVLength,
    pub(super) opacity: f32,
    pub(super) dimensions: Vec<SVDimension<'values>>,
    pub(super) range: Option<Vec<(f64, f64)>>,
    pub(super) label_angle: f32,
    pub(super) axis_label: Option<String>,
    pub(super) title: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) representation: SVRepresentation,
    pub(super) number_of_observations: usize,
    pub(super) normalization: SVParallelCoordinatesNormalization,
    pub(super) missing: SVParallelCoordinatesMissingPolicy,
}
