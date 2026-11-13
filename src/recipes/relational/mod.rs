//! Relational recipes: charts whose two visual channels are positions.
//!
//! Every chart in this family appends one `pub mod` and one `pub use` to the
//! marked block below. The block is shared by the whole family batch, so a
//! recipe change only ever adds its own lines and never reorders its siblings
//! (patterns P4).

// ## Relational recipe registrations (append one pair of lines per chart)
#[path = "line_plot/mod.rs"]
pub mod sv_line_plot;
pub use sv_line_plot::{SVLegendPlacement, SVLineInterpolation, SVLinePlot, SVLinePlotBuilder};
