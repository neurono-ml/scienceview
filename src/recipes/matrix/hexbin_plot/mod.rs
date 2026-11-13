//! `SVHexbinPlot`: hexagonal binning of two continuous variables (FDR-135).
//!
//! The chart is the composition
//! `data -> [hexagonal lattice] -> (cells, counts) -> [SVScale continuous over a
//! colour map] -> one path per hexagon`. The closed stat vocabulary has no
//! bivariate hexagonal binning variant (patterns P5), so the recipe owns the
//! lattice geometry; mean aggregation is delegated to
//! [`SVStat::Aggregate`](crate::sv_stat::SVStat::Aggregate) and every cell is
//! placed through [`sv_scale`](crate::sv_scale).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_matrix::sv_hexbin_plot::SVHexbinPlot;
//!
//! let horizontal = [0.0, 0.1, 0.2, 1.0, 1.1, 2.0, 2.1, 2.2];
//! let vertical = [0.0, 0.1, 0.2, 1.0, 1.1, 2.0, 2.1, 2.2];
//! let data = SVDataTableBuilder::new()
//!     .with_column("x", SVColumnRole::X, &horizontal)
//!     .with_column("y", SVColumnRole::Y, &vertical)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let composition = SVHexbinPlot::builder()
//!     .data_source(&data)
//!     .x("x")
//!     .y("y")
//!     .grid_size(8)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the hexbin composes");
//!
//! assert!(composition.number_of_cells() > 0);
//! ```
//!
//! `render()` serializes the composition through the path the configured kind
//! actually has: an SVG document for `SVBackendKind::Svg`, a native frame for
//! `SVBackendKind::Native`, and a typed error for a kind the recipe does not
//! serialize yet.

mod builder;
mod builder_options;
mod builder_validation;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod options;
mod rendering;

pub use builder::SVHexbinPlotBuilder;
pub use composition::{SVHexbinCell, SVHexbinComposition};
pub use core_implementation::SVHexbinPlot;
pub use options::{SVHexbinAggregation, SVHexbinMissingValuePolicy};

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "hexbin_plot_tests.rs"]
mod hexbin_plot_tests;
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
