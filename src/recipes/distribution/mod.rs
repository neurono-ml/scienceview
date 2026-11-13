//! The distributional family: charts that summarise a distribution — box,
//! violin, density, histogram and their relatives.
//!
//! Every chart in this family appends one `#[path] pub mod` and one `pub use`
//! to this dispatcher; the family batch integrator resolves the ordering, so
//! recipe changes never edit a central registry (patterns.md P4). Recipe module
//! names carry the `sv_` prefix so the family scope never collides.

#[path = "box_plot/mod.rs"]
pub mod sv_box_plot;
pub use sv_box_plot::{
    SVBoxPlot, SVBoxPlotBuilder, SVBoxRenderOutput, SVBoxSummary, SVWhiskerRule,
};
