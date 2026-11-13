//! `SVScatterPlot`: the relational scatter recipe.
//!
//! The chart is exactly the composition of FDR-101:
//! `data -> [no stat] -> values -> [SVMove::Jitter] -> [SVScale x, y, colour] -> SVMark::Dot`.
//! It adds no kernel variant; it resolves columns, maps the `series`, `colour`
//! and `size` channels onto the dot kernel's geometry and returns a
//! backend-agnostic [`SVScatterScene`].
//!
//! ```
//! use scienceview::recipes::relational::scatter_plot::SVScatterPlot;
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::SVDataTableBuilder;
//! use scienceview::sv_data::SVColumnRole;
//!
//! let horizontal = [0.0, 1.0, 2.0];
//! let vertical = [1.0, 4.0, 2.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("x", SVColumnRole::X, &horizontal)
//!     .with_column("y", SVColumnRole::Y, &vertical)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVScatterPlot::builder()
//!     .data_source(&data)
//!     .x("x")
//!     .y("y")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .render()
//!     .expect("the scatter renders");
//!
//! assert_eq!(scene.number_of_marks(), 3);
//! ```

mod builder;
mod channels;
mod core_implementation;
mod decorations;
mod geometry;
mod options;
mod scene;

pub use options::{SVScatterLegendPlacement, SVScatterSizeScale};
pub use scene::SVScatterScene;

pub use builder::SVScatterPlotBuilder;
pub use core_implementation::SVScatterPlot;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "observability_tests.rs"]
mod observability_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(all(test, any(feature = "render-svg", feature = "render-native")))]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "scatter_plot_tests.rs"]
mod scatter_plot_tests;
