//! The validated configuration a built pair grid carries.

use super::{SVPairDiagonalKind, SVPairMissingValuePolicy, SVPairOffDiagonalKind};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVDotShape;
use crate::sv_move::SVMove;

pub(super) const SV_PAIR_VIEWPORT_WIDTH: f64 = 920.0;
pub(super) const SV_PAIR_VIEWPORT_HEIGHT: f64 = 800.0;
pub(super) const SV_PAIR_MARGIN: f64 = 44.0;
pub(super) const SV_PAIR_CELL_SPACING: f64 = 6.0;
pub(super) const SV_PAIR_LEGEND_HEIGHT: f64 = 28.0;
pub(super) const SV_PAIR_TITLE_HEIGHT: f64 = 26.0;

/// The validated configuration a built pair grid carries.
#[derive(Clone)]
pub(super) struct SVPairConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x_variables: Vec<String>,
    pub(super) y_variables: Vec<String>,
    pub(super) hue: Option<String>,
    pub(super) diagonal_kind: SVPairDiagonalKind,
    pub(super) off_diagonal_kind: SVPairOffDiagonalKind,
    pub(super) marker: SVDotShape,
    pub(super) missing_value_policy: SVPairMissingValuePolicy,
    pub(super) alpha: f32,
    pub(super) jitter: Option<SVMove>,
    pub(super) title: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
}

impl SVPairConfiguration<'_> {
    pub(super) fn variable_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.x_variables.clone();
        names.extend(self.y_variables.iter().cloned());
        if let Some(hue) = &self.hue {
            names.push(hue.clone());
        }
        names
    }

    pub(super) fn resolved_diagonal_kind(&self) -> SVPairDiagonalKind {
        match self.diagonal_kind {
            SVPairDiagonalKind::Auto => SVPairDiagonalKind::Density,
            kind => kind,
        }
    }
}
