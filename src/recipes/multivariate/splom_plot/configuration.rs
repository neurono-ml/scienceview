//! The validated configuration a built scatterplot matrix carries.

use super::{SVSplomDiagonal, SVSplomMissingValuePolicy, SVSplomScaleMode};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVDotShape;

/// The validated configuration a built scatterplot matrix carries.
#[derive(Clone)]
pub(super) struct SVSplomConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) variables: Vec<String>,
    pub(super) colour: Option<String>,
    pub(super) diagonal: SVSplomDiagonal,
    pub(super) marker: SVDotShape,
    pub(super) size: SVLength,
    pub(super) opacity: f32,
    pub(super) scale_mode: SVSplomScaleMode,
    pub(super) cell_aspect: f64,
    pub(super) show_upper: bool,
    pub(super) show_lower: bool,
    pub(super) show_diagonal: bool,
    pub(super) missing_value_policy: SVSplomMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_observations: usize,
    pub(super) representation: SVRepresentation,
}
