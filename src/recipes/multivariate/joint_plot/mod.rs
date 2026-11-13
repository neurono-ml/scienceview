//! `SVJointPlot`: the multivariate joint recipe.
//!
//! The chart is the composition of FDR-145:
//! `data -> [no stat] -> values -> central cell (scatter, hexbin, density
//! contour or regression) -> marginal cells (histogram, density or rug) ->
//! [shared scales per axis] -> three panels sharing a layout`. It adds no
//! kernel variant: the central and marginal forms compose the existing kernels
//! and recipes, and the layout kernel solves the three panels.
//!
//! ```
//! use scienceview::sv_backend::SVBackendKind;
//! use scienceview::sv_data::SVDataTableBuilder;
//! use scienceview::sv_data::SVColumnRole;
//! use scienceview::sv_recipes::sv_multivariate::sv_joint_plot::SVJointPlot;
//!
//! let horizontal = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
//! let vertical = [1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
//! let data = SVDataTableBuilder::new()
//!     .with_column("x", SVColumnRole::X, &horizontal)
//!     .with_column("y", SVColumnRole::Y, &vertical)
//!     .build()
//!     .expect("the table is well formed");
//!
//! let scene = SVJointPlot::builder()
//!     .data_source(&data)
//!     .x("x")
//!     .y("y")
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete")
//!     .compose()
//!     .expect("the joint plot composes");
//!
//! assert!(scene.number_of_marks() > 0);
//! ```
//!
//! The scene carries the central panel, the top and right marginals, and the
//! shared scales. `render()` serializes it as an SVG document for
//! `SVBackendKind::Svg`, rasterizes it for `SVBackendKind::Native`, and returns
//! a typed error for a kind the recipe does not serialize yet.

mod builder;
mod builder_options;
mod channels;
mod composition;
mod configuration;
mod core_implementation;
mod missing_value_policy;
mod options;
mod render_output;
mod scene;

pub use builder::SVJointPlotBuilder;
pub use core_implementation::SVJointPlot;
pub use missing_value_policy::SVJointMissingValuePolicy;
pub use options::{SVJointKind, SVJointMarginal, SVJointRegressionModel};
pub use render_output::{SVJointFrame, SVJointRenderOutput};
pub use scene::SVJointScene;

#[cfg(test)]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;
#[cfg(test)]
#[path = "joint_plot_tests.rs"]
mod joint_plot_tests;
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
#[path = "test_support.rs"]
mod test_support;
