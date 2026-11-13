//! The relational family: charts that relate two continuous or nominal
//! dimensions — scatter, line, area, step, band and regression.
//!
//! Every chart in this family appends one `#[path] pub mod` and one `pub use`
//! to this dispatcher; the family batch integrator resolves the ordering, so
//! recipe changes never edit a central registry (patterns.md P4). Recipe
//! module names carry the `sv_` prefix so the family scope never collides.

#[path = "regression_plot/mod.rs"]
pub mod sv_regression_plot;

pub use sv_regression_plot::{
    SVExtrapolationPolicy, SVRegressionComposition, SVRegressionConfiguration, SVRegressionPlot,
    SVRegressionPlotBuilder, SVRegressionSeriesFit,
};
