//! The optional CPython extension module.
//!
//! This module is compiled only behind the `python` feature. It is deliberately
//! thin: Python objects are translated into [`SVPythonFigureSpec`] values, the
//! pure conversion layer does the work, and errors travel back as typed
//! exceptions. No rendering, validation or dispatch logic lives here.
//!
//! Build the loadable module with `cargo build -p scienceview_python --features
//! python`; maturin enables PyO3's `extension-module` behaviour for a wheel.

mod control;
mod data;
mod error;
mod figure;
mod module;

pub use control::PyControlState;
pub use data::{PySparseColumn, PyStreamingColumn};
pub use error::{
    SVBackendError, SVBackendSelectionError, SVBindingError, SVConfigurationError, SVDataError,
    SVScriptError,
};
pub use figure::{PyDashboard, PyFigure};

#[cfg(all(test, feature = "python-tests"))]
#[path = "extension_tests.rs"]
mod extension_tests;
