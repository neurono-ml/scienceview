//! The minimal JavaScript binding surface.
//!
//! JavaScript constructs a figure from a canvas id and values, pushes new data,
//! and asks for a render. The model stays in Rust: the bindings forward data to
//! [`SVWasmBackend`] and never keep a duplicate figure.

mod browser_figure;

pub use browser_figure::SVWasmFigure;
