//! Building the sampled field a contour plot draws.
//!
//! `mod.rs` is a pure dispatcher. The resolve module folds the configured
//! layout into a field, the matrix module reads a row-major matrix column onto
//! a lattice with configured axis ranges, the points module bins an `x`, `y`,
//! `value` cloud through the bin kernel and averages every cell through the
//! aggregate kernel, and the missing module applies the configured policy to a
//! non-finite sample.

mod matrix;
mod missing;
mod points;
mod resolve;

pub(crate) use resolve::sv_prepare_field;
