//! Backend-independent layout: the length model and the constraints that
//! bound it.
//!
//! A figure is measured from its content and solved against a viewport, in a
//! coordinate system independent of the raster grid. This module owns the
//! public length model; the measure and solve passes build on it.

mod constraints;
mod length;

pub use constraints::SVLayoutConstraints;
pub use length::SVLength;

#[cfg(test)]
mod length_tests;
