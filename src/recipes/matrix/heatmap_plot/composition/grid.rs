//! The regular cell grid of one matrix.

use super::super::SVHeatmapOrientation;
use crate::sv_layout::SVResolvedRectangle;

/// The regular cell grid of one matrix.
pub(super) struct SVHeatmapGrid {
    plot: SVResolvedRectangle,
    orientation: SVHeatmapOrientation,
    cell_width: f64,
    cell_height: f64,
    side: Option<f64>,
}

impl SVHeatmapGrid {
    pub(super) fn new(
        plot: SVResolvedRectangle,
        orientation: SVHeatmapOrientation,
        number_of_rows: usize,
        number_of_columns: usize,
        square: bool,
    ) -> Self {
        let (horizontal_slots, vertical_slots) = match orientation {
            SVHeatmapOrientation::RowsAlongVertical => (number_of_columns, number_of_rows),
            SVHeatmapOrientation::RowsAlongHorizontal => (number_of_rows, number_of_columns),
        };
        let cell_width = plot.width / horizontal_slots.max(1) as f64;
        let cell_height = plot.height / vertical_slots.max(1) as f64;
        let side = square.then(|| cell_width.min(cell_height));
        Self {
            plot,
            orientation,
            cell_width,
            cell_height,
            side,
        }
    }

    pub(super) fn rectangle(&self, row: usize, column: usize) -> SVResolvedRectangle {
        let (horizontal_slot, vertical_slot) = match self.orientation {
            SVHeatmapOrientation::RowsAlongVertical => (column, row),
            SVHeatmapOrientation::RowsAlongHorizontal => (row, column),
        };
        let (width, height) = match self.side {
            Some(side) => (side, side),
            None => (self.cell_width, self.cell_height),
        };
        let offset_x = (self.cell_width - width) / 2.0;
        let offset_y = (self.cell_height - height) / 2.0;
        SVResolvedRectangle::new(
            self.plot.x + horizontal_slot as f64 * self.cell_width + offset_x,
            self.plot.y + vertical_slot as f64 * self.cell_height + offset_y,
            width,
            height,
        )
    }
}
