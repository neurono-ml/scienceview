//! One density-heatmap render: grids in, backend-agnostic cells out.
//!
//! `mod.rs` is a pure dispatcher. The definition module owns the public scene
//! and its accessors, the serialization module owns the backend writers, and
//! the build module resolves the columns, grids, scales and decorations.

mod build;
mod definition;
mod serialization;

pub use definition::SVDensityHeatmapComposition;

pub(crate) use build::sv_compose;
