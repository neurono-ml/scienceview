//! The matrix family: charts that show a two-dimensional field.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "density_heatmap_plot/mod.rs"]
pub mod sv_density_heatmap_plot;
pub use sv_density_heatmap_plot::{
    SVDensityHeatmapComposition, SVDensityHeatmapGrid, SVDensityHeatmapPlot,
    SVDensityHeatmapPlotBuilder, SVEmptyBinPolicy,
};
