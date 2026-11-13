//! Relational recipes: charts whose two visual channels are continuous or
//! temporal positions.
//!
//! Each chart appends one `pub mod` line and one `pub use` line to the marked
//! block below. The block is shared by the whole family batch, so a recipe
//! change only ever adds its own lines and never reorders its siblings.
//!
//! The module is renamed to the `sv_`-prefixed public name through `#[path]`,
//! so the physical folder keeps the short name while the public path follows
//! the naming rule. Recipe names are not re-exported into the family scope:
//! two recipes may each carry a `SV…MissingValuePolicy`, and a family-wide
//! `pub use` would make them collide. Callers name the recipe module.

// ## Relational recipe registrations (append one pair of lines per chart)

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

#[path = "band_plot/mod.rs"]
pub mod sv_band_plot;
pub use sv_band_plot::{SVBandPlot, SVBandPlotBuilder, SVBandRenderOutput};
