//! Chart recipes.
//!
//! A recipe composes the closed kernels of `scienceview_core` into one chart.
//! Recipes live in the root package because they are the public chart surface:
//! the core stays a kernel library, and every family owns a module that
//! registers its charts (`docs/FDRs/patterns.md`, P4).
//!
//! The public module is `sv_recipes`; the folder keeps the dated `recipes`
//! name through `#[path]`, so the physical layout stays stable while the
//! public surface follows the mandatory `sv_` prefix.

#[path = "relational/mod.rs"]
pub mod sv_relational;
