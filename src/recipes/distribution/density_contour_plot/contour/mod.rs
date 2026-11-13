//! Marching squares and region stitching for the density-contour recipe.
//!
//! `mod.rs` is a pure dispatcher: it declares the two concerns and re-exports
//! their items. The marching module turns a scalar field into directed level
//! segments; the regions module stitches those segments into closed loops with
//! the non-zero fill rule in mind.

mod level_set;
mod marching;
mod regions;

pub(crate) use level_set::{SVContourPoint, SVContourSegment};
pub(crate) use marching::{sv_boundary_segments, sv_marching_segments};
pub(crate) use regions::{sv_contour_epsilon, sv_stitch_regions};
