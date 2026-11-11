//! A column backed by a borrowed `sprs` sparse vector.

use std::borrow::Cow;

use sprs::CsVecViewI;

use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataError, SVDataSource,
    SVSourceSchema, SVSparseView,
};

/// A static, single-column source over an `sprs` sparse vector.
///
/// The indices and the stored values are borrowed and handed to the kernel as
/// a [`SVSparseView`]; the vector is never expanded into a dense buffer.
#[derive(Debug)]
pub struct SVSparseColumn<'a> {
    descriptor: SVColumnDescriptor<'a>,
    values: CsVecViewI<'a, f64, usize>,
}

impl<'a> SVSparseColumn<'a> {
    /// Wraps an `sprs` view; the column plays the vertical role.
    pub fn new(name: impl Into<Cow<'a, str>>, values: CsVecViewI<'a, f64, usize>) -> Self {
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

    /// Returns the borrowed `sprs` view.
    pub fn view(&self) -> CsVecViewI<'a, f64, usize> {
        self.values
    }
}

impl SVDataSource for SVSparseColumn<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(std::slice::from_ref(&self.descriptor))
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVDataError> {
        if name != self.descriptor.name() {
            return Err(SVDataError::ColumnNotFound {
                name: name.to_owned(),
            });
        }

        let sparse =
            SVSparseView::new(self.values.indices(), self.values.data(), self.values.dim())?;
        Ok(SVColumnSnapshot::Borrowed(SVColumnView::Sparse(sparse)))
    }
}
#[cfg(test)]
#[path = "sprs_column_tests.rs"]
mod sprs_column_tests;
