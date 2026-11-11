//! `scienceview_core`: the dependency-light core of
//! [`scienceview`](https://github.com/neurono-ml/scienceview).
//!
//! The core owns the figure model, the data boundary, the kernels and the
//! layout. Rendering is never implemented here: every rendering form is a
//! separate crate commanded by the `SVRenderBackend` trait, so the default
//! build stays free of GUI and WebAssembly dependencies.

/// Returns the name of this crate.
///
/// Pipeline code and the workspace smoke tests use it to prove that the core
/// crate is reachable through the root facade.
pub fn sv_crate_name() -> &'static str {
    "scienceview_core"
}
