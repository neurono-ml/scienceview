//! The validated configuration a built heatmap carries.

use super::{SVHeatmapMatrix, SVHeatmapOrientation, SVMissingValue};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVFormatSpec;
use crate::sv_scale::SVColourMap;

/// The logical canvas the recipe solves its plot area inside.
pub(super) const SV_HEATMAP_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_HEATMAP_VIEWPORT_HEIGHT: f64 = 480.0;
pub(super) const SV_HEATMAP_PLOT_LEFT: f64 = 88.0;
pub(super) const SV_HEATMAP_PLOT_TOP: f64 = 56.0;
pub(super) const SV_HEATMAP_PLOT_HEIGHT: f64 = 344.0;
pub(super) const SV_HEATMAP_PLOT_WIDTH_WITH_BAR: f64 = 392.0;
pub(super) const SV_HEATMAP_PLOT_WIDTH_WITHOUT_BAR: f64 = 480.0;

/// Where the matrix comes from.
#[derive(Clone, Copy)]
pub(super) enum SVHeatmapInput<'data> {
    /// A direct, row-major matrix.
    Matrix(SVHeatmapMatrix<'data>),
    /// A long table pivoted into a matrix.
    Table {
        data_source: &'data dyn SVDataSource,
        row: &'data str,
        column: &'data str,
        value: &'data str,
    },
}

/// The validated configuration a built heatmap carries.
#[derive(Clone)]
pub(super) struct SVHeatmapConfiguration<'data> {
    pub(super) input: SVHeatmapInput<'data>,
    pub(super) row_labels: Option<&'data [&'data str]>,
    pub(super) column_labels: Option<&'data [&'data str]>,
    pub(super) colour_map: SVColourMap,
    pub(super) centre: Option<f64>,
    pub(super) robust: bool,
    pub(super) annot: bool,
    pub(super) annot_format: SVFormatSpec,
    pub(super) line_width: f64,
    pub(super) square: bool,
    pub(super) colour_bar: bool,
    pub(super) missing: SVMissingValue,
    pub(super) orientation: SVHeatmapOrientation,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) representation: SVRepresentation,
    pub(super) number_of_cells: usize,
}
