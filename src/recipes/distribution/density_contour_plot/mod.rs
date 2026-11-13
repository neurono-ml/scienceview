//! `SVDensityContourPlot`: the two-dimensional density contour recipe.
//!
//! The chart is the composition of FDR-113:
//! `data -> [SVStat::Bin on x and y] -> (joint grid, density) -> [marching
//! squares] -> [SVScale x, y, colour] -> contour paths`.
//!
//! The closed stat vocabulary has no bivariate density variant (patterns P5),
//! so the recipe derives the two axes through [`SVStat::Bin`] and folds every
//! observation pair into the joint grid; sparse views are walked through their
//! stored entries and streaming views through their current window.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::SVDataTableBuilder;
//! use scienceview::sv_data::SVColumnRole;
//! use scienceview::sv_recipes::sv_distribution::sv_density_contour_plot::SVDensityContourPlot;
//!
//! let horizontal = [0.0, 1.0, 0.5, 2.0, 2.5, 1.2];
//! let vertical = [0.1, 0.2, 1.0, 2.0, 2.2, 1.5];
//! let data = SVDataTableBuilder::new()
//!     .with_column("x", SVColumnRole::X, &horizontal)
//!     .with_column("y", SVColumnRole::Y, &vertical)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let composition = SVDensityContourPlot::builder()
//!     .data_source(&data)
//!     .x("x")
//!     .y("y")
//!     .grid_count(16)
//!     .levels(4)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the contour composes");
//!
//! assert_eq!(composition.grids().len(), 1);
//! assert_eq!(composition.number_of_levels(), 4);
//! ```

mod builder;
mod builder_options;
mod builder_validation;
mod composition;
mod configuration;
mod contour;
mod core_implementation;
mod decorations;
mod grid;
mod levels;
mod observation_pairs;
mod options;
mod rendering;

pub use composition::SVDensityContourComposition;
pub use grid::SVDensityGrid;
pub use options::SVContourStyle;

pub use builder::SVDensityContourPlotBuilder;
pub use core_implementation::SVDensityContourPlot;

#[cfg(test)]
#[path = "density_contour_plot_tests.rs"]
mod density_contour_plot_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
