//! The public binned density field.

/// One binned density field.
///
/// `values` is row-major over the `y` axis: the cell at column `x_index` and
/// row `y_index` is at `y_index * x_cell_count + x_index`. The value is the
/// accumulated observation weight inside the cell, normalised by the chart's
/// configured [`SVNormalisation`](crate::sv_stat::SVNormalisation).
#[derive(Clone, Debug, PartialEq)]
pub struct SVDensityHeatmapGrid {
    x_edges: Vec<f64>,
    y_edges: Vec<f64>,
    values: Vec<f64>,
    series: Option<f64>,
    total_weight: f64,
}

impl SVDensityHeatmapGrid {
    pub(crate) fn new(
        x_edges: Vec<f64>,
        y_edges: Vec<f64>,
        values: Vec<f64>,
        series: Option<f64>,
        total_weight: f64,
    ) -> Self {
        Self {
            x_edges,
            y_edges,
            values,
            series,
            total_weight,
        }
    }

    /// Returns the horizontal bin edges, one more than the cell count.
    pub fn x_edges(&self) -> &[f64] {
        &self.x_edges
    }

    /// Returns the vertical bin edges, one more than the cell count.
    pub fn y_edges(&self) -> &[f64] {
        &self.y_edges
    }

    /// Returns the value of every cell, row-major over `y`.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Returns the number of horizontal cells.
    pub fn x_cell_count(&self) -> usize {
        self.x_edges.len().saturating_sub(1)
    }

    /// Returns the number of vertical cells.
    pub fn y_cell_count(&self) -> usize {
        self.y_edges.len().saturating_sub(1)
    }

    /// Returns the total number of cells.
    pub fn number_of_cells(&self) -> usize {
        self.values.len()
    }

    /// Returns the value of one cell.
    pub fn value_at(&self, x_index: usize, y_index: usize) -> f64 {
        let position = y_index * self.x_cell_count() + x_index;
        self.values.get(position).copied().unwrap_or(0.0)
    }

    /// Returns the series code this grid describes, when the chart is grouped.
    pub fn series(&self) -> Option<f64> {
        self.series
    }

    /// Returns the total observation weight inside the grid.
    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }

    /// Returns `true` when no observation landed in the cell.
    pub fn is_empty_at(&self, x_index: usize, y_index: usize) -> bool {
        self.value_at(x_index, y_index) == 0.0
    }
}
