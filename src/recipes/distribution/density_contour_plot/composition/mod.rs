//! One density-contour render: grids in, backend-agnostic paths out.
//!
//! `mod.rs` is a pure dispatcher. The definition module owns the public scene
//! and its serializers; the build module resolves the columns, density grids,
//! scales and decorations; the contour-geometry module turns the level sets
//! into path primitives.

mod build;
mod contour_geometry;
mod definition;

pub use definition::SVDensityContourComposition;

pub(crate) use build::sv_compose;
