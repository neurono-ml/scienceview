//! The recipe families, built by composing the core kernels.
//!
//! The public name is prefixed like every public module; the folder stays
//! `recipes/` because the `#[path]` attribute keeps the two apart.

#[path = "relational/mod.rs"]
pub mod sv_relational;
