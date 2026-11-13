//! Relational charts: one position against another.
//!
//! The family module is the registration boundary: each recipe appends one
//! `pub mod` and one `pub use` here, and the family batch integrator resolves
//! the ordering (patterns.md, P4).

pub mod step_plot;

pub use step_plot::{SVStepDuplicatePolicy, SVStepPlot, SVStepPlotBuilder, SVStepPosition};
