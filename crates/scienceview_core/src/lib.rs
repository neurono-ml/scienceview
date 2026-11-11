//! `scienceview_core`: the dependency-light core of
//! [`scienceview`](https://github.com/neurono-ml/scienceview).
//!
//! The core owns the figure model, the data boundary, the kernels and the
//! layout. Rendering is never implemented here: every rendering form is a
//! separate crate commanded by the `SVRenderBackend` trait, so the default
//! build stays free of GUI and WebAssembly dependencies.
//!
//! ## Crate map
//!
//! The workspace has one core crate plus one crate per rendering backend, all
//! under `crates/`, and the root `scienceview` package is a facade that
//! re-exports the core and the backends whose features are enabled.
//!
//! | Crate | Responsibility |
//! | --- | --- |
//! | `scienceview_core` | Figure model, data boundary, kernels and layout; depends on `thiserror` and `tracing` only. |
//! | `scienceview_svg` | Self-contained SVG output. |
//! | `scienceview_html` | Self-contained HTML page embedding the figure. |
//! | `scienceview_tui` | Terminal rendering. |
//! | `scienceview_native` | From-scratch CPU renderer in a native window. |
//! | `scienceview_wasm` | WebAssembly canvas target. |
//! | `scienceview_react` | React component output. |
//! | `scienceview_vue` | Vue.js component output. |
//! | `scienceview_angular` | Angular component output. |
//! | `scienceview_tauri` | Tauri native window output. |
//! | `scienceview_dioxus` | Dioxus native component output. |
//!
//! ## Feature map
//!
//! Every backend feature maps 1:1 to an optional path dependency on its crate.
//! Enabling a feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend. `default = []`, so a default build compiles the facade and the core
//! alone.
//!
//! | Feature | Enables | Dependency |
//! | --- | --- | --- |
//! | `render-svg` | `scienceview_svg` | optional |
//! | `render-html` | `scienceview_html` | optional |
//! | `render-tui` | `scienceview_tui` | optional |
//! | `render-native` | `scienceview_native` | optional |
//! | `render-react` | `scienceview_react` | optional |
//! | `render-vue` | `scienceview_vue` | optional |
//! | `render-angular` | `scienceview_angular` | optional |
//! | `render-tauri` | `scienceview_tauri` | optional |
//! | `render-dioxus` | `scienceview_dioxus` | optional |
//! | `wasm` | `scienceview_wasm` | optional |
//! | `data-ndarray` | Zero-copy `ndarray` views | optional |
//! | `spec-serde` | JSON figure specification | optional |
//! | `streaming` | Standard-library channel updates | none |
//! | `streaming-tokio` | Tokio watch channel updates | optional |
//! | `default` | Nothing | none |
//!
//! `scienceview_core` itself is always compiled: the facade re-exports it as a
//! required dependency.

// The directories follow the change grid (`backend/`, `error/`,
// `figure/`, `observability/`) while the public module names follow the naming
// rule of PRD §3.8: public modules carry the `sv_` prefix.
#[path = "backend/mod.rs"]
pub mod sv_backend;
#[path = "error/mod.rs"]
pub mod sv_error;
#[path = "figure/mod.rs"]
pub mod sv_figure;
#[path = "observability/mod.rs"]
pub mod sv_observability;

/// Returns the name of this crate.
///
/// Pipeline code and the workspace smoke tests use it to prove that the core
/// crate is reachable through the root facade.
pub fn sv_crate_name() -> &'static str {
    "scienceview_core"
}

#[cfg(test)]
mod crate_map_tests;
