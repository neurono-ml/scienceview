//! `SVAreaPlot`: fills the region between a line and a baseline.
//!
//! The chart is the composition
//! `data -> [SVStat::Aggregate] -> values -> [SVMove::Stack] -> [SVScale] -> SVMark::Area`
//! recorded in FDR-103. It introduces no kernel variant: the stack comes from
//! [`sv_move`](scienceview_core::sv_move), the placement from
//! [`sv_scale`](scienceview_core::sv_scale) and the region from the area,
//! ribbon and line marks of [`sv_mark`](scienceview_core::sv_mark).
//!
//! This module is a pure dispatcher (patterns.md P3): every implementation
//! lives in a named sibling file.

mod builder;
mod configuration;
mod core_implementation;
mod missing_value_policy;
mod preparation;
mod rendering;

pub use builder::SVAreaPlotBuilder;
pub use configuration::SVAreaInterpolation;
pub use core_implementation::SVAreaPlot;
pub use missing_value_policy::SVMissingValuePolicy;
pub use rendering::SVAreaRendering;

#[cfg(test)]
#[path = "area_plot_tests.rs"]
mod area_plot_tests;
