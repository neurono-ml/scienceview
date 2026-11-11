//! A column backed by a borrowed `ndarray` view.

use std::borrow::Cow;

use ndarray::ArrayView1;

use crate::data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataError, SVDataSource,
    SVSourceSchema,
};

/// A static, single-column source over an `ndarray` view.
///
/// The view is borrowed; nothing is copied. A view that is not contiguous
/// cannot be borrowed as a slice, so it is rejected with a named alternative
/// instead of being materialised silently.
#[derive(Debug)]
pub struct SVNdarrayColumn<'a> {
    descriptor: SVColumnDescriptor<'a>,
    values: ArrayView1<'a, f64>,
}

impl<'a> SVNdarrayColumn<'a> {
    /// Wraps an `ndarray` view; the column plays the vertical role.
    pub fn new(name: impl Into<Cow<'a, str>>, values: ArrayView1<'a, f64>) -> Self {
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

    /// Returns the borrowed `ndarray` view.
    pub fn view(&self) -> ArrayView1<'a, f64> {
        self.values
    }
}

impl SVDataSource for SVNdarrayColumn<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(std::slice::from_ref(&self.descriptor))
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVDataError> {
        if name != self.descriptor.name() {
            return Err(SVDataError::ColumnNotFound {
                name: name.to_owned(),
            });
        }

        match self.values.as_slice() {
            Some(values) => Ok(SVColumnSnapshot::Borrowed(SVColumnView::Dense(values))),
            None => Err(SVDataError::NonContiguousView {
                origin: "ndarray",
                alternative: "call `.to_owned()` on the array and pass the contiguous buffer as a borrowed slice",
            }),
        }
    }
}

#[cfg(test)]
#[path = "ndarray_view_tests.rs"]
mod ndarray_view_tests;
