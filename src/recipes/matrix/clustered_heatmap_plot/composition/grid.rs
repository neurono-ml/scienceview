//! The regular cell grid of the reordered matrix.

use crate::sv_layout::SVResolvedRectangle;

/// The regular cell grid of one matrix, in slot space.
pub(super) struct SVClusteredHeatmapGrid {
    plot: SVResolvedRectangle,
    cell_width: f64,
    cell_height: f64,
}

impl SVClusteredHeatmapGrid {
    pub(super) fn new(
        plot: SVResolvedRectangle,
        number_of_rows: usize,
        number_of_columns: usize,
    ) -> Self {
        Self {
            plot,
            cell_width: plot.width / number_of_columns.max(1) as f64,
            cell_height: plot.height / number_of_rows.max(1) as f64,
        }
    }

    pub(super) fn rectangle(&self, slot_row: usize, slot_column: usize) -> SVResolvedRectangle {
        SVResolvedRectangle::new(
            self.plot.x + slot_column as f64 * self.cell_width,
            self.plot.y + slot_row as f64 * self.cell_height,
            self.cell_width,
            self.cell_height,
        )
    }
}
