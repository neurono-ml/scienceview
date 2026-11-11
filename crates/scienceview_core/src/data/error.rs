//! Errors raised by the data boundary.
//!
//! The data boundary owns its failures in this module so that it can land
//! independently of the figure error module. The integration point is a single
//! wrapping variant once `SVError` exists:
//!
//! ```text
//! #[error(transparent)]
//! Data(#[from] SVDataError),
//! ```
//!
//! Nothing outside this module constructs a data failure, so folding the
//! enumeration into `SVError` later is a mechanical change.

use crate::data::SVRepresentation;

/// A failure raised while reading data or building a view.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SVDataError {
    /// The requested column is not part of the source.
    #[error("column `{name}` is not part of the data source")]
    ColumnNotFound {
        /// The name that was requested.
        name: String,
    },

    /// Two columns of one table declared the same name.
    #[error("the data source declares the column `{name}` more than once")]
    DuplicateColumnName {
        /// The repeated name.
        name: String,
    },

    /// Two columns of one table have different lengths.
    #[error(
        "the columns of a data source must have the same length; the column `{name}` has {actual} values but {expected} were expected"
    )]
    MismatchedLengths {
        /// The column whose length differs.
        name: String,
        /// The length fixed by the first column.
        expected: usize,
        /// The length of the offending column.
        actual: usize,
    },

    /// A sparse view declared a different number of indices and values.
    #[error(
        "a sparse view needs one value per index; it received {indices} indices and {values} values"
    )]
    MismatchedSparseLengths {
        /// The number of stored indices.
        indices: usize,
        /// The number of stored values.
        values: usize,
    },

    /// A sparse index points past the logical length of the column.
    #[error("the sparse index {index} is out of range for a column of length {length}")]
    SparseIndexOutOfRange {
        /// The offending index.
        index: usize,
        /// The logical length of the column.
        length: usize,
    },

    /// Sparse indices are not strictly increasing.
    #[error(
        "sparse indices must be strictly increasing; the entry at position {position} does not increase"
    )]
    SparseIndicesNotIncreasing {
        /// The position of the offending index.
        position: usize,
    },

    /// An operation cannot consume this representation.
    #[error(
        "`{operation}` does not support the {representation} representation; use {alternative}"
    )]
    UnsupportedRepresentation {
        /// The operation that rejected the view.
        operation: &'static str,
        /// The rejected representation.
        representation: SVRepresentation,
        /// The documented alternative path to use instead.
        alternative: &'static str,
    },

    /// An external view is not contiguous, so it cannot be borrowed as a slice.
    #[error(
        "the {origin} view is not contiguous, so it cannot be borrowed as a dense slice; {alternative}"
    )]
    NonContiguousView {
        /// The external library that produced the view.
        origin: &'static str,
        /// The documented alternative path to use instead.
        alternative: &'static str,
    },

    /// A rolling window was created with a capacity of zero.
    #[error("a rolling window needs a capacity of at least one sample, received {capacity}")]
    InvalidWindowCapacity {
        /// The rejected capacity.
        capacity: usize,
    },

    /// A source adapter was asked for a column it does not expose.
    #[error("the source adapter exposes `{expected}` and cannot serve `{requested}`")]
    UnknownColumn {
        /// The column the adapter exposes.
        expected: String,
        /// The column that was requested.
        requested: String,
    },
}
