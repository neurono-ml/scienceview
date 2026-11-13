//! One hexbin render: cells in, backend-agnostic paths out.
//!
//! `mod.rs` is a pure dispatcher. The definition module owns the public scene
//! and its serializers; the build module resolves the columns, deposits the
//! observations in the lattice and paints the cells; the lattice module owns
//! the hexagonal coordinate system; the geometry module turns cells into paths.

mod build;
mod definition;
mod geometry;
mod lattice;

pub use definition::{SVHexbinCell, SVHexbinComposition};

pub(crate) use build::sv_compose;

#[cfg(test)]
#[path = "lattice_tests.rs"]
mod lattice_tests;
