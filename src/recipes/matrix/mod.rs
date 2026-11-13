//! The matrix family: charts that draw a field defined over two axes.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "contour_plot/mod.rs"]
pub mod sv_contour_plot;
pub use sv_contour_plot::{
    SVContourComposition, SVContourField, SVContourPlot, SVContourPlotBuilder,
};
