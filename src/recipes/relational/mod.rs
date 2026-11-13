//! Relational charts: scatter, line, area, step, band and regression.
//!
//! The family re-exports the plot types only. Option enumerations stay
//! qualified under their recipe module, so two recipes that grow an option with
//! the same bare name never collide in the family scope.

#[path = "scatter_plot/mod.rs"]
pub mod sv_scatter_plot;

pub use sv_scatter_plot::{SVScatterPlot, SVScatterPlotBuilder, SVScatterScene};
