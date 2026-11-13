//! The builder value and its starting state.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVFormatSpec;
use crate::sv_recipes::sv_matrix::sv_heatmap_plot::{
    SVHeatmapMatrix, SVHeatmapOrientation, SVMissingValue,
};
use crate::sv_scale::{SVColourMap, sv_default_colour_map};

/// Builds a [`SVHeatmapPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. A heatmap is configured either from a direct
/// [`SVHeatmapMatrix`] with `matrix(...)` or from a long table with
/// `data_source(...)` plus `row(...)`, `column(...)` and `value(...)`.
#[derive(Clone)]
pub struct SVHeatmapPlotBuilder<'data> {
    pub(super) data_source: Option<&'data dyn SVDataSource>,
    pub(super) row: Option<&'data str>,
    pub(super) column: Option<&'data str>,
    pub(super) value: Option<&'data str>,
    pub(super) matrix: Option<SVHeatmapMatrix<'data>>,
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
    pub(super) backend_kind: Option<SVBackendKind>,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVHeatmapPlotBuilder<'_> {
    /// Starts a heatmap with no data, no matrix and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            row: None,
            column: None,
            value: None,
            matrix: None,
            row_labels: None,
            column_labels: None,
            colour_map: sv_default_colour_map(),
            centre: None,
            robust: false,
            annot: false,
            annot_format: SVFormatSpec::Automatic,
            line_width: 0.0,
            square: false,
            colour_bar: true,
            missing: SVMissingValue::Transparent,
            orientation: SVHeatmapOrientation::RowsAlongVertical,
            title: None,
            x_label: None,
            y_label: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }
}

impl Default for SVHeatmapPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
