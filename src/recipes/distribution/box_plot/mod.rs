//! `SVBoxPlot`: the five-number distribution summary (FDR-116).

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod data_contract;
mod decoration;
mod grouping;
mod output;
mod scales;
mod statistics;
mod whisker;

pub use builder::SVBoxPlotBuilder;
pub use core_implementation::SVBoxPlot;
pub use output::SVBoxRenderOutput;
pub use statistics::SVBoxSummary;
pub use whisker::SVWhiskerRule;

#[cfg(test)]
#[path = "box_plot_concurrency_tests.rs"]
mod box_plot_concurrency_tests;
#[cfg(all(test, feature = "render-native"))]
#[path = "box_plot_native_tests.rs"]
mod box_plot_native_tests;
#[cfg(test)]
#[path = "box_plot_observability_tests.rs"]
mod box_plot_observability_tests;
#[cfg(test)]
#[path = "box_plot_parity_tests.rs"]
mod box_plot_parity_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "box_plot_svg_tests.rs"]
mod box_plot_svg_tests;
#[cfg(test)]
#[path = "box_plot_test_support.rs"]
mod box_plot_test_support;
#[cfg(test)]
#[path = "box_plot_tests.rs"]
mod box_plot_tests;
