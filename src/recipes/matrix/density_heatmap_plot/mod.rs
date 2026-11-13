//! `SVDensityHeatmapPlot`: the binned two-dimensional density recipe.
//!
//! The chart is the composition of FDR-133:
//! `data -> [SVStat::Bin (2D)] -> (grid, counts) -> [SVScale::Continuous over a
//! colour map] -> SVMark::Image`.
//!
//! Both axes derive their edges through [`SVStat::Bin`](crate::sv_stat::SVStat::Bin)
//! and the per-cell values are folded through
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate), so the recipe adds
//! no kernel variant. Sparse views are walked through their stored entries and
//! streaming views through their current window.

mod builder;
mod builder_options;
mod builder_validation;
mod cell_accumulation;
mod cells;
mod colour_scale;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod grid;
mod observation_pairs;
mod options;
mod rendering;

pub use builder::SVDensityHeatmapPlotBuilder;
pub use composition::SVDensityHeatmapComposition;
pub use core_implementation::SVDensityHeatmapPlot;
pub use grid::SVDensityHeatmapGrid;
pub use options::SVEmptyBinPolicy;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "density_heatmap_plot_tests.rs"]
mod density_heatmap_plot_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
