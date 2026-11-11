//! A column backed by a borrowed slice of `f64`.

use std::borrow::Cow;

use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataError, SVDataSource,
    SVSourceSchema,
};

/// A static, single-column source over a borrowed slice.
///
/// This is the reference implementation of the ingestion trait: it copies
/// nothing and is the representation every other adapter is compared against.
#[derive(Clone, Debug)]
pub struct SVSliceColumn<'a> {
    descriptor: SVColumnDescriptor<'a>,
    values: &'a [f64],
}

impl<'a> SVSliceColumn<'a> {
    /// Wraps a borrowed slice; the column plays the vertical role.
    pub fn new(name: impl Into<Cow<'a, str>>, values: &'a [f64]) -> Self {
        Self {
            descriptor: SVColumnDescriptor::new(name, SVColumnRole::Y),
            values,
        }
    }

    /// Changes the role the column declares.
    #[must_use]
    pub fn with_role(mut self, role: SVColumnRole) -> Self {
        self.descriptor = SVColumnDescriptor::new(self.descriptor.name().to_owned(), role);
        self
    }

    /// Returns the column name.
    pub fn name(&self) -> &str {
        self.descriptor.name()
    }

    /// Returns the borrowed values.
    pub fn values(&self) -> &'a [f64] {
        self.values
    }
}

impl SVDataSource for SVSliceColumn<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(std::slice::from_ref(&self.descriptor))
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVDataError> {
        if name == self.descriptor.name() {
            Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(self.values)))
        } else {
            Err(SVDataError::ColumnNotFound {
                name: name.to_owned(),
            })
        }
    }
}

#[cfg(test)]
#[path = "borrowed_slice_tests.rs"]
mod borrowed_slice_tests;
