//! The multivariate family: charts that relate more than two dimensions —
//! scatterplot matrix, pair plot, parallel coordinates, parallel categories and
//! joint plot.
//!
//! Every chart in this family appends one `#[path] pub mod` and one `pub use`
//! to this dispatcher; the family batch integrator resolves the ordering, so
//! recipe changes never edit a central registry (patterns.md P4). Recipe module
//! names carry the `sv_` prefix so the family scope never collides.

#[path = "splom_plot/mod.rs"]
pub mod sv_splom_plot;
pub use sv_splom_plot::{
    SVSplomCell, SVSplomCellKind, SVSplomDiagonal, SVSplomFrame, SVSplomMissingValuePolicy,
    SVSplomPlot, SVSplomPlotBuilder, SVSplomRenderOutput, SVSplomScaleMode, SVSplomScene,
};
