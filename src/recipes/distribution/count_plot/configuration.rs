//! The validated configuration a built count plot carries.

use super::SVCountMissingValuePolicy;
use super::SVCountMultiple;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::SVColour;
use crate::sv_stat::{SVCountOrder, SVNormalisation};

/// The logical canvas the recipe solves its plot area inside.
pub(super) const SV_COUNT_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_COUNT_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_COUNT_PLOT_LEFT: f64 = 72.0;
pub(super) const SV_COUNT_PLOT_TOP: f64 = 56.0;
pub(super) const SV_COUNT_PLOT_WIDTH: f64 = 468.0;
pub(super) const SV_COUNT_PLOT_HEIGHT: f64 = 272.0;

/// The validated configuration a built count plot carries.
#[derive(Clone)]
pub(super) struct SVCountConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) category: String,
    pub(super) series: Option<String>,
    pub(super) category_labels: Vec<String>,
    pub(super) series_labels: Vec<String>,
    pub(super) orientation: SVBarOrientation,
    pub(super) width: f32,
    pub(super) normalisation: SVNormalisation,
    pub(super) order: SVCountOrder,
    pub(super) colour: Option<SVColour>,
    pub(super) multiple: SVCountMultiple,
    pub(super) missing_value_policy: SVCountMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_observations: usize,
    pub(super) representation: SVRepresentation,
}

/// Builds the configuration error the recipe reports.
pub(super) fn sv_invalid(message: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: message.to_string(),
    }
}
