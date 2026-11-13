//! The validated configuration a built parallel categories plot carries.

use super::SVCategoryOrder;
use super::SVParallelCategoriesColourMode;
use super::SVParallelCategoriesMissingValuePolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;

/// The logical canvas the recipe solves its plot area inside.
pub(super) const SV_PARALLEL_CATEGORIES_VIEWPORT_WIDTH: f64 = 720.0;
pub(super) const SV_PARALLEL_CATEGORIES_VIEWPORT_HEIGHT: f64 = 460.0;
pub(super) const SV_PARALLEL_CATEGORIES_PLOT_LEFT: f64 = 110.0;
pub(super) const SV_PARALLEL_CATEGORIES_PLOT_TOP: f64 = 92.0;
pub(super) const SV_PARALLEL_CATEGORIES_PLOT_WIDTH: f64 = 500.0;
pub(super) const SV_PARALLEL_CATEGORIES_PLOT_HEIGHT: f64 = 320.0;

/// The validated configuration a built parallel categories plot carries.
#[derive(Clone)]
pub(super) struct SVParallelCategoriesConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) variables: Vec<String>,
    pub(super) variable_labels: Vec<String>,
    pub(super) category_labels: Vec<(String, Vec<String>)>,
    pub(super) colour: Option<String>,
    pub(super) colour_mode: SVParallelCategoriesColourMode,
    pub(super) opacity: f32,
    pub(super) band_gap: f32,
    pub(super) category_order: SVCategoryOrder,
    pub(super) label_angle: f32,
    pub(super) missing_value_policy: SVParallelCategoriesMissingValuePolicy,
    pub(super) title: Option<String>,
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
