//! The matrix family: charts that draw a relationship between two continuous
//! variables as a lattice of cells.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md P4).
//! Recipe module names carry the `sv_` prefix so the family scope never
//! collides.

#[path = "hexbin_plot/mod.rs"]
pub mod sv_hexbin_plot;
pub use sv_hexbin_plot::{
    SVHexbinAggregation, SVHexbinCell, SVHexbinComposition, SVHexbinMissingValuePolicy,
    SVHexbinPlot, SVHexbinPlotBuilder,
};
