//! `scienceview`: scientific visualization for Rust, in the same family as
//! `sciencekit`.
//!
//! Figures are built through builders, styled like the most popular Python
//! plotting libraries, interactive, and able to absorb streaming data updates.
//! Every figure selects its rendering backend explicitly (`SVBackendKind`):
//! native OS embedded components, HTML with JavaScript, WebAssembly, or
//! terminal TUI.
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
//! - `render-svg` (default): self-contained SVG output, no extra dependencies.
//! - `render-html`: self-contained HTML page embedding the SVG, no JavaScript.
//! - `render-tui`: terminal rendering via `ratatui`.
//! - `render-native`: from-scratch interactive renderer — CPU pixels via
//!   `softbuffer` in a native `winit` window.
//! - `streaming`: live updates over standard-library channels.
//! - `streaming-tokio`: live updates over Tokio watch channels.
//! - `wasm`: WebAssembly canvas target for the from-scratch renderer.
