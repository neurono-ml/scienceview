//! Relational charts: one position against another.
//!
//! The family module is the registration boundary: each recipe appends one
//! `pub mod` and one `pub use` here, and the family batch integrator resolves
//! the ordering (patterns.md, P4).
//!
//! The public module is `sv_relational`; the folder keeps its physical name
//! through `#[path]`, so the public surface follows the mandatory `sv_`
//! prefix while siblings can append their own registrations without moving
//! files.

#[path = "step_plot/mod.rs"]
pub mod sv_step_plot;

pub use sv_step_plot::{SVStepDuplicatePolicy, SVStepPlot, SVStepPlotBuilder, SVStepPosition};
