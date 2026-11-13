//! `scienceview_python`: the Python binding of the validated `scienceview`
//! surface.
//!
//! The crate is split in two layers on purpose.
//!
//! ## The pure conversion layer
//!
//! [`sv_spec`] owns the data a Python caller attaches to a figure, describes
//! the figure as a value ([`SVPythonFigureSpec`](sv_spec::SVPythonFigureSpec))
//! and performs the real `scienceview` builder calls and operations. It needs no
//! interpreter, no `pyo3` and no Python development headers, so the semantics of
//! the binding are proven by ordinary unit tests.
//!
//! ```no_run
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview_python::sv_spec::{
//!     SVPythonData, SVPythonFigureSpec, SVPythonPlotKind, SVPythonPlotOptions,
//!     sv_render_python_figure,
//! };
//!
//! let horizontal = [0.0, 1.0, 2.0];
//! let vertical = [1.0, 4.0, 2.0];
//! let spec = SVPythonFigureSpec::new(
//!     SVPythonPlotKind::ScatterPlot,
//!     SVPythonData::dense(horizontal.to_vec()).expect("a dense column"),
//! )
//! .with_options(SVPythonPlotOptions {
//!     x: Some("x".to_string()),
//!     y: Some("y".to_string()),
//!     ..SVPythonPlotOptions::default()
//! })
//! .with_backend(SVBackendKind::Svg);
//! # let _ = (vertical, sv_render_python_figure(&spec));
//! ```
//!
//! ## The interpreter layer
//!
//! [`sv_extension`] is compiled only behind the `python` feature. It translates
//! keyword arguments into the specification, returns bytes and paths, and maps
//! [`SVError`](scienceview::sv_error::SVError) variants onto typed Python
//! exception classes. `pyo3` is never part of the default build.
//!
//! Build the loadable module with `cargo build --features python,extension-module`
//! (a Python development environment is required); see the crate README for the
//! wheel instructions.

#![warn(missing_docs)]

#[path = "spec/mod.rs"]
pub mod sv_spec;

#[cfg(feature = "python")]
#[path = "extension/mod.rs"]
pub mod sv_extension;
