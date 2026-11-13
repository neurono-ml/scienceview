//! The distribution family: charts that show the shape of one variable.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "box_plot/mod.rs"]
pub mod sv_box_plot;
pub use sv_box_plot::{
    SVBoxPlot, SVBoxPlotBuilder, SVBoxRenderOutput, SVBoxSummary, SVWhiskerRule,
};

#[path = "density_contour_plot/mod.rs"]
pub mod sv_density_contour_plot;
pub use sv_density_contour_plot::{
    SVContourStyle, SVDensityContourComposition, SVDensityContourPlot, SVDensityContourPlotBuilder,
    SVDensityGrid,
};

#[path = "density_plot/mod.rs"]
pub mod sv_density_plot;
pub use sv_density_plot::{
    SVDensityCurve, SVDensityMissingValuePolicy, SVDensityPlot, SVDensityPlotBuilder,
    SVDensityRendering,
};

#[path = "ecdf_plot/mod.rs"]
pub mod sv_ecdf_plot;
pub use sv_ecdf_plot::{SVECdfPlot, SVECdfPlotBuilder, SVEcdfPlotComposition};

#[path = "histogram_plot/mod.rs"]
pub mod sv_histogram_plot;
pub use sv_histogram_plot::{SVBarElement, SVHistogramPlot, SVHistogramPlotBuilder};

#[path = "rug_plot/mod.rs"]
pub mod sv_rug_plot;
pub use sv_rug_plot::{SVRugPlot, SVRugPlotBuilder, SVRugRenderOutput, SVRugScene};

#[path = "swarm_plot/mod.rs"]
pub mod sv_swarm_plot;
pub use sv_swarm_plot::{
    SVSwarmFrame, SVSwarmMissingValuePolicy, SVSwarmPlot, SVSwarmPlotBuilder, SVSwarmRenderOutput,
    SVSwarmScene,
};
