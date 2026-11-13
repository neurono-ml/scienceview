//! `SVSwarmPlot`: the collision-free distributional recipe.
//!
//! The chart is exactly the composition of FDR-120:
//! `data -> [no stat] -> values -> [SVMove::Swarm] -> [SVScale] -> SVMark::Dot`.
//! It adds no kernel variant: it resolves the value, category and optional
//! series columns, lets the swarm move lay each category out without overlap,
//! maps the produced offsets onto the categorical axis and returns a
//! backend-agnostic [`SVSwarmScene`].
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::{SVColumnRole, SVDataTableBuilder};
//! use scienceview::sv_recipes::sv_distribution::sv_swarm_plot::SVSwarmPlot;
//!
//! let values = [1.0, 2.0, 3.0, 2.0];
//! let categories = [0.0, 0.0, 1.0, 1.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("value", SVColumnRole::Y, &values)
//!     .with_column("category", SVColumnRole::Grouping, &categories)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVSwarmPlot::builder()
//!     .data_source(&data)
//!     .value("value")
//!     .category("category")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the swarm composes");
//!
//! assert_eq!(scene.number_of_marks(), 4);
//! ```
//!
//! `render()` goes one step further and serializes the scene through the path
//! the configured kind actually has: an SVG document for
//! `SVBackendKind::Svg`, a native frame for `SVBackendKind::Native`, and a
//! typed error for a kind the recipe does not serialize yet.

mod build;
mod builder;
mod channels;
mod composition;
mod configuration;
mod core_implementation;
mod decorations;
mod groups;
mod marks;
mod missing_value_policy;
mod render_output;
mod scales;
mod scene;
mod swarm_layout;

pub use builder::SVSwarmPlotBuilder;
pub use core_implementation::SVSwarmPlot;
pub use missing_value_policy::SVSwarmMissingValuePolicy;
pub use render_output::{SVSwarmFrame, SVSwarmRenderOutput};
pub use scene::SVSwarmScene;

#[cfg(test)]
#[path = "error_tests.rs"]
mod error_tests;
#[cfg(test)]
#[path = "large_swarm_tests.rs"]
mod large_swarm_tests;
#[cfg(test)]
#[path = "option_tests.rs"]
mod option_tests;
#[cfg(test)]
#[path = "orientation_tests.rs"]
mod orientation_tests;
#[cfg(test)]
#[path = "parity_tests.rs"]
mod parity_tests;
#[cfg(test)]
#[path = "render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "streaming_tests.rs"]
mod streaming_tests;
#[cfg(test)]
#[path = "swarm_plot_tests.rs"]
mod swarm_plot_tests;
#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
