//! `SVRugPlot`: the distributional rug recipe.
//!
//! The chart is exactly the composition of FDR-115:
//! `data -> [no stat] -> values -> [no move] -> [SVScale] -> SVMark::Line
//! (segment geometry)`. It adds no kernel variant: it resolves the value and
//! series columns once, places each stored observation through a continuous
//! scale and composes one short line segment per observation in the margin of
//! the axis it belongs to.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_rug_plot::SVRugPlot;
//!
//! let values = [1.0, 2.0, 2.0, 3.5, 4.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::X, &values)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVRugPlot::builder()
//!     .data_source(&data)
//!     .value("value")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the rug composes");
//!
//! assert_eq!(scene.number_of_marks(), 5);
//! ```
//!
//! `render()` goes one step further and serializes the scene through the path
//! the configured kind actually has: an SVG document for `SVBackendKind::Svg`,
//! a native frame for `SVBackendKind::Native`, and a typed error for a kind the
//! recipe does not serialize yet.

mod build;
mod builder;
mod channels;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod geometry;
mod operations;
mod render_output;
mod scene;

pub use builder::SVRugPlotBuilder;
pub use core_implementation::SVRugPlot;
pub use render_output::{SVRugFrame, SVRugRenderOutput};
pub use scene::SVRugScene;

#[cfg(test)]
#[path = "large_rug_tests.rs"]
mod large_rug_tests;
#[cfg(test)]
#[path = "option_tests.rs"]
mod option_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "rug_plot_tests.rs"]
mod rug_plot_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
