//! The optional CPython extension module.
//!
//! This module is compiled only behind the `python` feature. It is deliberately
//! thin: Python objects are translated into [`SVPythonFigureSpec`] values, the
//! pure conversion layer does the work, and errors travel back as typed
//! exceptions. No rendering, validation or dispatch logic lives here.
//!
//! Build the loadable module with
//! `cargo build -p scienceview_python --features python,extension-module`.

mod control;
mod data;
mod error;
mod figure;

pub use control::PyControlState;
pub use data::{PySparseColumn, PyStreamingColumn};
pub use error::{
    SVBackendError, SVBackendSelectionError, SVBindingError, SVConfigurationError, SVDataError,
    SVScriptError,
};
pub use figure::{PyDashboard, PyFigure};

use crate::sv_spec::sv_register_available_backends;
use pyo3::prelude::*;

/// The `scienceview` Python module.
#[pymodule]
#[pyo3(name = "scienceview")]
fn sv_scienceview_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    sv_register_available_backends();
    error::sv_register_exceptions(module)?;
    module.add_class::<PyFigure>()?;
    module.add_class::<PyDashboard>()?;
    module.add_class::<PySparseColumn>()?;
    module.add_class::<PyStreamingColumn>()?;
    module.add_class::<PyControlState>()?;
    figure::sv_register_module_functions(module)?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[cfg(all(test, feature = "python-tests"))]
#[path = "extension_tests.rs"]
mod extension_tests;
