//! The pure-Rust conversion layer: a declarative figure specification becomes
//! real builder calls and real bytes, with no interpreter involved.
//!
//! This layer exists so the binding's semantics are testable in an environment
//! that has no Python development headers. The optional `pyo3` extension in
//! [`sv_extension`](crate::sv_extension) only translates Python objects into
//! [`SVPythonFigureSpec`] and maps errors back to typed exceptions.

mod backend;
mod dashboard;
mod data;
mod data_source;
mod figure_spec;
mod operation;
mod options;
mod plot_kind;
mod registration;
mod render_output;
mod stream;

pub use backend::{sv_backend_kind_names, sv_parse_backend_kind, sv_require_backend_kind};
pub use dashboard::{SVPythonDashboardPanel, SVPythonDashboardSpec, sv_render_python_dashboard};
pub use data::{SVPythonColumn, SVPythonColumnValues, SVPythonTable};
pub use data_source::SVPythonData;
pub use figure_spec::{SVPythonFigureSpec, SVPythonRequirements};
pub use operation::{
    sv_python_point_count, sv_render_python_figure, sv_render_python_figure_svg,
    sv_save_python_figure, sv_update_python_figure,
};
pub use options::{SVPythonOptionDescriptor, SVPythonPlotOptions, sv_option_descriptors};
pub use plot_kind::SVPythonPlotKind;
pub use registration::sv_register_available_backends;
pub use render_output::SVPythonRenderOutput;
pub use stream::SVPythonStream;

#[cfg(test)]
#[path = "backend_selection_tests.rs"]
mod backend_selection_tests;
#[cfg(test)]
#[path = "binding_parity_tests.rs"]
mod binding_parity_tests;
#[cfg(test)]
#[path = "dashboard_tests.rs"]
mod dashboard_tests;
#[cfg(test)]
#[path = "data_source_tests.rs"]
mod data_source_tests;
#[cfg(test)]
#[path = "plot_kind_tests.rs"]
mod plot_kind_tests;
#[cfg(test)]
#[path = "representation_tests.rs"]
mod representation_tests;
#[cfg(test)]
#[path = "streaming_tests.rs"]
mod streaming_tests;
#[cfg(test)]
mod test_support;
