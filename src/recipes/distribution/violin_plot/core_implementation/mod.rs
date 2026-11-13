//! The built violin plot, its accessors and its operations.
//!
//! This module is a pure dispatcher (patterns.md P3): the built value and its
//! accessors and the public operations each live in their own file.

mod built_plot;
mod operations;

pub use built_plot::SVViolinPlot;
