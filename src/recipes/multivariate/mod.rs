//! Multivariate recipes: charts that show several variables at once.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "parallel_coordinates_plot/mod.rs"]
pub mod sv_parallel_coordinates_plot;
pub use sv_parallel_coordinates_plot::{
    SVParallelCoordinatesPlot, SVParallelCoordinatesPlotBuilder, SVParallelCoordinatesRenderOutput,
};
