//! `SVPointPlot`: the point estimate of each category and its uncertainty
//! (FDR-123).
//!
//! The chart is exactly the composition of FDR-123:
//! `data -> [SVStat::Aggregate + interval] -> (estimate, lower, upper) per
//! category -> [SVMove::Dodge] -> [SVScale] -> SVMark::Dot + error bar +
//! optional Line`. It adds no kernel variant: the estimator and the interval
//! come from [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate), the
//! series layout from [`sv_move`](crate::sv_move), placement from
//! [`sv_scale`](crate::sv_scale), and the geometry from the dot, error-bar and
//! line marks of [`sv_mark`](crate::sv_mark).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_point_plot::SVPointPlotBuilder;
//!
//! let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
//! let categories = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::Y, &values)
//!     .with_column("category", SVColumnRole::Grouping, &categories)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let output = SVPointPlotBuilder::new()
//!     .data_source(&data)
//!     .value("value")
//!     .category("category")
//!     .join(true)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .render()
//!     .expect("the point plot draws");
//!
//! assert_eq!(output.estimates().len(), 3);
//! ```
//!
//! `render()` produces backend-agnostic geometry. The output serializes it as
//! an SVG document for `SVBackendKind::Svg`, as a native frame for
//! `SVBackendKind::Native`, and reports a typed error for a kind the recipe
//! does not serialize yet.

mod aggregation;
mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod data_contract;
mod decoration;
mod missing_value_policy;
mod ordering;
mod output;
mod placement;
mod scales;
mod statistics;

pub use builder::SVPointPlotBuilder;
pub use core_implementation::SVPointPlot;
pub use missing_value_policy::SVPointMissingValuePolicy;
pub use ordering::SVPointOrdering;
pub use output::SVPointRenderOutput;
pub use statistics::{SVPointEstimate, SVPointSummary};

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
#[cfg(test)]
#[path = "point_plot_tests.rs"]
mod point_plot_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "svg_tests.rs"]
mod svg_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
