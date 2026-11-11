//! `scienceview_core`: the dependency-light core of
//! [`scienceview`](https://github.com/neurono-ml/scienceview).
//!
//! The core owns the figure model, the data boundary, the kernels and the
//! layout. Rendering is never implemented here: every rendering form is a
//! separate crate commanded by the `SVRenderBackend` trait, so the default
//! build stays free of GUI and WebAssembly dependencies.
//!
//! ## The figure substrate
//!
//! Every figure and artist is built through a builder that requires an explicit
//! backend and carries an execution mode:
//!
//! - [`sv_figure`] owns `SVFigure`, the typestate `SVFigureBuilder`, the data
//!   binding it carries and `SVExecutionMode` (defaulting to `Automatic`).
//! - [`sv_backend`] owns the closed `SVBackendKind` (no automatic variant), the
//!   capability matrix, the `SVRenderBackend` contract and the registry through
//!   which backend crates plug in.
//! - [`sv_error`] owns `SVError`, the single public error enumeration.
//! - [`sv_observability`] owns the one operation span every public operation
//!   runs inside, carrying the configured backend kind, the representation, the
//!   number of marks and the elapsed time.
//!
//! ## The scale kernel
//!
//! [`sv_scale`] is the only place where a data value becomes a visual property.
//! `SVScale` is a closed model of four kinds — continuous, nominal, temporal and
//! boolean — and each scale owns its data domain and display range together, so
//! an axis cannot disagree with the marks on it. Domains are explicit or derived
//! from a [`SVColumnView`](sv_data::SVColumnView) without densifying sparse data
//! or blocking on a stream; transforms (linear, logarithmic, symmetric
//! logarithmic, square root) are invertible; nominal colours are stable and
//! continuous colours resolve theme colour maps; and the tick derivation hook
//! turns the space the layout engine grants an axis into readable ticks.
//!
//! The backend kind is a runtime value, never a type parameter, and `build`
//! performs no dispatch: each operation resolves the configured kind to a
//! concrete backend at call time.
//!
//! ```
//! use scienceview_core::sv_backend::SVBackendKind;
//! use scienceview_core::sv_data::SVColumnView;
//! use scienceview_core::sv_figure::SVFigureBuilder;
//!
//! let values = [1.0, 2.0, 3.0];
//! let figure = SVFigureBuilder::new()
//!     .data_source(SVColumnView::Dense(&values))
//!     .backend(SVBackendKind::Svg)
//!     .build()
//!     .expect("the configuration is complete");
//!
//! assert_eq!(figure.backend_kind(), SVBackendKind::Svg);
//! ```
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
//! | `data-sprs` | Zero-copy `sprs` sparse views | optional |
//! | `spec-serde` | JSON figure specification | optional |
//! | `streaming` | Standard-library channel updates | none |
//! | `streaming-tokio` | Tokio watch channel updates | optional |
//! | `theme` | Theme resolution for scale palettes and colour maps | optional |
//! | `default` | Nothing | none |
//!
//! `scienceview_core` itself is always compiled: the facade re-exports it as a
//! required dependency.
//!
//! ## Data boundary
//!
//! [`sv_data`] holds the ingestion trait (`SVDataSource`) and the closed
//! representation kernels receive (`SVColumnView`: dense, sparse or
//! streaming). Sparse storage is never densified, and [`sv_window`] provides the
//! bounded rolling window that lets a live source be read without blocking the
//! render thread. Representation parity is asserted with the harness in
//! [`sv_data`], whose tolerance policy lives beside it; the scale kernel is the
//! first real consumer of that harness.
//!
//! ## Layout
//!
//! [`sv_layout`] resolves figure geometry independently of the raster grid. A
//! backend supplies a [`sv_layout::SVViewport`]; the layout tree measures the
//! content, solves the viewport against the declared lengths and constraints,
//! and hands back a solved tree every backend rasterizes on its own grid. The
//! `theme` feature lets the default spacing come from compiled theme tokens;
//! without it the layout uses documented built-in defaults.
//!
//! ## Scale
//!
//! [`sv_scale`] maps data values onto visual channels: continuous, nominal,
//! temporal and boolean scales own their domain and range together, transform
//! through linear, logarithmic, symmetric-logarithmic or square-root functions,
//! derive axis ticks from the space the layout engine grants, and resolve their
//! colours from the same compiled theme tokens.
//!
//! ## Marks
//!
//! [`sv_mark`] is the closed vocabulary every recipe draws with: dot, line,
//! area, bar, ribbon, text, image, wedge and error bar. A mark owns geometry and
//! paint, never data. It consumes the closed [`sv_data`] representation, places
//! its values through [`sv_scale`] inside the rectangle [`sv_layout`] solved,
//! and returns backend-agnostic primitives a backend serializes without
//! re-deriving a position. The interpolable-property table the transition layer
//! consumes is exported from the same module, next to the properties it
//! describes.
//!
//! ## Decorations
//!
//! [`sv_decoration`] is what makes an encoding legible: axes with ticks that
//! fit the granted space, grid lines, the legend that follows the series
//! lifecycle, the colour bar tied to its continuous colour scale, and the
//! titles. Every decoration measures itself before the plot area is solved and
//! places itself into the solved rectangle afterwards, emitting
//! backend-agnostic primitives; text is laid out as a box until a backend
//! rasterizes glyphs.

#[path = "data/mod.rs"]
pub mod sv_data;
#[path = "decoration/mod.rs"]
pub mod sv_decoration;
#[path = "layout/mod.rs"]
pub mod sv_layout;
#[path = "mark/mod.rs"]
pub mod sv_mark;
#[path = "scale/mod.rs"]
pub mod sv_scale;
#[path = "window/mod.rs"]
pub mod sv_window;

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
