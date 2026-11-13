//! Registration of the `scienceview` Python module.
//!
//! The module object is assembled here and nowhere else: classes, exception
//! classes and module-level functions are added in one place, so a missing
//! registration is visible at a glance.

use pyo3::prelude::*;

use crate::sv_spec::sv_register_available_backends;

/// The `scienceview` Python module.
#[pymodule]
#[pyo3(name = "scienceview")]
fn sv_scienceview_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    sv_register_available_backends();
    super::error::sv_register_exceptions(module)?;
    module.add_class::<super::figure::PyFigure>()?;
    module.add_class::<super::figure::PyDashboard>()?;
    module.add_class::<super::data::PySparseColumn>()?;
    module.add_class::<super::data::PyStreamingColumn>()?;
    module.add_class::<super::control::PyControlState>()?;
    super::figure::sv_register_module_functions(module)?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
