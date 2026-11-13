//! `SVClusteredHeatmapPlot`: the reordered heatmap with two dendrograms.
//!
//! The chart is a renderer of a supplied clustering, not a clustering engine.
//! It consumes the matrix (dense, or a long table pivoted through
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate)) together with a
//! precomputed row order, a precomputed column order and the two merge trees,
//! and draws the reordered cells plus the dendrograms. It adds no kernel
//! variant: the placement reuses [`sv_scale`](crate::sv_scale), the cells and
//! the dendrogram paths are [`sv_mark`](crate::sv_mark) primitives and the
//! colour bar is the [`SVColourBar`](crate::sv_decoration::SVColourBar)
//! decoration.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_recipes::sv_matrix::sv_clustered_heatmap_plot::{
//!     SVClusterMerge, SVClusterTree, SVClusteredHeatmapPlot, SVHeatmapMatrix,
//! };
//!
//! let values = [1.0, 2.0, 3.0, 4.0];
//! let matrix = SVHeatmapMatrix::new(&values, 2, 2).expect("the shape matches");
//! let tree = SVClusterTree::new(&[SVClusterMerge::new(0, 1, 1.0, 2)]);
//! let row_order = [1, 0];
//! let column_order = [0, 1];
//! let scene = SVClusteredHeatmapPlot::builder()
//!     .matrix(matrix)
//!     .row_order(&row_order)
//!     .column_order(&column_order)
//!     .row_linkage(tree)
//!     .column_linkage(tree)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the clustered heatmap composes");
//!
//! assert_eq!(scene.number_of_marks(), 4);
//! assert_eq!(scene.row_labels(), ["row 1", "row 0"]);
//! ```
//!
//! A streaming view is rejected up front with
//! [`SVError::UnsupportedRepresentation`](crate::sv_error::SVError::UnsupportedRepresentation):
//! the supplied order and tree describe a whole snapshot. The documented
//! alternative is to take a snapshot of the current window, compute the
//! clustering outside `scienceview`, then render that snapshot.

mod builder;
mod composition;
mod configuration;
mod core_implementation;
mod options;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVClusteredHeatmapPlotBuilder;
pub use core_implementation::SVClusteredHeatmapPlot;
pub use options::{SVClusterAxis, SVClusterMerge, SVClusterTree};
pub use render_output::{SVClusteredHeatmapFrame, SVClusteredHeatmapRenderOutput};
pub use scene::{SVClusteredHeatmapCell, SVClusteredHeatmapScene, SVDendrogram, SVDendrogramMerge};

pub use crate::sv_recipes::sv_matrix::sv_heatmap_plot::{SVHeatmapMatrix, SVMissingValue};

#[cfg(test)]
#[path = "clustered_heatmap_plot_tests.rs"]
mod clustered_heatmap_plot_tests;
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
#[path = "test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "tree_tests.rs"]
mod tree_tests;
