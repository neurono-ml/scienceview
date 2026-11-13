//! `SVHeatmapPlot`: the matrix heatmap recipe.
//!
//! The chart is the composition of FDR-131:
//! `data -> [SVStat::Aggregate (pivot into a matrix)] -> (rows, columns, values)
//! -> [SVScale::Continuous over a colour map] -> SVMark::Image | SVMark::Rect`.
//! It adds no kernel variant: the pivot reuses
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate), placement reuses
//! [`sv_scale`](crate::sv_scale), the cells are rectangle primitives of
//! [`sv_mark`](crate::sv_mark) and the colour bar is the
//! [`SVColourBar`](crate::sv_decoration::SVColourBar) decoration.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_recipes::sv_matrix::sv_heatmap_plot::{SVHeatmapMatrix, SVHeatmapPlot};
//!
//! let values = [1.0, 2.0, 3.0, 4.0];
//! let matrix = SVHeatmapMatrix::new(&values, 2, 2).expect("the shape matches");
//! let scene = SVHeatmapPlot::builder()
//!     .matrix(matrix)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the heatmap composes");
//!
//! assert_eq!(scene.number_of_marks(), 4);
//! ```
//!
//! `render()` serializes the scene through the path the configured kind
//! actually has: an SVG document for `SVBackendKind::Svg`, a native frame for
//! `SVBackendKind::Native`, and a typed error for a kind the recipe does not
//! serialize yet.

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod options;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVHeatmapPlotBuilder;
pub use core_implementation::SVHeatmapPlot;
pub use options::{SVHeatmapMatrix, SVHeatmapOrientation, SVMissingValue};
pub use render_output::{SVHeatmapFrame, SVHeatmapRenderOutput};
pub use scene::{SVHeatmapCell, SVHeatmapScene};

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "heatmap_plot_tests.rs"]
mod heatmap_plot_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
