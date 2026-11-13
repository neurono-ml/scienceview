//! `SVDensityPlot`: the smoothed distribution of a continuous variable.
//!
//! The chart is the composition
//! `data -> [SVStat::KernelDensity] -> (grid, density) -> [no move] -> [SVScale]
//! -> SVMark::Line | SVMark::Area` recorded in FDR-112. It introduces no kernel
//! variant: the estimate comes from [`sv_stat`](scienceview_core::sv_stat), the
//! placement from [`sv_scale`](scienceview_core::sv_scale) and the curve from
//! the line and area marks of [`sv_mark`](scienceview_core::sv_mark).
//!
//! This module is a pure dispatcher (patterns.md P3): every implementation
//! lives in a named sibling file.

mod builder;
mod configuration;
mod core_implementation;
mod missing_value_policy;
mod preparation;
mod rendering;

pub use builder::SVDensityPlotBuilder;
pub use core_implementation::SVDensityPlot;
pub use missing_value_policy::SVDensityMissingValuePolicy;
pub use rendering::{SVDensityCurve, SVDensityRendering};

#[cfg(test)]
#[path = "density_plot_tests.rs"]
mod density_plot_tests;
