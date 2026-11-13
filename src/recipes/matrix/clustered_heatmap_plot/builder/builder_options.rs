//! The option surface of the clustered heatmap builder.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::{
    SVClusterTree, SVHeatmapMatrix, SVMissingValue,
};
use crate::sv_scale::SVColourMap;

use super::builder_state::SVClusteredHeatmapPlotBuilder;

impl<'data> SVClusteredHeatmapPlotBuilder<'data> {
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

    /// Supplies a direct, row-major dense matrix.
    pub fn matrix(mut self, matrix: SVHeatmapMatrix<'data>) -> Self {
        self.matrix = Some(matrix);
        self
    }

    /// Supplies the long table the matrix is pivoted from.
    ///
    /// A sparse value column is read through its stored entries only, so a
    /// sparse matrix is never densified.
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

    /// Supplies the precomputed row order, a permutation of the row indices.
    ///
    /// This is the clustering input: the recipe renders this order, it never
    /// computes it.
    pub fn row_order(mut self, row_order: &'data [usize]) -> Self {
        self.row_order = Some(row_order);
        self
    }

    /// Supplies the precomputed column order, a permutation of the column
    /// indices.
    pub fn column_order(mut self, column_order: &'data [usize]) -> Self {
        self.column_order = Some(column_order);
        self
    }

    /// Supplies the precomputed row merge tree.
    pub fn row_linkage(mut self, row_linkage: SVClusterTree<'data>) -> Self {
        self.row_tree = Some(row_linkage);
        self
    }

    /// Supplies the precomputed column merge tree.
    pub fn column_linkage(mut self, column_linkage: SVClusterTree<'data>) -> Self {
        self.column_tree = Some(column_linkage);
        self
    }

    /// Draws the two dendrograms. Enabled by default; requires both trees.
    pub fn dendrogram(mut self, dendrogram: bool) -> Self {
        self.dendrogram = dendrogram;
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
