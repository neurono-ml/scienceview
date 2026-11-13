//! The public binned density field.

/// One binned density field.
///
/// `densities` is row-major over the `y` axis: the cell at column `x_index` and
/// row `y_index` is at `y_index * x_cell_count + x_index`. Every cell holds a
/// probability density, so the sum of `density * cell_width * cell_height`
/// over the grid is one.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDensityGrid {
    x_edges: Vec<f64>,
    y_edges: Vec<f64>,
    densities: Vec<f64>,
    series: Option<f64>,
    total_weight: f64,
}

impl SVDensityGrid {
    pub(crate) fn new(
        x_edges: Vec<f64>,
        y_edges: Vec<f64>,
        densities: Vec<f64>,
        series: Option<f64>,
        total_weight: f64,
    ) -> Self {
        Self {
            x_edges,
            y_edges,
            densities,
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

    /// Returns the probability density of every cell, row-major over `y`.
    pub fn densities(&self) -> &[f64] {
        &self.densities
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
        self.densities.len()
    }

    /// Returns the density at one cell.
    pub fn density_at(&self, x_index: usize, y_index: usize) -> f64 {
        let position = y_index * self.x_cell_count() + x_index;
        self.densities.get(position).copied().unwrap_or(0.0)
    }

    /// Returns the series code this grid describes, when the chart is grouped.
    pub fn series(&self) -> Option<f64> {
        self.series
    }

    /// Returns the sum of the observation weights inside the grid.
    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }

    /// Returns the probability mass the density integrates to over the domain.
    pub fn integrated_probability(&self) -> f64 {
        let horizontal = self.x_cell_count();
        let mut integral = 0.0;
        for y_index in 0..self.y_cell_count() {
            let height = self.y_edges[y_index + 1] - self.y_edges[y_index];
            for x_index in 0..horizontal {
                let width = self.x_edges[x_index + 1] - self.x_edges[x_index];
                integral += self.density_at(x_index, y_index) * width * height;
            }
        }
        integral
    }
}
