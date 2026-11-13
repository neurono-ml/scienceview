//! The built density plot, its operations and its serializers.
//!
//! This module is a pure dispatcher (patterns.md P3): the built value and its
//! accessors, the public operations and the backend serializers each live in
//! their own file.

mod built_plot;
mod operations;
mod serialization;

pub use built_plot::SVDensityPlot;
