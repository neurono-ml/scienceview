//! The builder value and its starting state.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::{
    SVClusterTree, SVHeatmapMatrix, SVMissingValue,
};
use crate::sv_scale::{SVColourMap, sv_default_colour_map};

/// Builds a [`SVClusteredHeatmapPlot`](super::super::SVClusteredHeatmapPlot).
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. The clustering is an input: `row_order(...)`
/// and `column_order(...)` are required, and `row_linkage(...)` /
/// `column_linkage(...)` supply the dendrograms.
#[derive(Clone)]
pub struct SVClusteredHeatmapPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) row: Option<&'data str>,
    pub(super) column: Option<&'data str>,
    pub(super) value: Option<&'data str>,
    pub(super) matrix: Option<SVHeatmapMatrix<'data>>,
    pub(super) row_order: Option<&'data [usize]>,
    pub(super) column_order: Option<&'data [usize]>,
    pub(super) row_tree: Option<SVClusterTree<'data>>,
    pub(super) column_tree: Option<SVClusterTree<'data>>,
    pub(super) dendrogram: bool,
    pub(super) row_labels: Option<&'data [&'data str]>,
    pub(super) column_labels: Option<&'data [&'data str]>,
    pub(super) colour_map: SVColourMap,
    pub(super) centre: Option<f64>,
    pub(super) colour_bar: bool,
    pub(super) missing: SVMissingValue,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVClusteredHeatmapPlotBuilder<'_> {
    /// Starts a clustered heatmap with no data, no order, no tree and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            row: None,
            column: None,
            value: None,
            matrix: None,
            row_order: None,
            column_order: None,
            row_tree: None,
            column_tree: None,
            dendrogram: true,
            row_labels: None,
            column_labels: None,
            colour_map: sv_default_colour_map(),
            centre: None,
            colour_bar: true,
            missing: SVMissingValue::Transparent,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVClusteredHeatmapPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
