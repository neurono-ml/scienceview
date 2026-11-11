//! A named, multi-column static source built through a validating builder.

use crate::sv_error::SVError;
use std::borrow::Cow;

use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
};

/// A static source made of named columns that share one logical length.
#[derive(Debug)]
pub struct SVDataTable<'a> {
    descriptors: Vec<SVColumnDescriptor<'a>>,
    columns: Vec<&'a [f64]>,
}

impl<'a> SVDataTable<'a> {
    /// Starts building a table.
    pub fn builder() -> SVDataTableBuilder<'a> {
        SVDataTableBuilder::new()
    }

    /// Returns the descriptor of a named column.
    pub fn descriptor(&self, name: &str) -> Option<&SVColumnDescriptor<'a>> {
        self.descriptors
            .iter()
            .find(|descriptor| descriptor.name() == name)
    }

    /// Returns the borrowed values of a named column.
    pub fn values(&self, name: &str) -> Option<&'a [f64]> {
        self.columns
            .iter()
            .zip(self.descriptors.iter())
            .find(|(_, descriptor)| descriptor.name() == name)
            .map(|(values, _)| *values)
    }

    /// Returns the number of columns.
    pub fn number_of_columns(&self) -> usize {
        self.columns.len()
    }

    /// Returns the logical length every column shares.
    pub fn length(&self) -> usize {
        self.columns.first().map_or(0, |values| values.len())
    }

    /// Returns the borrowed values of every column, in declaration order.
    pub fn columns(&self) -> &[&'a [f64]] {
        &self.columns
    }
}

impl SVDataSource for SVDataTable<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(&self.descriptors)
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match self.values(name) {
            Some(values) => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(values))),
            None => Err(SVError::ColumnNotFound {
                name: name.to_owned(),
            }),
        }
    }
}

/// The validating builder of [`SVDataTable`].
///
/// Columns are declared freely; [`SVDataTableBuilder::build`] is where a
/// duplicate name or a mismatched length is rejected, so a table that exists is
/// always well formed.
#[derive(Debug, Default)]
pub struct SVDataTableBuilder<'a> {
    descriptors: Vec<SVColumnDescriptor<'a>>,
    columns: Vec<&'a [f64]>,
}

impl<'a> SVDataTableBuilder<'a> {
    /// Starts an empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a column with its role.
    #[must_use]
    pub fn with_column(
        mut self,
        name: impl Into<Cow<'a, str>>,
        role: SVColumnRole,
        values: &'a [f64],
    ) -> Self {
        self.descriptors.push(SVColumnDescriptor::new(name, role));
        self.columns.push(values);
        self
    }

    /// Adds a column with the default role.
    #[must_use]
    pub fn with_values(self, name: impl Into<Cow<'a, str>>, values: &'a [f64]) -> Self {
        self.with_column(name, SVColumnRole::Y, values)
    }

    /// Validates and builds the table.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::DuplicateColumnName`] for a repeated name and
    /// [`SVError::MismatchedLengths`] for a column whose length differs
    /// from the first one, before any column is read.
    pub fn build(self) -> Result<SVDataTable<'a>, SVError> {
        for (position, descriptor) in self.descriptors.iter().enumerate() {
            let repeated = self.descriptors[..position]
                .iter()
                .any(|earlier| earlier.name() == descriptor.name());
            if repeated {
                return Err(SVError::DuplicateColumnName {
                    name: descriptor.name().to_owned(),
                });
            }
        }

        if let Some(expected) = self.columns.first().map(|values| values.len()) {
            for (descriptor, values) in self.descriptors.iter().zip(self.columns.iter()) {
                if values.len() != expected {
                    return Err(SVError::MismatchedLengths {
                        name: descriptor.name().to_owned(),
                        expected,
                        actual: values.len(),
                    });
                }
            }
        }

        Ok(SVDataTable {
            descriptors: self.descriptors,
            columns: self.columns,
        })
    }
}

#[cfg(test)]
#[path = "table_tests.rs"]
mod table_tests;
