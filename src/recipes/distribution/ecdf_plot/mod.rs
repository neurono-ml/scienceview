//! `SVECdfPlot`: the empirical cumulative distribution function (FDR-114).
//!
//! The recipe is exactly the composition
//! `data -> [SVStat::EmpiricalCdf] -> (values, cumulative probability) ->
//! [no move] -> [SVScale] -> SVMark::Line (step geometry)`. It composes the
//! existing kernels and adds no mark, stat, move or scale variant (patterns
//! P5).
//!
//! ```
//! use scienceview::sv_recipes::sv_distribution::sv_ecdf_plot::SVECdfPlotBuilder;
//! use scienceview_core::sv_backend::SVBackendKind;
//! use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
//!
//! let values = [1.0, 3.0, 2.0, 3.0];
//! let table = SVDataTable::builder()
//!     .with_column("value", SVColumnRole::Y, &values)
//!     .build()
//!     .expect("the column is valid");
//!
//! let ecdf = SVECdfPlotBuilder::new()
//!     .data_source(&table)
//!     .value("value")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete");
//!
//! assert_eq!(ecdf.render().expect("the curve draws").point_count(), 4);
//! ```

mod builder;
mod composition;
mod core_implementation;
mod data_contract;
mod decorations;
mod render;

pub use builder::SVECdfPlotBuilder;
pub use composition::SVEcdfPlotComposition;
pub use core_implementation::SVECdfPlot;

#[cfg(test)]
#[path = "ecdf_plot_tests/mod.rs"]
mod ecdf_plot_tests;
