//! The validated configuration a built joint plot carries.

use super::SVJointKind;
use super::SVJointMarginal;
use super::SVJointMissingValuePolicy;
use super::SVJointRegressionModel;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVDotShape;

pub(super) const SV_JOINT_VIEWPORT_WIDTH: f64 = 760.0;
pub(super) const SV_JOINT_VIEWPORT_HEIGHT: f64 = 600.0;
pub(super) const SV_JOINT_LEFT_INSET: f64 = 64.0;
pub(super) const SV_JOINT_TOP_INSET: f64 = 40.0;
pub(super) const SV_JOINT_BOTTOM_INSET: f64 = 52.0;
pub(super) const SV_JOINT_MARGINAL_EXTENT: f64 = 104.0;
pub(super) const SV_JOINT_GAP: f64 = 8.0;
pub(super) const SV_JOINT_HISTOGRAM_BIN_COUNT: usize = 24;
pub(super) const SV_JOINT_HEXBIN_GRID_SIZE: usize = 16;
pub(super) const SV_JOINT_CONTOUR_LEVELS: usize = 5;

/// The validated configuration a built joint plot carries.
#[derive(Clone)]
pub(super) struct SVJointConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x: String,
    pub(super) y: String,
    pub(super) series: Option<String>,
    pub(super) kind: SVJointKind,
    pub(super) marginal_kind: SVJointMarginal,
    pub(super) marker: SVDotShape,
    pub(super) alpha: f32,
    pub(super) regression_model: SVJointRegressionModel,
    pub(super) missing_value_policy: SVJointMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
}
