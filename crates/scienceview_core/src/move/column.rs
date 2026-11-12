//! One logical output column of a move, dense or sparse.
//!
//! A dense column stores one value per logical position. A sparse column
//! stores only the entries the input actually held and answers every other
//! logical position with a single documented *implicit contribution* — the
//! zero a sparse input already means. A sparse column therefore never
//! allocates one slot per logical position, which is what keeps a move over a
//! sparse column of huge logical length cheap.
//!
//! The working buffers a value-changing move writes use the same type, so the
//! displacement, value, baseline and width columns all share one
//! representation-aware shape.

use crate::sv_data::SVColumnView;

/// A logical column a move produced.
///
/// [`Self::Dense`] stores every position; [`Self::Sparse`] stores the entries
/// that exist and answers the rest from its implicit contribution. Read either
/// through [`Self::value_at`], or through [`Self::dense_values`] when the
/// column happens to be dense.
#[derive(Clone, Debug, PartialEq)]
pub enum SVMoveColumn {
    /// One stored value per logical position.
    Dense(Vec<f64>),
    /// Stored entries plus one implicit contribution for every other position.
    Sparse(SVMoveSparseColumn),
}

/// The sparse shape of an [`SVMoveColumn`].
///
/// `values[i]` is the value stored at `indices[i]`; every other position in
/// `0..length` carries [`Self::implicit_value`]. The indices are strictly
/// increasing, so the implicit contribution is the only value an unstored
/// logical position can have.
#[derive(Clone, Debug, PartialEq)]
pub struct SVMoveSparseColumn {
    indices: Vec<usize>,
    values: Vec<f64>,
    implicit_value: f64,
    length: usize,
}

impl SVMoveColumn {
    /// Returns the logical length of the column.
    pub fn length(&self) -> usize {
        match self {
            Self::Dense(values) => values.len(),
            Self::Sparse(sparse) => sparse.length,
        }
    }

    /// Returns `true` when the column stores every logical position.
    pub fn is_dense(&self) -> bool {
        matches!(self, Self::Dense(_))
    }

    /// Returns `true` when the column stores only its explicit entries.
    pub fn is_sparse(&self) -> bool {
        matches!(self, Self::Sparse(_))
    }

    /// Returns the value at a logical position, or `None` past the length.
    pub fn value_at(&self, index: usize) -> Option<f64> {
        match self {
            Self::Dense(values) => values.get(index).copied(),
            Self::Sparse(sparse) => sparse.value_at(index),
        }
    }

    /// Returns the borrowed dense slice, when the column is dense.
    ///
    /// A sparse column is never expanded, so it returns `None`; read it
    /// through [`Self::value_at`] or its stored entries instead.
    pub fn dense_values(&self) -> Option<&[f64]> {
        match self {
            Self::Dense(values) => Some(values),
            Self::Sparse(_) => None,
        }
    }

    /// Returns the stored entries of a sparse column, as `(index, value)`.
    ///
    /// A dense column stores every position, so it returns the pairs for the
    /// whole slice as well; this is the one iteration that visits only what
    /// the column actually holds.
    pub fn stored_entries(&self) -> SVMoveStoredEntries<'_> {
        match self {
            Self::Dense(values) => SVMoveStoredEntries::Dense(values.iter().enumerate()),
            Self::Sparse(sparse) => SVMoveStoredEntries::Sparse(
                sparse
                    .indices
                    .iter()
                    .copied()
                    .zip(sparse.values.iter().copied()),
            ),
        }
    }

    /// Returns the implicit contribution of every unstored position.
    ///
    /// A dense column has no unstored position, so it reports `0.0`.
    pub fn implicit_value(&self) -> f64 {
        match self {
            Self::Dense(_) => 0.0,
            Self::Sparse(sparse) => sparse.implicit_value,
        }
    }

    /// Builds a zero column shaped like the source: dense for a dense source,
    /// sparse with no stored entry for a sparse one.
    pub(crate) fn sv_zeroed_like(view: SVColumnView<'_>) -> Self {
        match view {
            SVColumnView::Dense(values) => Self::Dense(vec![0.0; values.len()]),
            SVColumnView::Sparse(sparse) => {
                Self::Sparse(SVMoveSparseColumn::sv_zeroed(sparse.length()))
            }
            SVColumnView::Streaming(streaming) => Self::Dense(vec![0.0; streaming.length()]),
        }
    }

    /// Writes one logical position, inserting it into a sparse column when it
    /// is not stored yet.
    pub(crate) fn sv_set(&mut self, index: usize, value: f64) {
        match self {
            Self::Dense(values) => {
                if let Some(slot) = values.get_mut(index) {
                    *slot = value;
                }
            }
            Self::Sparse(sparse) => sparse.set(index, value),
        }
    }

    /// Adds `delta` to one logical position, reading the implicit contribution
    /// first when a sparse column does not store it.
    pub(crate) fn sv_add(&mut self, index: usize, delta: f64) {
        let current = self.value_at(index).unwrap_or(self.implicit_value());
        self.sv_set(index, current + delta);
    }

    /// Returns the logical positions the column stores.
    ///
    /// A dense column returns `0..length`; a sparse column returns the indices
    /// it actually holds, so iterating it never visits an implicit position.
    pub(crate) fn sv_stored_positions(&self) -> SVMoveStoredPositions {
        match self {
            Self::Dense(values) => SVMoveStoredPositions::Dense(0..values.len()),
            Self::Sparse(sparse) => {
                SVMoveStoredPositions::Sparse(sparse.indices.clone().into_iter())
            }
        }
    }
}

impl SVMoveSparseColumn {
    /// Builds an all-implicit zero column.
    pub(crate) fn sv_zeroed(length: usize) -> Self {
        Self {
            indices: Vec::new(),
            values: Vec::new(),
            implicit_value: 0.0,
            length,
        }
    }

    /// Builds a sparse column from validated stored entries.
    pub(crate) fn sv_from_stored(
        indices: &[usize],
        values: &[f64],
        implicit_value: f64,
        length: usize,
    ) -> Self {
        Self {
            indices: indices.to_vec(),
            values: values.to_vec(),
            implicit_value,
            length,
        }
    }

    /// Returns the logical length, implicit positions included.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the stored logical indices, strictly increasing.
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    /// Returns the value stored at each index.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Returns the number of stored entries.
    pub fn number_of_stored_values(&self) -> usize {
        self.values.len()
    }

    /// Returns the value every unstored position carries.
    pub fn implicit_value(&self) -> f64 {
        self.implicit_value
    }

    /// Returns the value at a logical position, or `None` past the length.
    pub fn value_at(&self, index: usize) -> Option<f64> {
        if index >= self.length {
            return None;
        }
        match self.indices.binary_search(&index) {
            Ok(position) => Some(self.values[position]),
            Err(_) => Some(self.implicit_value),
        }
    }

    /// Writes one logical position, storing it when it was implicit.
    fn set(&mut self, index: usize, value: f64) {
        if index >= self.length {
            return;
        }
        match self.indices.binary_search(&index) {
            Ok(position) => self.values[position] = value,
            Err(position) => {
                self.indices.insert(position, index);
                self.values.insert(position, value);
            }
        }
    }
}

/// The stored entries of an [`SVMoveColumn`].
pub enum SVMoveStoredEntries<'a> {
    /// A dense column, whose every position is stored.
    Dense(core::iter::Enumerate<core::slice::Iter<'a, f64>>),
    /// A sparse column, whose stored entries come from its indices and values.
    Sparse(
        core::iter::Zip<
            core::iter::Copied<core::slice::Iter<'a, usize>>,
            core::iter::Copied<core::slice::Iter<'a, f64>>,
        >,
    ),
}

impl Iterator for SVMoveStoredEntries<'_> {
    type Item = (usize, f64);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Dense(entries) => entries.next().map(|(index, value)| (index, *value)),
            Self::Sparse(entries) => entries.next(),
        }
    }
}

/// The stored logical positions of an [`SVMoveColumn`].
pub(crate) enum SVMoveStoredPositions {
    /// A dense column stores every position.
    Dense(core::ops::Range<usize>),
    /// A sparse column stores the indices it holds.
    Sparse(std::vec::IntoIter<usize>),
}

impl Iterator for SVMoveStoredPositions {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Dense(positions) => positions.next(),
            Self::Sparse(positions) => positions.next(),
        }
    }
}
