//! `SVHistogramPlot`: the distributional histogram recipe.
//!
//! The chart is the composition of FDR-111:
//! `data -> [SVStat::Bin] -> (edges, counts) -> [SVMove::Stack | Dodge] -> [SVScale x, y, colour] -> SVMark::Bar`.
//! It adds no kernel variant: binning goes to [`SVStat::Bin`](crate::sv_stat::SVStat::Bin),
//! stacking and dodging to [`sv_move`](crate::sv_move), placement to
//! [`sv_scale`](crate::sv_scale), and the geometry to the bar, line and area
//! marks of [`sv_mark`](crate::sv_mark).
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_histogram_plot::SVHistogramPlot;
//!
//! let values = [1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::X, &values)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVHistogramPlot::builder()
//!     .data_source(&data)
//!     .value("value")
//!     .bin_count(3)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the histogram composes");
//!
//! assert_eq!(scene.number_of_bins(), 3);
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
mod element;
mod geometry;
mod missing_value_policy;
mod multiple;
mod preparation;
mod render_output;
mod scene;

pub use builder::SVHistogramPlotBuilder;
pub use core_implementation::SVHistogramPlot;
pub use element::SVBarElement;
pub use missing_value_policy::SVHistogramMissingValuePolicy;
pub use multiple::SVHistogramMultiple;
pub use render_output::{SVHistogramFrame, SVHistogramRenderOutput};
pub use scene::{SVHistogramScene, SVHistogramSeriesSummary};

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "histogram_plot_tests.rs"]
mod histogram_plot_tests;
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
