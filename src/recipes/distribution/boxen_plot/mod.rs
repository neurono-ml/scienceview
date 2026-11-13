//! `SVBoxenPlot`: the letter-value plot (FDR-118).

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod data_contract;
mod decoration;
mod depth_rule;
mod grouping;
mod output;
mod scales;
mod statistics;

pub use builder::SVBoxenPlotBuilder;
pub use core_implementation::SVBoxenPlot;
pub use depth_rule::SVBoxenTailRule;
pub use output::SVBoxenRenderOutput;
pub use statistics::{SVBoxenLevel, SVBoxenSummary};

#[cfg(test)]
#[path = "boxen_plot_concurrency_tests.rs"]
mod boxen_plot_concurrency_tests;
#[cfg(all(test, feature = "render-native"))]
#[path = "boxen_plot_native_tests.rs"]
mod boxen_plot_native_tests;
#[cfg(test)]
#[path = "boxen_plot_observability_tests.rs"]
mod boxen_plot_observability_tests;
#[cfg(test)]
#[path = "boxen_plot_parity_tests.rs"]
mod boxen_plot_parity_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "boxen_plot_svg_tests.rs"]
mod boxen_plot_svg_tests;
#[cfg(test)]
#[path = "boxen_plot_test_support.rs"]
mod boxen_plot_test_support;
#[cfg(test)]
#[path = "boxen_plot_tests.rs"]
mod boxen_plot_tests;
