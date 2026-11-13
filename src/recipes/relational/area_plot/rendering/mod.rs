//! Placing the prepared areas and serializing them through a backend.
//!
//! This module is a pure dispatcher (patterns.md P3): the scene type, the
//! composition of scales, axes and decorations, and the placement of every
//! segment live in their own files.

mod composition;
mod geometry;
mod scene;

pub(crate) use composition::sv_compose_area;
pub use scene::SVAreaRendering;
