//! The multivariate family: charts whose subject is the relation between
//! several variables at once.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "pair_plot/mod.rs"]
pub mod sv_pair_plot;
pub use sv_pair_plot::{
    SVPairDiagonalKind, SVPairFrame, SVPairMissingValuePolicy, SVPairOffDiagonalKind, SVPairPlot,
    SVPairPlotBuilder, SVPairRenderOutput, SVPairScene,
};
