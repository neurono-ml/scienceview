//! The relational family: charts that relate two continuous or nominal
//! dimensions — scatter, line, area, step, band and regression.
//!
//! Every chart in this family appends one `pub mod` and one `pub use` to this
//! dispatcher; the family batch integrator resolves the ordering, so recipe
//! changes never edit a central registry (patterns.md P4).

pub mod regression_plot;

pub use regression_plot::{
    SVExtrapolationPolicy, SVRegressionComposition, SVRegressionConfiguration, SVRegressionPlot,
    SVRegressionPlotBuilder, SVRegressionSeriesFit,
};
