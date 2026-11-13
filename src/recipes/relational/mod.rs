//! Relational recipes: charts whose two visual channels are continuous or
//! temporal positions.
//!
//! Each chart appends one `pub mod` line and one `pub use` line to the marked
//! block below. The block is shared by the whole family batch, so a recipe
//! change only ever adds its own lines and never reorders its siblings.

// ## Relational recipe registrations (append one pair of lines per chart)
pub mod band_plot;
pub use band_plot::{SVBandPlot, SVBandPlotBuilder, SVBandRenderOutput, SVMissingValuePolicy};
