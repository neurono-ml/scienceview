//! The validated configuration one built clustered heatmap carries.

use super::SVMissingValue;
use super::{SVClusterTree, SVHeatmapMatrix};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::SVColourMap;

/// Where the matrix comes from.
#[derive(Clone, Copy)]
pub(super) enum SVClusteredHeatmapInput<'data> {
    /// A direct, row-major dense matrix.
    Matrix(SVHeatmapMatrix<'data>),
    /// A long table pivoted into a matrix.
    Table {
        data_source: &'data dyn SVDataSource,
        row: &'data str,
        column: &'data str,
        value: &'data str,
    },
}

/// Everything one built clustered heatmap needs to compose.
#[derive(Clone)]
pub(super) struct SVClusteredHeatmapConfiguration<'data> {
    pub(super) input: SVClusteredHeatmapInput<'data>,
    pub(super) row_order: &'data [usize],
    pub(super) column_order: &'data [usize],
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
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) representation: SVRepresentation,
    pub(super) number_of_cells: usize,
}

/// The logical canvas the clustered heatmap solves its regions inside.
pub(super) const SV_CLUSTERED_VIEWPORT_WIDTH: f64 = 760.0;
pub(super) const SV_CLUSTERED_VIEWPORT_HEIGHT: f64 = 600.0;
pub(super) const SV_CLUSTERED_ROW_DENDROGRAM_WIDTH: f64 = 84.0;
pub(super) const SV_CLUSTERED_COLUMN_DENDROGRAM_HEIGHT: f64 = 84.0;
pub(super) const SV_CLUSTERED_PLOT_LEFT: f64 = 132.0;
pub(super) const SV_CLUSTERED_PLOT_TOP: f64 = 104.0;
pub(super) const SV_CLUSTERED_PLOT_WIDTH: f64 = 440.0;
pub(super) const SV_CLUSTERED_PLOT_HEIGHT: f64 = 400.0;
