//! `SVPairPlot`: the categorical scatterplot matrix.
//!
//! The chart is exactly the composition of FDR-144:
//! `data -> [no stat] -> values -> [SVMove::Dodge (+ Jitter) within each cell]
//! -> [SVScale per axis] -> SVMark::Dot, one cell per pair`. Every cell shares
//! the domain of its column and row, the diagonal shows the distribution of a
//! variable per hue group, and the off-diagonal shape is one of the closed
//! recipe forms.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_multivariate::sv_pair_plot::SVPairPlot;
//!
//! let horizontal = [0.0, 1.0, 2.0, 3.0];
//! let vertical = [1.0, 4.0, 2.0, 5.0];
//! let depth = [2.0, 3.0, 4.0, 5.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("horizontal", SVColumnRole::X, &horizontal)
//!     .with_column("vertical", SVColumnRole::Y, &vertical)
//!     .with_column("depth", SVColumnRole::Y, &depth)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVPairPlot::builder()
//!     .data_source(&data)
//!     .variables(&["horizontal", "vertical"])
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the pair grid composes");
//!
//! assert_eq!(scene.cell_rectangles().len(), 4);
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
mod layout;
mod off_diagonal;
mod options;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVPairPlotBuilder;
pub use core_implementation::SVPairPlot;
pub use options::{SVPairDiagonalKind, SVPairMissingValuePolicy, SVPairOffDiagonalKind};
pub use render_output::{SVPairFrame, SVPairRenderOutput};
pub use scene::SVPairScene;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "pair_plot_tests.rs"]
mod pair_plot_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
