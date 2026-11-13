//! Multivariate recipes: charts that show several variables at once.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "joint_plot/mod.rs"]
pub mod sv_joint_plot;
pub use sv_joint_plot::{
    SVJointFrame, SVJointPlot, SVJointPlotBuilder, SVJointRenderOutput, SVJointScene,
};

#[path = "pair_plot/mod.rs"]
pub mod sv_pair_plot;
pub use sv_pair_plot::{
    SVPairFrame, SVPairPlot, SVPairPlotBuilder, SVPairRenderOutput, SVPairScene,
};

#[path = "parallel_categories_plot/mod.rs"]
pub mod sv_parallel_categories_plot;
pub use sv_parallel_categories_plot::{
    SVParallelCategoriesFrame, SVParallelCategoriesPlot, SVParallelCategoriesPlotBuilder,
    SVParallelCategoriesRenderOutput, SVParallelCategoriesScene,
};

#[path = "parallel_coordinates_plot/mod.rs"]
pub mod sv_parallel_coordinates_plot;
pub use sv_parallel_coordinates_plot::{
    SVParallelCoordinatesPlot, SVParallelCoordinatesPlotBuilder, SVParallelCoordinatesRenderOutput,
};

#[path = "splom_plot/mod.rs"]
pub mod sv_splom_plot;
pub use sv_splom_plot::{
    SVSplomFrame, SVSplomPlot, SVSplomPlotBuilder, SVSplomRenderOutput, SVSplomScene,
};
