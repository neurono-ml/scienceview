//! Owned columns and tables: the dense and sparse representations at the
//! Python boundary.
//!
//! Python owns its lists; the Rust surface borrows. This module is the single
//! owned copy between the two, and it implements [`SVDataSource`] so the very
//! same recipe builders consume it without knowing where the numbers came from.

use scienceview::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource,
    SVRepresentation, SVSourceSchema, SVSparseView,
};
use scienceview::sv_error::SVError;

/// The stored values of one column.
#[derive(Clone, Debug, PartialEq)]
pub enum SVPythonColumnValues {
    /// A dense column copied once from the Python sequence.
    Dense(Vec<f64>),
    /// Borrowed-at-read sparse storage, never expanded.
    Sparse {
        /// The logical positions that carry a value.
        indices: Vec<usize>,
        /// The values at those positions.
        values: Vec<f64>,
        /// The logical length, implicit zeros included.
        length: usize,
    },
}

impl SVPythonColumnValues {
    /// The logical number of values, implicit zeros included.
    pub fn length(&self) -> usize {
        match self {
            Self::Dense(values) => values.len(),
            Self::Sparse { length, .. } => *length,
        }
    }

    /// Resolves the view a kernel dispatches on.
    ///
    /// # Errors
    ///
    /// Returns the sparse validation error of the stored indices.
    pub fn view(&self) -> Result<SVColumnView<'_>, SVError> {
        match self {
            Self::Dense(values) => Ok(SVColumnView::Dense(values)),
            Self::Sparse {
                indices,
                values,
                length,
            } => Ok(SVColumnView::Sparse(SVSparseView::new(
                indices, values, *length,
            )?)),
        }
    }

    /// The representation this column carries.
    pub fn representation(&self) -> SVRepresentation {
        match self {
            Self::Dense(_) => SVRepresentation::Dense,
            Self::Sparse { .. } => SVRepresentation::Sparse,
        }
    }
}

/// One named column owned by the binding.
#[derive(Clone, Debug, PartialEq)]
pub struct SVPythonColumn {
    name: String,
    role: SVColumnRole,
    values: SVPythonColumnValues,
}

impl SVPythonColumn {
    /// Builds a column from a name, a role and its stored values.
    pub fn new(name: impl Into<String>, role: SVColumnRole, values: SVPythonColumnValues) -> Self {
        Self {
            name: name.into(),
            role,
            values,
        }
    }

    /// Builds a dense column.
    pub fn dense(name: impl Into<String>, role: SVColumnRole, values: Vec<f64>) -> Self {
        Self::new(name, role, SVPythonColumnValues::Dense(values))
    }

    /// The column name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The role the column declares.
    pub fn role(&self) -> SVColumnRole {
        self.role
    }

    /// The stored values.
    pub fn values(&self) -> &SVPythonColumnValues {
        &self.values
    }

    /// The logical length of the column.
    pub fn length(&self) -> usize {
        self.values.length()
    }
}

/// A static multi-column source owned by the binding.
///
/// Construction validates the names and the shared length once, so a table
/// that exists is always well formed and every later read is infallible apart
/// from the name lookup.
#[derive(Clone, Debug, PartialEq)]
pub struct SVPythonTable {
    columns: Vec<SVPythonColumn>,
}

impl SVPythonTable {
    /// Validates and builds a table.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::DuplicateColumnName`] for a repeated name and
    /// [`SVError::MismatchedLengths`] for a column whose logical length
    /// differs from the first one.
    pub fn new(columns: Vec<SVPythonColumn>) -> Result<Self, SVError> {
        for (position, column) in columns.iter().enumerate() {
            if columns[..position]
                .iter()
                .any(|earlier| earlier.name() == column.name())
            {
                return Err(SVError::DuplicateColumnName {
                    name: column.name().to_owned(),
                });
            }
        }
        if let Some(expected) = columns.first().map(SVPythonColumn::length) {
            for column in &columns {
                if column.length() != expected {
                    return Err(SVError::MismatchedLengths {
                        name: column.name().to_owned(),
                        expected,
                        actual: column.length(),
                    });
                }
            }
        }
        Ok(Self { columns })
    }

    /// The columns, in declaration order.
    pub fn columns(&self) -> &[SVPythonColumn] {
        &self.columns
    }

    /// The logical length every column shares.
    pub fn length(&self) -> usize {
        self.columns.first().map_or(0, SVPythonColumn::length)
    }

    /// The representation of a named column.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ColumnNotFound`] when the name is unknown.
    pub fn representation_of(&self, name: &str) -> Result<SVRepresentation, SVError> {
        self.column_named(name)
            .map(|column| column.values().representation())
    }

    /// The first column, preferring one declared with the `Y` role.
    pub fn primary_column(&self) -> Option<&SVPythonColumn> {
        self.columns
            .iter()
            .find(|column| column.role() == SVColumnRole::Y)
            .or_else(|| self.columns.first())
    }

    fn column_named(&self, name: &str) -> Result<&SVPythonColumn, SVError> {
        self.columns
            .iter()
            .find(|column| column.name() == name)
            .ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_owned(),
            })
    }
}

impl SVDataSource for SVPythonTable {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(
            self.columns
                .iter()
                .map(|column| SVColumnDescriptor::new(column.name().to_owned(), column.role()))
                .collect(),
        )
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        let column = self.column_named(name)?;
        Ok(SVColumnSnapshot::Borrowed(column.values().view()?))
    }
}
