//! `SVBarPlot`: the aggregate value of each category as a bar (FDR-121).
//!
//! The chart is exactly the composition of FDR-121:
//! `data -> [SVStat::Aggregate] -> (value per category, optional interval)
//! -> [SVMove::Dodge | Stack] -> [SVScale] -> SVMark::Bar (+ error bars)`.
//! It adds no kernel variant: the estimator and the interval come from
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate), the grouping
//! layout from [`sv_move`](crate::sv_move), placement from
//! [`sv_scale`](crate::sv_scale), and the geometry from the bar, error-bar and
//! text marks of [`sv_mark`](crate::sv_mark).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_bar_plot::SVBarPlotBuilder;
//!
//! let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
//! let categories = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::Y, &values)
//!     .with_column("category", SVColumnRole::Grouping, &categories)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let output = SVBarPlotBuilder::new()
//!     .data_source(&data)
//!     .value("value")
//!     .category("category")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .render()
//!     .expect("the bar plot draws");
//!
//! assert_eq!(output.bins().len(), 3);
//! ```
//!
//! `render()` produces backend-agnostic geometry. The output serializes it as
//! an SVG document for `SVBackendKind::Svg`, as a native frame for
//! `SVBackendKind::Native`, and reports a typed error for a kind the recipe
//! does not serialize yet.

mod arrangement;
mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod data_contract;
mod decoration;
mod missing_value_policy;
mod ordering;
mod output;
mod scales;
mod statistics;

pub use arrangement::SVBarArrangement;
pub use builder::SVBarPlotBuilder;
pub use core_implementation::SVBarPlot;
pub use missing_value_policy::SVBarMissingValuePolicy;
pub use ordering::SVBarOrdering;
pub use output::SVBarRenderOutput;
pub use statistics::{SVBarBin, SVBarSummary};

#[cfg(test)]
#[path = "bar_plot_tests.rs"]
mod bar_plot_tests;
#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(all(test, feature = "render-native"))]
#[path = "native_tests.rs"]
mod native_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "svg_tests.rs"]
mod svg_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
