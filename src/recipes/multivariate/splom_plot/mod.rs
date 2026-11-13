//! `SVSplomPlot`: the multivariate scatterplot matrix.
//!
//! A scatterplot matrix is a grid of scatter plots, one per pair of variables,
//! with the observations shared across the cells. The chart is the composition
//! of FDR-143: the variables come from [`sv_data`](crate::sv_data), the diagonal
//! distribution from [`sv_stat`](crate::sv_stat), the axes from
//! [`sv_scale`](crate::sv_scale), the grid from [`sv_layout`](crate::sv_layout),
//! the dots, bars and area from [`sv_mark`](crate::sv_mark), and the borders,
//! labels and legend from [`sv_decoration`](crate::sv_decoration).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_multivariate::sv_splom_plot::SVSplomPlot;
//!
//! let height = [1.0, 2.0, 3.0, 4.0, 5.0];
//! let weight = [2.0, 4.0, 5.0, 8.0, 9.0];
//! let age = [20.0, 25.0, 30.0, 35.0, 40.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("height", SVColumnRole::X, &height)
//!     .with_column("weight", SVColumnRole::Y, &weight)
//!     .with_column("age", SVColumnRole::Grouping, &age)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVSplomPlot::builder()
//!     .data_source(&data)
//!     .variables(&["height", "weight", "age"])
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the matrix composes");
//!
//! assert_eq!(scene.number_of_variables(), 3);
//! assert_eq!(scene.number_of_cells(), 9);
//! ```
//!
//! `render()` serializes the scene through the path the configured kind
//! actually has: an SVG document for `SVBackendKind::Svg`, a native frame for
//! `SVBackendKind::Native`, and a typed error for a kind the recipe does not
//! serialize yet.

mod builder;
mod builder_options;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod diagonal;
mod geometry;
mod matrix;
mod options;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVSplomPlotBuilder;
pub use core_implementation::SVSplomPlot;
pub use options::{SVSplomDiagonal, SVSplomMissingValuePolicy, SVSplomScaleMode};
pub use render_output::{SVSplomFrame, SVSplomRenderOutput};
pub use scene::{SVSplomCell, SVSplomCellKind, SVSplomScene};

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
#[path = "splom_plot_tests.rs"]
mod splom_plot_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
