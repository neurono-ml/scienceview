//! Placing the prepared density estimates and serializing them.
//!
//! This module is a pure dispatcher (patterns.md P3): the scene type, the
//! composition of scales, axes and decorations, and the placement of every
//! curve each live in their own file.

mod composition;
mod geometry;
mod scene;

pub(crate) use composition::sv_compose_density;
pub use scene::{SVDensityCurve, SVDensityRendering};
