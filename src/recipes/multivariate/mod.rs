//! Multivariate recipes: charts that read several variables at once.
//!
//! Every chart in this family appends one `#[path] pub mod` and one
//! `pub use` to this dispatcher; the family batch integrator resolves the
//! ordering, so recipe changes never edit a central registry (patterns.md
//! P4). Recipe module names carry the `sv_` prefix so the family scope
//! never collides.

#[path = "parallel_categories_plot/mod.rs"]
pub mod sv_parallel_categories_plot;
pub use sv_parallel_categories_plot::{
    SVCategoryOrder, SVParallelCategoriesAxis, SVParallelCategoriesColourMode,
    SVParallelCategoriesFrame, SVParallelCategoriesMissingValuePolicy, SVParallelCategoriesPlot,
    SVParallelCategoriesPlotBuilder, SVParallelCategoriesRenderOutput, SVParallelCategoriesRibbon,
    SVParallelCategoriesScene,
};
