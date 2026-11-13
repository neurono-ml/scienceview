//! The distribution family: charts that show the shape of one variable.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "bar_plot/mod.rs"]
pub mod sv_bar_plot;
pub use sv_bar_plot::{SVBarPlot, SVBarPlotBuilder, SVBarRenderOutput};

#[path = "box_plot/mod.rs"]
pub mod sv_box_plot;
pub use sv_box_plot::{SVBoxPlot, SVBoxPlotBuilder, SVBoxRenderOutput};

#[path = "boxen_plot/mod.rs"]
pub mod sv_boxen_plot;
pub use sv_boxen_plot::{SVBoxenPlot, SVBoxenPlotBuilder, SVBoxenRenderOutput};

#[path = "count_plot/mod.rs"]
pub mod sv_count_plot;
pub use sv_count_plot::{
    SVCountFrame, SVCountPlot, SVCountPlotBuilder, SVCountRenderOutput, SVCountScene,
};

#[path = "density_contour_plot/mod.rs"]
pub mod sv_density_contour_plot;
pub use sv_density_contour_plot::{
    SVDensityContourComposition, SVDensityContourPlot, SVDensityContourPlotBuilder,
};

#[path = "density_plot/mod.rs"]
pub mod sv_density_plot;
pub use sv_density_plot::{SVDensityPlot, SVDensityPlotBuilder};

#[path = "ecdf_plot/mod.rs"]
pub mod sv_ecdf_plot;
pub use sv_ecdf_plot::{SVECdfPlot, SVECdfPlotBuilder, SVEcdfPlotComposition};

#[path = "histogram_plot/mod.rs"]
pub mod sv_histogram_plot;
pub use sv_histogram_plot::{
    SVHistogramFrame, SVHistogramPlot, SVHistogramPlotBuilder, SVHistogramRenderOutput,
    SVHistogramScene,
};

#[path = "rug_plot/mod.rs"]
pub mod sv_rug_plot;
pub use sv_rug_plot::{SVRugFrame, SVRugPlot, SVRugPlotBuilder, SVRugRenderOutput, SVRugScene};

#[path = "strip_plot/mod.rs"]
pub mod sv_strip_plot;
pub use sv_strip_plot::{
    SVStripFrame, SVStripPlot, SVStripPlotBuilder, SVStripRenderOutput, SVStripScene,
};

#[path = "swarm_plot/mod.rs"]
pub mod sv_swarm_plot;
pub use sv_swarm_plot::{
    SVSwarmFrame, SVSwarmPlot, SVSwarmPlotBuilder, SVSwarmRenderOutput, SVSwarmScene,
};

#[path = "violin_plot/mod.rs"]
pub mod sv_violin_plot;
pub use sv_violin_plot::{SVViolinPlot, SVViolinPlotBuilder, SVViolinRenderOutput};
