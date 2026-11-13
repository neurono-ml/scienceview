//! `SVParallelCategoriesPlot`: the multivariate parallel categories recipe.
//!
//! The chart is the composition of FDR-142:
//! `data -> [SVStat::Count] -> category bands per axis -> [SVScale per axis
//! (nominal bands)] -> SVMark::Ribbon`. It adds no kernel variant: counting
//! goes to [`SVStat::Count`](crate::sv_stat::SVStat::Count), placement to
//! [`sv_scale`](crate::sv_scale), and the ribbon geometry to the ribbon mark of
//! [`sv_mark`](crate::sv_mark).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_multivariate::sv_parallel_categories_plot::SVParallelCategoriesPlot;
//!
//! let first = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0];
//! let second = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("first", SVColumnRole::Grouping, &first)
//!     .with_column("second", SVColumnRole::Grouping, &second)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVParallelCategoriesPlot::builder()
//!     .data_source(&data)
//!     .variables(&["first", "second"])
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the parallel categories plot composes");
//!
//! assert_eq!(scene.number_of_axes(), 2);
//! assert_eq!(scene.number_of_observations(), 6);
//! ```
//!
//! `render()` serializes the scene through the path the configured kind
//! actually has: an SVG document for `SVBackendKind::Svg`, a native frame for
//! `SVBackendKind::Native`, and a typed error for a kind the recipe does not
//! serialize yet.

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod geometry;
mod options;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVParallelCategoriesPlotBuilder;
pub use core_implementation::SVParallelCategoriesPlot;
pub use options::{
    SVCategoryOrder, SVParallelCategoriesColourMode, SVParallelCategoriesMissingValuePolicy,
};
pub use render_output::{SVParallelCategoriesFrame, SVParallelCategoriesRenderOutput};
pub use scene::{SVParallelCategoriesAxis, SVParallelCategoriesRibbon, SVParallelCategoriesScene};

#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "parallel_categories_plot_tests.rs"]
mod parallel_categories_plot_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
