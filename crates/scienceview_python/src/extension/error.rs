//! Typed Python exceptions for the Rust failure catalogue.
//!
//! Every `SVError` variant maps onto one exception class that a Python caller
//! can catch precisely. The classes form a hierarchy rooted at
//! `SVBindingError`, so `except SVBindingError` still catches everything while
//! `except SVBackendSelectionError` catches exactly the missing backend.

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use scienceview::sv_error::SVError;

create_exception!(
    scienceview,
    SVBindingError,
    PyException,
    "Base class of every error raised by the scienceview Python binding."
);
create_exception!(
    scienceview,
    SVBackendSelectionError,
    SVBindingError,
    "Raised when a figure is built without an explicit backend selection."
);
create_exception!(
    scienceview,
    SVConfigurationError,
    SVBindingError,
    "Raised for an impossible option, an unknown kind or an unsupported representation."
);
create_exception!(
    scienceview,
    SVDataError,
    SVBindingError,
    "Raised when the data source is missing, unknown or inconsistent."
);
create_exception!(
    scienceview,
    SVBackendError,
    SVBindingError,
    "Raised when the selected backend is unavailable, lacks a capability or fails."
);
create_exception!(
    scienceview,
    SVScriptError,
    SVBindingError,
    "Raised for a scripting failure."
);

/// Translates one `SVError` into the typed exception a caller can catch.
pub(crate) fn sv_python_error(error: SVError) -> PyErr {
    let message = error.to_string();
    match &error {
        SVError::MissingBackendSelection => SVBackendSelectionError::new_err(message),
        SVError::MissingDataSource
        | SVError::ColumnNotFound { .. }
        | SVError::DuplicateColumnName { .. }
        | SVError::MismatchedLengths { .. }
        | SVError::MismatchedSparseLengths { .. }
        | SVError::SparseIndexOutOfRange { .. }
        | SVError::SparseIndicesNotIncreasing { .. }
        | SVError::UnknownColumn { .. }
        | SVError::EmptyInput { .. }
        | SVError::NonFiniteValue { .. }
        | SVError::MismatchedStatisticLengths { .. }
        | SVError::NonContiguousView { .. }
        | SVError::InvalidWindowCapacity { .. } => SVDataError::new_err(message),
        SVError::BackendUnavailable { .. }
        | SVError::BackendFailure { .. }
        | SVError::UnsupportedBackendCapability { .. }
        | SVError::UnsupportedMark { .. } => SVBackendError::new_err(message),
        SVError::ScriptNotFound { .. }
        | SVError::ScriptAlreadyRegistered { .. }
        | SVError::ScriptParseFailure { .. }
        | SVError::ScriptRuntimeFailure { .. }
        | SVError::ScriptSandboxViolation { .. }
        | SVError::ScriptUnknownAction { .. }
        | SVError::ScriptBudgetExceeded { .. }
        | SVError::UnsupportedScriptPlacement { .. }
        | SVError::ScriptEngineMismatch { .. } => SVScriptError::new_err(message),
        _ => SVConfigurationError::new_err(message),
    }
}

/// Registers every exception class on the extension module.
pub(crate) fn sv_register_exceptions(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let python = module.py();
    module.add("SVBindingError", python.get_type::<SVBindingError>())?;
    module.add(
        "SVBackendSelectionError",
        python.get_type::<SVBackendSelectionError>(),
    )?;
    module.add(
        "SVConfigurationError",
        python.get_type::<SVConfigurationError>(),
    )?;
    module.add("SVDataError", python.get_type::<SVDataError>())?;
    module.add("SVBackendError", python.get_type::<SVBackendError>())?;
    module.add("SVScriptError", python.get_type::<SVScriptError>())?;
    Ok(())
}
