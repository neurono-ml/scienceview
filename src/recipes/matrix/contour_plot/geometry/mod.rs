//! Marching squares and region stitching for one scalar field.
//!
//! This module is the contour extraction of the matrix family. The density
//! contour recipe carries the same geometry today, because the core stat
//! vocabulary has no contour variant (patterns P5). The extraction is kept
//! here as a self-contained geometry concern so a future core change can
//! promote it to a shared kernel without touching the recipe surface.
//!
//! `mod.rs` is a pure dispatcher: the level-set module owns the primitive
//! types and the marching-square case table, the marching module walks the
//! cells and the field boundary, and the regions module stitches directed
//! segments into closed loops.

mod level_set;
mod marching;
mod regions;

pub(crate) use level_set::{SVContourPoint, SVContourSegment};
pub(crate) use marching::{sv_boundary_segments, sv_marching_segments};
pub(crate) use regions::{sv_contour_epsilon, sv_stitch_regions};
