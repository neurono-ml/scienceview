//! `SVParallelCoordinatesPlot`: many observations across many parallel axes.
//!
//! The chart is the composition of FDR-141:
//! `data -> [no stat] -> values -> [one line per observation] -> [SVScale per
//! axis] -> SVMark::Line`. Each variable owns one vertical axis with its own
//! continuous scale, and each observation is a polyline through the visible
//! axes.
//!
//! This module is a pure dispatcher (patterns.md P3): every implementation
//! lives in a named sibling file.

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod model;
mod options;
mod output;
mod preparation;
mod rendering;
mod serialization;

pub use builder::SVParallelCoordinatesPlotBuilder;
pub use core_implementation::SVParallelCoordinatesPlot;
pub use model::{SVParallelCoordinatesAxis, SVParallelCoordinatesLine};
pub use options::{
    SVDimension, SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization,
};
pub use output::SVParallelCoordinatesRenderOutput;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "error_tests.rs"]
mod error_tests;
#[cfg(test)]
#[path = "missing_tests.rs"]
mod missing_tests;
#[cfg(test)]
#[path = "observability_support.rs"]
mod observability_support;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "option_tests.rs"]
mod option_tests;
#[cfg(test)]
#[path = "parallel_coordinates_plot_tests.rs"]
mod parallel_coordinates_plot_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
