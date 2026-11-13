//! Composing one joint render.
//!
//! `mod.rs` is a pure dispatcher (patterns.md P3). The data boundary, the
//! panel solver, the central panel, the marginals, the geometry translation and
//! the decorations each live in their own file.

mod build;
pub(super) mod central;
mod data;
mod decorations;
mod layout;
mod marginals;
mod translate;

pub(crate) use build::sv_compose_scene;
