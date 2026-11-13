//! Relational recipes: charts whose two visual channels are continuous or
//! temporal positions.
//!
//! Every chart appends one `pub mod` line and one `pub use` line to the marked
//! block below. The block is shared by the whole family batch, so a recipe
//! change only ever adds its own lines and never reorders its siblings.

// ## Relational recipe registrations (append one pair of lines per chart)
pub mod area_plot;

pub use area_plot::{SVAreaInterpolation, SVAreaPlot, SVAreaPlotBuilder};
