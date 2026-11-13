//! The cell grid, solved through the layout kernel.
//!
//! The matrix is a layout concern, not a chart concern: the recipe declares one
//! row node per variable, one cell node per pair, and asks
//! [`SVLayoutTree`](crate::sv_layout::SVLayoutTree) to solve them against the
//! viewport. Every backend receives the same solved rectangles, so a cell can
//! never drift between the SVG document and the native raster.

use crate::sv_error::SVError;
use crate::sv_layout::{
    SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVResolvedRectangle, SVViewport,
};

/// The height of one cell in logical pixels.
pub(super) const SV_SPLOM_CELL_HEIGHT: f64 = 128.0;
/// The space between two neighbouring cells.
pub(super) const SV_SPLOM_CELL_GAP: f64 = 10.0;
/// The space kept around the whole grid for the title and the variable labels.
pub(super) const SV_SPLOM_GRID_PADDING: f64 = 56.0;

/// One solved cell of the matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SVSplomCellRectangle {
    /// The row index: the variable on the vertical axis.
    pub(super) row: usize,
    /// The column index: the variable on the horizontal axis.
    pub(super) column: usize,
    /// The solved rectangle in logical pixels.
    pub(super) rectangle: SVResolvedRectangle,
}

/// The solved grid: the viewport and every cell rectangle.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVSplomGrid {
    pub(super) viewport: SVViewport,
    pub(super) cells: Vec<SVSplomCellRectangle>,
}

impl SVSplomGrid {
    /// Returns the cell at one grid position, when it is part of the grid.
    pub(super) fn cell(&self, row: usize, column: usize) -> Option<SVSplomCellRectangle> {
        self.cells
            .iter()
            .find(|cell| cell.row == row && cell.column == column)
            .copied()
    }
}

/// Solves the square grid for a number of variables and a cell aspect ratio.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the variable count or the
/// aspect ratio cannot produce a solvable viewport.
pub(super) fn sv_solve_grid(
    number_of_variables: usize,
    cell_aspect: f64,
) -> Result<SVSplomGrid, SVError> {
    if number_of_variables < 2 {
        return Err(SVError::InvalidConfiguration {
            message: "a scatterplot matrix needs at least two variables".to_string(),
        });
    }
    let cell_width = SV_SPLOM_CELL_HEIGHT * cell_aspect;
    if !cell_width.is_finite() || cell_width < 1.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the cell aspect {cell_aspect} produces an unsolvable cell width {cell_width}"
            ),
        });
    }

    let span = number_of_variables as f64;
    let gaps = (number_of_variables.saturating_sub(1)) as f64 * SV_SPLOM_CELL_GAP;
    let viewport = SVViewport::new(
        2.0 * SV_SPLOM_GRID_PADDING + span * cell_width + gaps,
        2.0 * SV_SPLOM_GRID_PADDING + span * SV_SPLOM_CELL_HEIGHT + gaps,
    );

    let rows: Vec<SVLayoutNode> = (0..number_of_variables)
        .map(|row| {
            let cells = (0..number_of_variables).map(|column| {
                SVLayoutNode::new(sv_cell_name(row, column))
                    .with_width(SVLength::absolute(cell_width))
                    .with_height(SVLength::absolute(SV_SPLOM_CELL_HEIGHT))
            });
            SVLayoutNode::new(format!("sv-splom-row-{row}"))
                .with_direction(SVLayoutDirection::Horizontal)
                .with_width(SVLength::fill())
                .with_height(SVLength::absolute(SV_SPLOM_CELL_HEIGHT))
                .with_spacing(SV_SPLOM_CELL_GAP)
                .with_children(cells)
        })
        .collect();

    let root = SVLayoutNode::new("sv-splom-grid")
        .with_direction(SVLayoutDirection::Vertical)
        .with_width(SVLength::absolute(viewport.width()))
        .with_height(SVLength::absolute(viewport.height()))
        .with_padding(SV_SPLOM_GRID_PADDING)
        .with_spacing(SV_SPLOM_CELL_GAP)
        .with_children(rows);

    let mut tree = SVLayoutTree::new(root).with_viewport(viewport);
    let resolved = tree.resolved_layout();

    let mut cells = Vec::with_capacity(number_of_variables * number_of_variables);
    for row in 0..number_of_variables {
        for column in 0..number_of_variables {
            let name = sv_cell_name(row, column);
            let node = resolved
                .find(&name)
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!("the layout solver did not place the cell `{name}`"),
                })?;
            cells.push(SVSplomCellRectangle {
                row,
                column,
                rectangle: node.rectangle(),
            });
        }
    }

    Ok(SVSplomGrid { viewport, cells })
}

fn sv_cell_name(row: usize, column: usize) -> String {
    format!("sv-splom-cell-{row}-{column}")
}
