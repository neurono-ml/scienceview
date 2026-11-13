//! The binned two-dimensional density field.
//!
//! `mod.rs` is a pure dispatcher. The definition module owns the public
//! `SVDensityGrid`; the builder derives the edges through the bin kernel and
//! folds observations; the accumulation module normalises and smooths the cell
//! values; the series module resolves the optional grouping and weight columns.

mod accumulation;
mod builder;
mod definition;
mod series;

pub use definition::SVDensityGrid;

pub(crate) use builder::sv_build_density_grids;
