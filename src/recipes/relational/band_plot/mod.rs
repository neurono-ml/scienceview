//! `SVBandPlot`: the filled region between two boundary lines (FDR-105).
//!
//! The recipe is exactly the composition
//! `data -> [SVStat::Aggregate + error] -> (lower, upper) -> [no move] ->
//! [SVScale] -> SVMark::Ribbon`. It composes the existing kernels and adds no
//! mark, stat, move or scale variant (patterns P5).
//!
//! ```
//! use scienceview::recipes::relational::SVBandPlotBuilder;
//! use scienceview_core::sv_backend::SVBackendKind;
//! use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
//!
//! let x = [0.0, 1.0, 2.0];
//! let lower = [0.5, 0.8, 1.2];
//! let upper = [1.5, 2.2, 3.1];
//! let table = SVDataTable::builder()
//!     .with_column("x", SVColumnRole::X, &x)
//!     .with_column("lower", SVColumnRole::Y, &lower)
//!     .with_column("upper", SVColumnRole::Y, &upper)
//!     .build()
//!     .expect("the columns share one length");
//!
//! let band = SVBandPlotBuilder::new()
//!     .data_source(&table)
//!     .x("x")
//!     .lower("lower")
//!     .upper("upper")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete");
//!
//! assert_eq!(band.render().expect("the band draws").number_of_points(), 3);
//! ```

mod builder;
mod column_access;
mod composition;
mod composition_geometry;
mod configuration;
mod core_implementation;
mod data_contract;
mod decoration;
mod derived_contract;
mod missing_value_policy;
mod output;
mod scales;

pub use builder::SVBandPlotBuilder;
pub use core_implementation::SVBandPlot;
pub use missing_value_policy::SVMissingValuePolicy;
pub use output::SVBandRenderOutput;

#[cfg(test)]
#[path = "band_plot_concurrency_tests.rs"]
mod band_plot_concurrency_tests;
#[cfg(test)]
#[path = "band_plot_derived_tests.rs"]
mod band_plot_derived_tests;
#[cfg(test)]
#[path = "band_plot_missing_tests.rs"]
mod band_plot_missing_tests;
#[cfg(test)]
#[path = "band_plot_observability_tests.rs"]
mod band_plot_observability_tests;
#[cfg(test)]
#[path = "band_plot_parity_tests.rs"]
mod band_plot_parity_tests;
#[cfg(test)]
#[path = "band_plot_series_tests.rs"]
mod band_plot_series_tests;
#[cfg(all(test, feature = "render-svg"))]
#[path = "band_plot_svg_tests.rs"]
mod band_plot_svg_tests;
#[cfg(test)]
#[path = "band_plot_test_support.rs"]
mod band_plot_test_support;
#[cfg(test)]
#[path = "band_plot_tests.rs"]
mod band_plot_tests;
