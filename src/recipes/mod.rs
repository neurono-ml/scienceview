//! Chart recipes: one module per chart family.
//!
//! A recipe composes the core kernels — data, statistics, scales, marks,
//! decorations — into one named chart. It adds no kernel variant of its own
//! (patterns P5), and it is registered in its family module rather than in a
//! central list (patterns P4), so recipe changes never serialise against each
//! other.
//! The public name is prefixed like every public module; the folder stays
//! `recipes/` because the `#[path]` attribute keeps the two apart.

#[path = "distribution/mod.rs"]
pub mod sv_distribution;

#[path = "relational/mod.rs"]
pub mod sv_relational;

#[path = "matrix/mod.rs"]
pub mod sv_matrix;
