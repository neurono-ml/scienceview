//! Relational recipes: charts whose two visual channels are positions.
//!
//! Every chart in this family appends one `pub mod` and one `pub use` to the
//! marked block below. The block is shared by the whole family batch, so a
//! recipe change only ever adds its own lines and never reorders its siblings
//! (patterns P4).
//!
//! The family re-exports the plot types only. Option enumerations stay
//! qualified under their recipe module, so two recipes that grow an option with
//! the same bare name never collide in the family scope.

// ## Relational recipe registrations (append one pair of lines per chart)
#[path = "scatter_plot/mod.rs"]
pub mod sv_scatter_plot;
pub use sv_scatter_plot::{SVScatterPlot, SVScatterPlotBuilder, SVScatterScene};

#[path = "line_plot/mod.rs"]
pub mod sv_line_plot;
pub use sv_line_plot::{SVLinePlot, SVLinePlotBuilder};
