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

#[path = "area_plot/mod.rs"]
pub mod sv_area_plot;
pub use sv_area_plot::{SVAreaPlot, SVAreaPlotBuilder, SVAreaRendering};

#[path = "line_plot/mod.rs"]
pub mod sv_line_plot;
pub use sv_line_plot::{SVLinePlot, SVLinePlotBuilder};

#[path = "scatter_plot/mod.rs"]
pub mod sv_scatter_plot;
pub use sv_scatter_plot::{SVScatterPlot, SVScatterPlotBuilder, SVScatterScene};

#[path = "step_plot/mod.rs"]
pub mod sv_step_plot;
pub use sv_step_plot::{SVStepDuplicatePolicy, SVStepPlot, SVStepPlotBuilder, SVStepPosition};
