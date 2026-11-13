//! The solved rectangles of one grid for one viewport.

use crate::sv_grid::SVGridPlacement;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};

/// The solved rectangles of one grid for one viewport.
#[derive(Clone, Debug, PartialEq)]
pub struct SVGridSolution {
    pub(super) viewport: SVViewport,
    pub(super) row_rectangles: Vec<SVResolvedRectangle>,
    pub(super) column_rectangles: Vec<SVResolvedRectangle>,
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
        let right = columns
            .iter()
            .map(|column| column.right())
            .fold(first_column.right(), |maximum, right| maximum.max(right));
        let bottom = rows
            .iter()
            .map(|row| row.bottom())
            .fold(first_row.bottom(), |maximum, bottom| maximum.max(bottom));
        SVResolvedRectangle::new(
            first_column.x,
            first_row.y,
            right - first_column.x,
            bottom - first_row.y,
        )
    }
}
