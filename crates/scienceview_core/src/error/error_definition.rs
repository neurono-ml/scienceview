//! The single public error enumeration of the crate (P7).
//!
//! Every fallible public operation — figure configuration, rendering and the
//! data boundary — returns this type. Nothing here panics on a user path: a
//! missing backend, an unsupported capability, a malformed sparse view and a
//! source that lacks a column are all values.
//!
//! The data boundary used to carry its own `SVDataError`; its variants now live
//! here so callers match on one enumeration instead of translating between two.

use crate::sv_backend::SVBackendCapability;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_mark::SVMarkKind;
use crate::sv_move::SVMoveStage;
use thiserror::Error;

/// Everything that can go wrong while configuring, operating or reading a
/// figure.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SVError {
    /// The builder reached `build` without a backend selection.
    #[error("a backend selection is required before a figure can be built")]
    MissingBackendSelection,

    /// The builder reached `build` without a data source.
    #[error("a data source is required before a figure can be built")]
    MissingDataSource,

    /// The selected backend cannot host a capability the figure requested.
    #[error("backend `{backend_kind}` does not support the `{capability}` capability")]
    UnsupportedBackendCapability {
        /// The configured backend.
        backend_kind: SVBackendKind,
        /// The capability it lacks.
        capability: SVBackendCapability,
    },

    /// The selected backend cannot render a mark variant the figure composed.
    #[error("backend `{backend_kind}` cannot render the `{mark_kind}` mark")]
    UnsupportedMark {
        /// The configured backend.
        backend_kind: SVBackendKind,
        /// The mark variant it lacks.
        mark_kind: SVMarkKind,
    },

    /// A kernel cannot accept the representation it was given.
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

    /// No implementation of the configured backend is registered in the process.
    #[error("backend `{backend_kind}` is unavailable: no render backend is registered for it")]
    BackendUnavailable {
        /// The configured backend.
        backend_kind: SVBackendKind,
    },

    /// A registered backend failed while performing an operation.
    #[error("backend `{backend_kind}` failed during `{operation}`: {message}")]
    BackendFailure {
        /// The configured backend.
        backend_kind: SVBackendKind,
        /// The operation that failed (`render`, `update`, `save` or `show`).
        operation: &'static str,
        /// The backend's own explanation.
        message: String,
    },

    /// The configuration is internally inconsistent.
    #[error("invalid figure configuration: {message}")]
    InvalidConfiguration {
        /// What is wrong and how to fix it.
        message: String,
    },

    /// Two moves claim the same stage of the fixed composition order.
    #[error("two moves claim the `{stage}` stage of the move order; a stage runs at most once")]
    ConflictingMoveStage {
        /// The stage both moves claim.
        stage: SVMoveStage,
    },

    /// A move occupies a stage earlier than its predecessor.
    #[error(
        "the move at position {position} occupies the `{found_stage}` stage after `{preceding_stage}`; the fixed order is displacement, then shift, then normalize"
    )]
    MoveOrderViolation {
        /// The position of the offending move in the sequence.
        position: usize,
        /// The stage of the preceding move.
        preceding_stage: SVMoveStage,
        /// The stage of the offending move.
        found_stage: SVMoveStage,
    },

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
