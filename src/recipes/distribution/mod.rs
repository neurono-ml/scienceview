//! The distribution family: charts that describe the shape of one variable —
//! histogram, density, ECDF, rug, box, violin, strip, swarm and their
//! relatives.
//!
//! Every chart in this family appends one `#[path] pub mod` and one `pub use`
//! to this dispatcher; the family batch integrator resolves the ordering, so
//! recipe changes never edit a central registry (patterns.md P4). Recipe
//! module names carry the `sv_` prefix so the family scope never collides.

#[path = "ecdf_plot/mod.rs"]
pub mod sv_ecdf_plot;
pub use sv_ecdf_plot::{SVECdfPlot, SVECdfPlotBuilder, SVEcdfPlotComposition};
