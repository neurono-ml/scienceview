//! The distribution family: charts that show the shape of one variable.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "density_plot/mod.rs"]
pub mod sv_density_plot;
pub use sv_density_plot::{
    SVDensityCurve, SVDensityMissingValuePolicy, SVDensityPlot, SVDensityPlotBuilder,
    SVDensityRendering,
};

#[path = "histogram_plot/mod.rs"]
pub mod sv_histogram_plot;
pub use sv_histogram_plot::{SVBarElement, SVHistogramPlot, SVHistogramPlotBuilder};
