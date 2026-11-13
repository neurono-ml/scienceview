//! `SVLinePlot`: how a continuous value evolves along an ordered axis.
//!
//! The chart composes the core kernels — data boundary, aggregate stat, scales,
//! line/dot/ribbon marks and decorations — into one recipe. It adds no kernel
//! variant of its own (patterns P5) and is registered in the relational family
//! module, never in a central list (patterns P4).

mod builder;
mod composition;
mod core_implementation;
mod decorations;
mod geometry;
mod interpolation;
mod series;
#[cfg(feature = "render-svg")]
mod svg_output;

pub use builder::{SVLineInterpolation, SVLinePlotBuilder};
pub use composition::SVLinePlotComposition;
pub use core_implementation::SVLinePlot;
pub use series::{SVLinePlotInterval, SVLinePlotSegment, SVLinePlotSeries};

#[cfg(test)]
#[path = "line_plot_tests.rs"]
mod line_plot_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "svg_render_tests.rs"]
mod svg_render_tests;
