//! The option surface of the heatmap builder, one method per setting.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVFormatSpec;
use crate::sv_recipes::sv_matrix::sv_heatmap_plot::{
    SVHeatmapMatrix, SVHeatmapOrientation, SVMissingValue,
};
use crate::sv_scale::SVColourMap;

use super::builder_state::SVHeatmapPlotBuilder;

impl<'data> SVHeatmapPlotBuilder<'data> {
    /// Selects the rendering backend. Required.
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to `Automatic`.
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Supplies a direct, row-major matrix.
    pub fn matrix(mut self, matrix: SVHeatmapMatrix<'data>) -> Self {
        self.matrix = Some(matrix);
        self
    }

    /// Supplies the long table the matrix is pivoted from.
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the row-key column of a long table.
    pub fn row(mut self, row: &'data str) -> Self {
        self.row = Some(row);
        self
    }

    /// Names the column-key column of a long table.
    pub fn column(mut self, column: &'data str) -> Self {
        self.column = Some(column);
        self
    }

    /// Names the value column of a long table.
    pub fn value(mut self, value: &'data str) -> Self {
        self.value = Some(value);
        self
    }

    /// Overrides the row labels of a direct matrix.
    pub fn row_labels(mut self, labels: &'data [&'data str]) -> Self {
        self.row_labels = Some(labels);
        self
    }

    /// Overrides the column labels of a direct matrix.
    pub fn column_labels(mut self, labels: &'data [&'data str]) -> Self {
        self.column_labels = Some(labels);
        self
    }

    /// Sets the continuous colour map. Defaults to the theme's default map.
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = colour_map;
        self
    }

    /// Centres the colour map on a value.
    pub fn centre(mut self, centre: f64) -> Self {
        self.centre = Some(centre);
        self
    }

    /// Derives the colour domain from the 2nd and 98th percentiles.
    pub fn robust(mut self, robust: bool) -> Self {
        self.robust = robust;
        self
    }

    /// Draws a formatted value inside every cell.
    pub fn annot(mut self, annot: bool) -> Self {
        self.annot = annot;
        self
    }

    /// Sets the annotation format. Defaults to `Automatic`.
    pub fn annot_format(mut self, annot_format: SVFormatSpec) -> Self {
        self.annot_format = annot_format;
        self
    }

    /// Sets the cell border width in logical pixels.
    pub fn line_width(mut self, line_width: f64) -> Self {
        self.line_width = line_width;
        self
    }

    /// Forces square cells.
    pub fn square(mut self, square: bool) -> Self {
        self.square = square;
        self
    }

    /// Draws the colour bar. Enabled by default.
    pub fn colour_bar(mut self, colour_bar: bool) -> Self {
        self.colour_bar = colour_bar;
        self
    }

    /// Selects the missing-cell rule. Defaults to `Transparent`.
    pub fn missing(mut self, missing: SVMissingValue) -> Self {
        self.missing = missing;
        self
    }

    /// Selects which axis the rows run along.
    pub fn orientation(mut self, orientation: SVHeatmapOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the figure title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the horizontal axis title.
    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    /// Sets the vertical axis title.
    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
        self
    }
}
