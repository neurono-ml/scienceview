//! The single public error enumeration of the figure surface (P7).
//!
//! Every fallible public operation returns this type. Nothing here panics on a
//! user path: a missing backend, an unsupported capability, an unsupported
//! representation and a backend that is not registered are all values.

use crate::sv_backend::SVBackendCapability;
use crate::sv_backend::SVBackendKind;
use crate::sv_figure::SVDataRepresentation;
use thiserror::Error;

/// Everything that can go wrong while configuring or operating a figure.
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

    /// A kernel cannot accept the representation it was given.
    #[error(
        "the `{representation}` representation is not supported here; use {alternative_path} instead"
    )]
    UnsupportedRepresentation {
        /// The representation the caller supplied.
        representation: SVDataRepresentation,
        /// The documented alternative path.
        alternative_path: &'static str,
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
}
