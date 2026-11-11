//! Backend-independent layout: the length model and the constraints that
//! bound it.
//!
//! A figure is measured from its content and solved against a viewport, in a
//! coordinate system independent of the raster grid. This module owns the
//! public length model; the measure and solve passes build on it.

mod constraints;
mod engine;
mod length;
pub(crate) mod measurement;
mod node;
mod resolved;
mod solver;
mod spacing;
mod viewport;

pub use constraints::SVLayoutConstraints;
pub use engine::SVLayoutTree;
pub use length::SVLength;
pub use node::{SVLayoutDirection, SVLayoutNode};
pub use resolved::{SVResolvedLayout, SVResolvedNode, SVResolvedRectangle};
pub use spacing::SVLayoutSpacing;
pub use viewport::SVViewport;

#[cfg(test)]
mod distribution_tests;
#[cfg(test)]
mod length_tests;
#[cfg(test)]
mod solver_tests;
