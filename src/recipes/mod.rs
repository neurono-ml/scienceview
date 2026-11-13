//! Chart recipes: one module per chart family.
//!
//! A recipe composes the core kernels — data, statistics, scales, marks,
//! decorations — into one named chart. It adds no kernel variant of its own
//! (patterns P5), and it is registered in its family module rather than in a
//! central list (patterns P4), so recipe changes never serialise against each
//! other.
//!
//! The public path is `scienceview::sv_recipes::sv_relational::…`; the
//! physical folders keep the short names and the `#[path]` attributes map the
//! `sv_`-prefixed public names onto them.

#[path = "relational/mod.rs"]
pub mod sv_relational;
