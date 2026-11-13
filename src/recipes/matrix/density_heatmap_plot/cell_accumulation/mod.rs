//! Deriving the two edge vectors and folding the observations into cells.
//!
//! `mod.rs` is a pure dispatcher. The edges module derives the bin boundaries
//! and the cell lookup through the bin kernel; the fold module walks the
//! observation pairs, aggregates them through the aggregate kernel and
//! normalises the returned cells.

mod edges;
mod fold;
mod normalisation;

pub(crate) use fold::sv_build_grids;
