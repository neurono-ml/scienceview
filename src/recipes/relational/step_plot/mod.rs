//! `SVStepPlot`: a value held constant between discrete events.

mod builder;
mod composition;
mod core_implementation;
mod decorations;
mod extraction;
mod options;
#[cfg(feature = "render-svg")]
mod render_svg;

pub use builder::SVStepPlotBuilder;
pub use composition::{SVStepPlotComposition, SVStepSeries};
pub use core_implementation::SVStepPlot;
pub use options::{SVStepDuplicatePolicy, SVStepPosition};

#[cfg(test)]
#[path = "step_plot_tests/mod.rs"]
mod step_plot_tests;
