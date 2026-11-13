//! The distributional family: charts that show one variable's distribution —
//! histogram, density, ECDF, rug, box, violin, strip, swarm and their relatives.
//!
//! Every chart in this family appends one `#[path] pub mod` and one `pub use`
//! to this dispatcher; the family batch integrator resolves the ordering, so
//! recipe changes never edit a central registry (patterns.md P4). Recipe module
//! names carry the `sv_` prefix so the family scope never collides.

#[path = "rug_plot/mod.rs"]
pub mod sv_rug_plot;
pub use sv_rug_plot::{SVRugPlot, SVRugPlotBuilder, SVRugRenderOutput, SVRugScene};
