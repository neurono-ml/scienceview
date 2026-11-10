//! `scienceview`: scientific visualization for Rust, in the same family as
//! [`sciencekit`](https://github.com/neurono-ml/sciencekit).
//!
//! Figures are built through builders, styled like the most popular Python
//! plotting libraries, interactive, and able to absorb streaming data updates.
//! Every figure selects its rendering backend explicitly through `backend(...)`
//! with an `SVBackendKind` value: there is no automatic backend, the rendering
//! form is always a deliberate choice.
//!
//! ## Trait-driven, feature-gated backends
//!
//! Rendering is commanded by the `SVRenderBackend` trait. Every backend is a
//! Cargo feature and maps 1:1 to a workspace crate under `crates/scienceview_*`,
//! so new backends — including third-party crates — plug in without touching
//! the core. Enabling a feature only makes the backend available; the builder
//! still requires an explicit `backend(...)` selection.
//!
//! Supported backends:
//!
//! - `render-svg`: self-contained SVG output, no extra dependencies.
//! - `render-html`: self-contained HTML page embedding the figure.
//! - `render-tui`: terminal rendering via `ratatui`.
//! - `render-native`: from-scratch interactive renderer — CPU pixels via
//!   `softbuffer` in a native `winit` window.
//! - `render-react`: React component output.
//! - `render-vue`: Vue.js component output.
//! - `render-angular`: Angular component output.
//! - `render-tauri`: Tauri native window output.
//! - `render-dioxus`: Dioxus native component output.
//! - `wasm`: WebAssembly canvas target for the from-scratch renderer.
//!
//! ## Dependency-light core
//!
//! The core data model is borrowed slices — no heavy algebra or GUI toolkit
//! is required to describe a figure. Every capability beyond the core is an
//! opt-in Cargo feature, so unused backends cost no binary size and no
//! compile time:
//!
//! - `data-ndarray`: zero-copy views over `ndarray` arrays.
//! - `spec-serde`: JSON serialization of the figure specification.
//! - `streaming`: live updates over standard-library channels.
//! - `streaming-tokio`: live updates over Tokio watch channels.
