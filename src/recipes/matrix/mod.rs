//! Matrix recipes: charts whose data is a two-dimensional field.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "contour_plot/mod.rs"]
pub mod sv_contour_plot;
pub use sv_contour_plot::{SVContourComposition, SVContourPlot, SVContourPlotBuilder};

#[path = "density_heatmap_plot/mod.rs"]
pub mod sv_density_heatmap_plot;
pub use sv_density_heatmap_plot::{
    SVDensityHeatmapComposition, SVDensityHeatmapPlot, SVDensityHeatmapPlotBuilder,
};

#[path = "heatmap_plot/mod.rs"]
pub mod sv_heatmap_plot;
pub use sv_heatmap_plot::{
    SVHeatmapFrame, SVHeatmapPlot, SVHeatmapPlotBuilder, SVHeatmapRenderOutput, SVHeatmapScene,
};

#[path = "image_plot/mod.rs"]
pub mod sv_image_plot;
pub use sv_image_plot::{
    SVImageFrame, SVImagePlot, SVImagePlotBuilder, SVImageRenderOutput, SVImageScene,
};
