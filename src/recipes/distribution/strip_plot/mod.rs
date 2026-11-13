//! `SVStripPlot`: the distributional strip recipe.
//!
//! The chart is exactly the composition of FDR-119:
//! `data -> [no stat] -> values -> [SVMove::Jitter] -> [SVScale] -> SVMark::Dot`.
//! It adds no kernel variant: it resolves the value, category and series
//! columns once, keys a jitter through [`sv_move`](crate::sv_move), maps the
//! displaced category slots and the observed values onto the scales and draws
//! one dot per observation.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_strip_plot::SVStripPlot;
//!
//! let values = [1.0, 2.0, 2.5, 3.5, 4.0];
//! let categories = [0.0, 0.0, 1.0, 1.0, 1.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::Y, &values)
//!     .with_column("category", SVColumnRole::Grouping, &categories)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVStripPlot::builder()
//!     .data_source(&data)
//!     .value("value")
//!     .category("category")
//!     .jitter(0.15)
//!     .seed(7)
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the strip plot composes");
//!
//! assert_eq!(scene.number_of_marks(), 5);
//! ```
//!
//! `render()` goes one step further and serializes the scene through the path
//! the configured kind actually has: an SVG document for
//! `SVBackendKind::Svg`, a native frame for `SVBackendKind::Native`, and a
//! typed error for a kind the recipe does not serialize yet.

mod build;
mod builder;
mod builder_options;
mod composition;
mod configuration;
mod core_implementation;
mod data_contract;
mod decorations;
mod geometry;
mod jitter;
mod missing_value_policy;
mod operations;
mod render_output;
mod scales;
mod scene;

pub use builder::SVStripPlotBuilder;
pub use core_implementation::SVStripPlot;
pub use missing_value_policy::SVStripMissingValuePolicy;
pub use render_output::{SVStripFrame, SVStripRenderOutput};
pub use scene::SVStripScene;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "large_strip_tests.rs"]
mod large_strip_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "option_tests.rs"]
mod option_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "strip_plot_tests.rs"]
mod strip_plot_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
