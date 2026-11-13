//! The builder value and its documented defaults.

use super::super::SVCategoryOrder;
use super::super::SVParallelCategoriesColourMode;
use super::super::SVParallelCategoriesMissingValuePolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;

/// Builds a [`SVParallelCategoriesPlot`](super::super::SVParallelCategoriesPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVParallelCategoriesPlotBuilder::build), so a plot that exists can
/// render.
#[derive(Clone)]
pub struct SVParallelCategoriesPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
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
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVParallelCategoriesPlotBuilder<'_> {
    /// Starts a parallel categories plot with its documented defaults.
    pub fn new() -> Self {
        Self {
            data_source: None,
            variables: Vec::new(),
            variable_labels: Vec::new(),
            category_labels: Vec::new(),
            colour: None,
            colour_mode: SVParallelCategoriesColourMode::Count,
            opacity: 0.5,
            band_gap: 0.2,
            category_order: SVCategoryOrder::DataOrder,
            label_angle: 0.0,
            missing_value_policy: SVParallelCategoriesMissingValuePolicy::Omit,
            title: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVParallelCategoriesPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
