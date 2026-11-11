//! The colour bar over a continuous colour scale.

mod builder;
mod colour_bar_model;
mod measurement;
mod placement;

pub use builder::SVColourBarBuilder;
pub use colour_bar_model::{SVColourBar, SVColourBarOrientation, SVColourBarSpace};
pub use measurement::SVColourBarMeasurement;
pub use placement::{SVColourBarPlacement, SVColourBarTickPlacement};

#[cfg(test)]
#[path = "colour_bar_tests.rs"]
mod colour_bar_tests;
