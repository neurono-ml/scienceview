//! A rectangular placement inside the dashboard grid.

use crate::sv_error::SVError;

/// The grid cells one dashboard panel occupies.
///
/// A placement names its first row and column and spans at least one unit on
/// each axis. Validation against the declared grid happens when the dashboard
/// is built, so a placement that exists may still be outside a particular grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SVGridPlacement {
    row: usize,
    column: usize,
    row_span: usize,
    column_span: usize,
}

impl SVGridPlacement {
    /// Places a panel in one cell.
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            row_span: 1,
            column_span: 1,
        }
    }

    /// Extends the placement over several rows.
    #[must_use]
    pub fn with_row_span(mut self, row_span: usize) -> Self {
        self.row_span = row_span;
        self
    }

    /// Extends the placement over several columns.
    #[must_use]
    pub fn with_column_span(mut self, column_span: usize) -> Self {
        self.column_span = column_span;
        self
    }

    /// Returns the first row.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the first column.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns the number of rows the placement occupies.
    pub fn row_span(&self) -> usize {
        self.row_span
    }

    /// Returns the number of columns the placement occupies.
    pub fn column_span(&self) -> usize {
        self.column_span
    }

    /// Returns `true` when the two placements share at least one grid unit.
    ///
    /// Two panels that overlap cannot both be visible, so the dashboard rejects
    /// them instead of letting declaration order decide.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.row < other.row.saturating_add(other.row_span)
            && other.row < self.row.saturating_add(self.row_span)
            && self.column < other.column.saturating_add(other.column_span)
            && other.column < self.column.saturating_add(self.column_span)
    }

    /// Validates the placement against a grid shape.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::DashboardPlacementOutOfBounds`] when the span is
    /// empty or reaches past the declared rows or columns.
    pub fn validate(&self, panel: &str, rows: usize, columns: usize) -> Result<(), SVError> {
        let reaches_past_rows = self.row_span == 0
            || self
                .row
                .checked_add(self.row_span)
                .is_none_or(|end| end > rows);
        let reaches_past_columns = self.column_span == 0
            || self
                .column
                .checked_add(self.column_span)
                .is_none_or(|end| end > columns);
        if reaches_past_rows || reaches_past_columns {
            return Err(SVError::DashboardPlacementOutOfBounds {
                panel: panel.to_owned(),
                row: self.row,
                column: self.column,
                row_span: self.row_span,
                column_span: self.column_span,
                rows,
                columns,
            });
        }
        Ok(())
    }
}
