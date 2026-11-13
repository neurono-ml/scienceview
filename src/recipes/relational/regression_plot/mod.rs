//! `SVRegressionPlot` — a fitted model overlaid on a scatter.
//!
//! The chart is exactly the composition
//! `data -> SVStat::Fit -> (coefficients, curve) -> SVScale -> Dot + Line + Ribbon`
//! from `docs/FDRs/recipes/relational-regression-plot.md` (FDR-106). It never
//! reimplements the fit: model selection, the polynomial order and the
//! confidence band all go to the `Fit` kernel, and the recipe groups, pairs and
//! places what the kernel returns.
//!
//! ```
//! use scienceview_core::sv_backend::SVBackendKind;
//! use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
//! use scienceview_core::sv_stat::SVFitModel;
//! use scienceview::recipes::relational::{SVRegressionPlot, SVRegressionPlotBuilder};
//!
//! let x = [0.0, 1.0, 2.0, 3.0, 4.0];
//! let y = [1.0, 3.0, 5.0, 7.0, 9.0];
//! let table = SVDataTable::builder()
//!     .with_column("x", SVColumnRole::X, &x)
//!     .with_column("y", SVColumnRole::Y, &y)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let plot = SVRegressionPlotBuilder::new()
//!     .data_source(&table)
//!     .x("x")
//!     .y("y")
//!     .model(SVFitModel::Linear)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete");
//!
//! let composition = plot.compose().expect("the fit runs");
//! assert_eq!(composition.series().len(), 1);
//! assert!((composition.series()[0].coefficients()[1] - 2.0).abs() < 1.0e-9);
//! ```

mod builder;
mod builder_options;
mod builder_state;
mod builder_validation;
mod composition;
mod configuration;
mod core_implementation;
mod domains;
mod extrapolation;
mod fitting;
mod grouping;
mod pairing;
#[cfg(feature = "render-svg")]
mod render_svg;
mod sparse_pairing;

pub use builder::SVRegressionPlotBuilder;
pub use builder_state::{SVWithBackend, SVWithDataSource, SVWithoutBackend, SVWithoutDataSource};
pub use composition::{SVRegressionComposition, SVRegressionSeriesFit};
pub use configuration::SVRegressionConfiguration;
pub use core_implementation::SVRegressionPlot;
pub use extrapolation::SVExtrapolationPolicy;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "regression_plot_tests/mod.rs"]
mod regression_plot_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "svg_tests.rs"]
mod svg_tests;
