//! The fit kernel, split into its single concerns.
//!
//! A fit pairs one independent column with one value column. Sparse storage is
//! rejected rather than densified: the implicit zeros of two independently
//! sparse columns would describe a regression through an origin point mass,
//! which is a different question from the one the user asked.

mod apply;
mod conditioning;
mod curve;
mod least_squares;

pub(crate) use apply::sv_apply_fit;
pub(crate) use curve::SV_FIT_CURVE_SAMPLE_COUNT;
