//! The grid shape and the layout solution shared by every panel.

use super::SVGridPlacement;
use crate::sv_error::SVError;
use crate::sv_layout::{
    SVLayoutDirection, SVLayoutNode, SVLayoutTree, SVLength, SVResolvedRectangle, SVViewport,
};

/// The grid a dashboard places its panels in.
///
/// The grid owns shape and spacing only. Cell rectangles are solved through the
/// layout kernel, which keeps a dashboard cell on the same layout contract as a
/// figure: the viewport is the only environment input and a viewport change
/// re-solves every cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVDashboardGrid {
    number_of_rows: usize,
    number_of_columns: usize,
    horizontal_gap: f64,
    vertical_gap: f64,
}

impl SVDashboardGrid {
    /// Declares a grid with no spacing.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidDashboardGrid`] when either count is zero.
    pub fn new(number_of_rows: usize, number_of_columns: usize) -> Result<Self, SVError> {
        if number_of_rows == 0 || number_of_columns == 0 {
            return Err(SVError::InvalidDashboardGrid {
                message: format!(
                    "a dashboard grid needs at least one row and one column, received {number_of_rows}x{number_of_columns}"
                ),
            });
        }
        Ok(Self {
            number_of_rows,
            number_of_columns,
            horizontal_gap: 0.0,
            vertical_gap: 0.0,
        })
    }

    /// Sets the space between two columns.
    #[must_use]
    pub fn with_horizontal_gap(mut self, horizontal_gap: f64) -> Self {
        self.horizontal_gap = horizontal_gap;
        self
    }

    /// Sets the space between two rows.
    #[must_use]
    pub fn with_vertical_gap(mut self, vertical_gap: f64) -> Self {
        self.vertical_gap = vertical_gap;
        self
    }

    /// Returns the number of rows.
    pub fn number_of_rows(&self) -> usize {
        self.number_of_rows
    }

    /// Returns the number of columns.
    pub fn number_of_columns(&self) -> usize {
        self.number_of_columns
    }

    /// Returns the space between two columns.
    pub fn horizontal_gap(&self) -> f64 {
        self.horizontal_gap
    }

    /// Returns the space between two rows.
    pub fn vertical_gap(&self) -> f64 {
        self.vertical_gap
    }

    /// Solves every row and column against a viewport.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidDashboardGrid`] for a negative or non-finite
    /// gap.
    pub fn solve(&self, viewport: SVViewport) -> Result<SVGridSolution, SVError> {
        self.validate_spacing()?;

        let mut root = SVLayoutNode::new("dashboard")
            .with_width(SVLength::fill())
            .with_height(SVLength::fill())
            .with_direction(SVLayoutDirection::Vertical)
            .with_spacing(self.vertical_gap);
        for row in 0..self.number_of_rows {
            let mut row_node = SVLayoutNode::new(format!("row-{row}"))
                .with_width(SVLength::fill())
                .with_height(SVLength::fill())
                .with_direction(SVLayoutDirection::Horizontal)
                .with_spacing(self.horizontal_gap);
            for column in 0..self.number_of_columns {
                row_node = row_node.child(
                    SVLayoutNode::new(format!("cell-{row}-{column}"))
                        .with_width(SVLength::fill())
                        .with_height(SVLength::fill()),
                );
            }
            root = root.child(row_node);
        }

        let mut tree = SVLayoutTree::new(root).with_viewport(viewport);
        let layout = tree.resolved_layout();
        let row_rectangles = layout
            .root()
            .children()
            .iter()
            .map(|row| row.rectangle())
            .collect::<Vec<_>>();
        let column_rectangles = layout
            .root()
            .children()
            .first()
            .map(|row| {
                row.children()
                    .iter()
                    .map(|column| column.rectangle())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(SVGridSolution {
            viewport,
            row_rectangles,
            column_rectangles,
        })
    }

    fn validate_spacing(&self) -> Result<(), SVError> {
        let valid = |gap: f64| gap.is_finite() && gap >= 0.0;
        if !valid(self.horizontal_gap) || !valid(self.vertical_gap) {
            return Err(SVError::InvalidDashboardGrid {
                message: format!(
                    "grid gaps must be finite and non-negative, received horizontal {} and vertical {}",
                    self.horizontal_gap, self.vertical_gap
                ),
            });
        }
        Ok(())
    }
}

/// The solved rectangles of one grid for one viewport.
#[derive(Clone, Debug, PartialEq)]
pub struct SVGridSolution {
    viewport: SVViewport,
    row_rectangles: Vec<SVResolvedRectangle>,
    column_rectangles: Vec<SVResolvedRectangle>,
}

impl SVGridSolution {
    /// Returns the viewport the grid was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the solved rectangle of every row.
    pub fn row_rectangles(&self) -> &[SVResolvedRectangle] {
        &self.row_rectangles
    }

    /// Returns the solved rectangle of every column.
    pub fn column_rectangles(&self) -> &[SVResolvedRectangle] {
        &self.column_rectangles
    }

    /// Returns the rectangle a placement covers.
    ///
    /// The rectangle is the union of the rows and columns the placement spans,
    /// so a spanning cell occupies exactly the space of the cells it covers.
    pub fn rectangle_for(&self, placement: &SVGridPlacement) -> SVResolvedRectangle {
        let rows = self
            .row_rectangles
            .iter()
            .skip(placement.row())
            .take(placement.row_span());
        let columns = self
            .column_rectangles
            .iter()
            .skip(placement.column())
            .take(placement.column_span());
        let rows = rows.collect::<Vec<_>>();
        let columns = columns.collect::<Vec<_>>();
        let Some(first_row) = rows.first() else {
            return SVResolvedRectangle::default();
        };
        let Some(first_column) = columns.first() else {
            return SVResolvedRectangle::default();
        };
        let right = columns.iter().map(|column| column.right()).fold(
            first_column.right(),
            |maximum, right| maximum.max(right),
        );
        let bottom = rows.iter().map(|row| row.bottom()).fold(
            first_row.bottom(),
            |maximum, bottom| maximum.max(bottom),
        );
        SVResolvedRectangle::new(
            first_column.x,
            first_row.y,
            right - first_column.x,
            bottom - first_row.y,
        )
    }
}
