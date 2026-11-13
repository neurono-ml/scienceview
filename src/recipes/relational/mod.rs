//! The relational family: charts that relate two continuous or nominal
//! dimensions — scatter, line, area, step, band and regression.
//!
//! Every chart in this family appends one `#[path] pub mod` and one `pub use`
//! to this dispatcher; the family batch integrator resolves the ordering, so
//! recipe changes never edit a central registry (patterns.md P4). Recipe
//! module names carry the `sv_` prefix so the family scope never collides.

#[path = "area_plot/mod.rs"]
pub mod sv_area_plot;
pub use sv_area_plot::{SVAreaPlot, SVAreaPlotBuilder, SVAreaRendering};

#[path = "band_plot/mod.rs"]
pub mod sv_band_plot;
pub use sv_band_plot::{SVBandPlot, SVBandPlotBuilder, SVBandRenderOutput};

#[path = "line_plot/mod.rs"]
pub mod sv_line_plot;
pub use sv_line_plot::{SVLinePlot, SVLinePlotBuilder};

#[path = "regression_plot/mod.rs"]
pub mod sv_regression_plot;
pub use sv_regression_plot::{
    SVExtrapolationPolicy, SVRegressionComposition, SVRegressionConfiguration, SVRegressionPlot,
    SVRegressionPlotBuilder, SVRegressionSeriesFit,
};

#[path = "scatter_plot/mod.rs"]
pub mod sv_scatter_plot;
pub use sv_scatter_plot::{SVScatterPlot, SVScatterPlotBuilder, SVScatterScene};

#[path = "step_plot/mod.rs"]
pub mod sv_step_plot;
pub use sv_step_plot::{SVStepDuplicatePolicy, SVStepPlot, SVStepPlotBuilder, SVStepPosition};
