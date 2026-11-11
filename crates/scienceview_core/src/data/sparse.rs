//! Borrowed sparse storage: indices plus the values stored at them.
//!
//! A sparse view is never expanded into a dense buffer. Kernels that can use
//! indices work on them directly; kernels that cannot report
//! [`SVDataError::UnsupportedRepresentation`](crate::sv_data::SVDataError::UnsupportedRepresentation)
//! and name the alternative path instead of densifying.

use crate::sv_data::SVDataError;

/// A borrowed sparse column of `length` logical values.
///
/// Only the entries listed in `indices` are stored; every other logical
/// position is an implicit zero. Indices must be strictly increasing and fall
/// inside `0..length`, which [`SVSparseView::new`] validates once.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVSparseView<'a> {
    indices: &'a [usize],
    values: &'a [f64],
    length: usize,
}

impl<'a> SVSparseView<'a> {
    /// Validates and builds a sparse view.
    ///
    /// # Errors
    ///
    /// Returns [`SVDataError::MismatchedSparseLengths`] when the slices differ
    /// in length, [`SVDataError::SparseIndicesNotIncreasing`] when the indices
    /// are not strictly increasing and
    /// [`SVDataError::SparseIndexOutOfRange`] when an index points past the
    /// logical length.
    pub fn new(
        indices: &'a [usize],
        values: &'a [f64],
        length: usize,
    ) -> Result<Self, SVDataError> {
        if indices.len() != values.len() {
            return Err(SVDataError::MismatchedSparseLengths {
                indices: indices.len(),
                values: values.len(),
            });
        }

        let mut previous: Option<usize> = None;
        for (position, index) in indices.iter().copied().enumerate() {
            if previous.is_some_and(|previous_index| index <= previous_index) {
                return Err(SVDataError::SparseIndicesNotIncreasing { position });
            }
            if index >= length {
                return Err(SVDataError::SparseIndexOutOfRange { index, length });
            }
            previous = Some(index);
        }

        Ok(Self {
            indices,
            values,
            length,
        })
    }

    /// Returns the logical length of the column, implicit zeros included.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns `true` when the column holds no logical position.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the borrowed indices.
    pub fn indices(&self) -> &'a [usize] {
        self.indices
    }

    /// Returns the borrowed stored values.
    pub fn values(&self) -> &'a [f64] {
        self.values
    }

    /// Returns the number of entries that are actually stored.
    pub fn number_of_stored_values(&self) -> usize {
        self.values.len()
    }

    /// Returns the stored entries as `(index, value)` pairs.
    pub fn stored_entries(&self) -> impl Iterator<Item = (usize, f64)> + 'a {
        self.indices
            .iter()
            .copied()
            .zip(self.values.iter().copied())
    }

    /// Returns the value at a logical position, zero when it is not stored.
    ///
    /// The lookup is a binary search over the indices; the column is never
    /// materialised.
    pub fn value_at(&self, index: usize) -> Option<f64> {
        if index >= self.length {
            return None;
        }

        match self.indices.binary_search(&index) {
            Ok(position) => Some(self.values[position]),
            Err(_) => Some(0.0),
        }
    }
}

#[cfg(test)]
#[path = "sparse_tests.rs"]
mod sparse_tests;
