//! A linked selection expressed as a filter over one shared source column.

use crate::sv_control::SVControlRange;
use crate::sv_data::{SVColumnView, SVDataSource};
use crate::sv_error::SVError;

/// One column and the interval a selection keeps.
#[derive(Clone, Debug, PartialEq)]
pub struct SVSelectionConstraint {
    column: String,
    range: SVControlRange,
}

impl SVSelectionConstraint {
    /// Pairs a column name with an interval.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the column name is empty.
    pub fn new(column: impl Into<String>, range: SVControlRange) -> Result<Self, SVError> {
        let column = column.into();
        if column.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a selection constraint needs a non-empty column name".to_string(),
            });
        }
        Ok(Self { column, range })
    }

    /// Returns the column name.
    pub fn column(&self) -> &str {
        &self.column
    }

    /// Returns the selected interval.
    pub fn range(&self) -> SVControlRange {
        self.range
    }
}

/// A selection over one shared source, shared by every linked panel.
///
/// The filter is a *description*, never a copy: [`selected_indices`] resolves
/// each column once into its current representation and borrows the values, so
/// a dense, sparse or streaming source is filtered without being densified or
/// duplicated.
///
/// [`selected_indices`]: SVSelectionFilter::selected_indices
#[derive(Clone, Debug, PartialEq)]
pub struct SVSelectionFilter {
    source_identifier: String,
    horizontal: SVSelectionConstraint,
    vertical: Option<SVSelectionConstraint>,
}

impl SVSelectionFilter {
    /// Builds a filter over a horizontal interval, with an optional vertical one.
    pub fn new(
        source_identifier: impl Into<String>,
        horizontal: SVSelectionConstraint,
        vertical: Option<SVSelectionConstraint>,
    ) -> Result<Self, SVError> {
        let source_identifier = source_identifier.into();
        if source_identifier.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a selection filter needs a non-empty source identifier".to_string(),
            });
        }
        Ok(Self {
            source_identifier,
            horizontal,
            vertical,
        })
    }

    /// Returns the shared source the filter applies to.
    pub fn source_identifier(&self) -> &str {
        &self.source_identifier
    }

    /// Returns the horizontal constraint.
    pub fn horizontal(&self) -> &SVSelectionConstraint {
        &self.horizontal
    }

    /// Returns the vertical constraint, when the selection is two-dimensional.
    pub fn vertical(&self) -> Option<&SVSelectionConstraint> {
        self.vertical.as_ref()
    }

    /// Returns `true` when the row at `index` falls inside the filter.
    ///
    /// # Errors
    ///
    /// Propagates the column resolution errors of the source.
    pub fn matches(&self, source: &dyn SVDataSource, index: usize) -> Result<bool, SVError> {
        Ok(self.selected_indices(source)?.contains(&index))
    }

    /// Returns the selected row indices, computed without copying the data.
    ///
    /// # Errors
    ///
    /// Propagates the column resolution errors of the source.
    pub fn selected_indices(&self, source: &dyn SVDataSource) -> Result<Vec<usize>, SVError> {
        let mut horizontal_snapshot = source.column(self.horizontal.column())?;
        let horizontal_view = horizontal_snapshot.column_view();
        let mut vertical_snapshot = match &self.vertical {
            Some(vertical) => Some(source.column(vertical.column())?),
            None => None,
        };
        let vertical_view = match (&self.vertical, vertical_snapshot.as_mut()) {
            (Some(vertical), Some(snapshot)) => Some((vertical, snapshot.column_view())),
            _ => None,
        };

        let length = horizontal_view.length();
        let mut selected = Vec::new();
        for index in 0..length {
            if !sv_view_contains(&horizontal_view, self.horizontal.range(), index) {
                continue;
            }
            let vertical_matches = match &vertical_view {
                Some((vertical, view)) => sv_view_contains(view, vertical.range(), index),
                None => true,
            };
            if vertical_matches {
                selected.push(index);
            }
        }
        Ok(selected)
    }

    /// Returns the number of selected rows.
    ///
    /// # Errors
    ///
    /// Propagates the column resolution errors of the source.
    pub fn selected_count(&self, source: &dyn SVDataSource) -> Result<usize, SVError> {
        Ok(self.selected_indices(source)?.len())
    }
}

fn sv_view_contains(view: &SVColumnView<'_>, range: SVControlRange, index: usize) -> bool {
    view.value_at(index)
        .is_some_and(|value| range.contains(value))
}
