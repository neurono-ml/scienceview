//! `SVContourPlot`: contour lines and filled bands over a supplied field.
//!
//! The chart is the composition of FDR-132:
//! `data -> [SVStat::Aggregate (grid interpolation)] -> scalar field ->
//! [contour extraction] -> [SVScale x, y, colour] -> SVMark::Line | SVMark::Area`.
//!
//! A matrix column is read directly onto a lattice; an `x`, `y`, `value` cloud
//! is binned through [`SVStat::Bin`](crate::sv_stat::SVStat::Bin) and averaged
//! per cell. The level extraction lives in the `geometry` submodule, because
//! the core stat vocabulary has no contour variant yet (patterns P5); a future
//! core change may promote it to a shared kernel.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_matrix::sv_contour_plot::SVContourPlot;
//!
//! let field = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::Y, &field)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVContourPlot::builder()
//!     .data_source(&data)
//!     .value("value")
//!     .matrix(3, 3)
//!     .levels(2)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the contour composes");
//!
//! assert_eq!(scene.number_of_levels(), 2);
//! assert_eq!(scene.field().number_of_samples(), 9);
//! ```

mod builder;
mod builder_options;
mod builder_validation;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod field;
mod geometry;
mod levels;
mod options;
mod preparation;
mod rendering;

pub use composition::SVContourComposition;
pub use field::SVContourField;
pub use options::{SVContourMissingPolicy, SVContourStyle};

pub use builder::SVContourPlotBuilder;
pub use core_implementation::SVContourPlot;

#[cfg(test)]
#[path = "contour_plot_tests.rs"]
mod contour_plot_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
