//! `SVViolinPlot`: the symmetric density silhouette of each category (FDR-117).
//!
//! The chart is the composition
//! `data -> [SVStat::KernelDensity per category] -> (grid, density per category)
//! -> [SVMove::Dodge] -> [SVScale] -> SVMark::Area` recorded in FDR-117. It
//! introduces no kernel variant: the estimate comes from
//! [`sv_stat`](scienceview_core::sv_stat), the placement from
//! [`sv_scale`](scienceview_core::sv_scale) and the silhouette from the area
//! mark of [`sv_mark`](scienceview_core::sv_mark).
//!
//! This module is a pure dispatcher (patterns.md P3): every implementation
//! lives in a named sibling file.

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod data_contract;
mod decoration;
mod grouping;
mod missing_value_policy;
mod output;
mod scale_mode;
mod scales;
mod statistics;
mod widths;

pub use builder::SVViolinPlotBuilder;
pub use core_implementation::SVViolinPlot;
pub use missing_value_policy::SVViolinMissingValuePolicy;
pub use output::SVViolinRenderOutput;
pub use scale_mode::SVViolinScale;
pub use statistics::SVViolinSummary;

#[cfg(test)]
#[path = "violin_plot_concurrency_tests.rs"]
mod violin_plot_concurrency_tests;
#[cfg(all(test, feature = "render-native"))]
#[path = "violin_plot_native_tests.rs"]
mod violin_plot_native_tests;
#[cfg(test)]
#[path = "violin_plot_observability_tests.rs"]
mod violin_plot_observability_tests;
#[cfg(test)]
#[path = "violin_plot_parity_tests.rs"]
mod violin_plot_parity_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "violin_plot_svg_tests.rs"]
mod violin_plot_svg_tests;
#[cfg(test)]
#[path = "violin_plot_test_support.rs"]
mod violin_plot_test_support;
#[cfg(test)]
#[path = "violin_plot_tests.rs"]
mod violin_plot_tests;
