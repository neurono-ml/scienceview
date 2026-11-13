//! `scienceview_vue`: Vue component rendering backend for scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
//!
//! ## What the backend produces
//!
//! The backend renders a **self-contained JSX component source string**: the
//! inline SVG markup of the figure, the figure data, a theme object and the
//! action event name are embedded in the source. No npm dependency is used at
//! build time — the crate only writes text, and the consuming application
//! compiles that text with its own Vue toolchain.
//!
//! The component exposes the shared framework contract:
//!
//! - properties `data`, `markup`, `theme` and `title`;
//! - the `scienceview-action` event, carrying `select` and `zoom` actions.
//!
//! It swaps the inline markup for a newer `markup` property, so a host applies
//! the output of [`SVRenderBackend::update`] without reloading the component.

mod backend;
mod component;

pub use backend::{SVVueBackend, SVVueBackendProvider, sv_register_vue_backend};
pub use component::{SV_VUE_COMPONENT_CONTRACT, SVVueComponentContract, sv_render_vue_component};
pub use scienceview_core::sv_backend::SVRenderBackend;

#[cfg(test)]
mod crate_tests;
