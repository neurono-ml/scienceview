//! `SVCountPlot`: the distributional count recipe.
//!
//! The chart is the composition of FDR-122:
//! `data -> [SVStat::Count] -> (count per category) -> [SVMove::Stack | Dodge] -> [SVScale x, y, colour] -> SVMark::Bar`.
//! It adds no kernel variant: counting goes to
//! [`SVStat::Count`](crate::sv_stat::SVStat::Count), stacking and dodging to
//! [`sv_move`](crate::sv_move), placement to [`sv_scale`](crate::sv_scale), and
//! the geometry to the bar mark of [`sv_mark`](crate::sv_mark).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_count_plot::SVCountPlot;
//!
//! let categories = [0.0, 1.0, 1.0, 2.0, 2.0, 2.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("category", SVColumnRole::Grouping, &categories)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVCountPlot::builder()
//!     .data_source(&data)
//!     .category("category")
//!     .category_labels(&["setosa", "versicolor", "virginica"])
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the count plot composes");
//!
//! assert_eq!(scene.counts(), &[1.0, 2.0, 3.0]);
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
mod missing_value_policy;
mod multiple;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVCountPlotBuilder;
pub use core_implementation::SVCountPlot;
pub use missing_value_policy::SVCountMissingValuePolicy;
pub use multiple::SVCountMultiple;
pub use render_output::{SVCountFrame, SVCountRenderOutput};
pub use scene::{SVCountScene, SVCountSeriesSummary};

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "count_plot_tests.rs"]
mod count_plot_tests;
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
