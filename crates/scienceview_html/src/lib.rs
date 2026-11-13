//! `scienceview_html`: self-contained HTML rendering backend for scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
//!
//! ## What the backend guarantees
//!
//! - **Self-contained**: the figure, the compiled theme stylesheet and a small
//!   runtime are inlined. Nothing is fetched at display time and no dependency
//!   beyond the workspace is added.
//! - **Deterministic**: the element order is fixed, the runtime is a constant
//!   and the payload uses one JSON representation, so rendering the same
//!   figure twice yields byte-identical output.
//! - **Updates in place**: `update` returns only the payload element. Replacing
//!   exactly that element changes the data without reloading the page and
//!   without losing the host page's view state.
//! - **The model stays the authority**: the payload carries the SVG markup the
//!   Rust model rendered, the current representation and the model clock. The
//!   runtime applies payloads and moves a view transform; it never re-derives
//!   geometry and never owns figure state.
//!
//! ## Capability row
//!
//! The `Html` row exports, streams, delivers gesture actions and updates in
//! place. It does not host a script engine and does not present a window, so
//! `show` reports the missing presentation capability instead of pretending to
//! succeed.
//!
//! ## Rendering a figure
//!
//! ```ignore
//! sv_register_html_backend();
//! let document = figure.render()?.bytes;
//! ```
//!
//! A recipe that composes its own document can use the emitter directly:
//!
//! ```ignore
//! let payload = SVHtmlDataPayload::new(view, svg_markup);
//! let html = SVHtmlDocument::new(payload).with_title("Weights").to_html();
//! ```

mod backend;
mod document;
mod payload;
mod runtime;

pub use backend::{SVHtmlBackend, SVHtmlBackendProvider, sv_register_html_backend};
pub use document::{
    SV_HTML_CAPTION_ELEMENT_ID, SV_HTML_DATA_ELEMENT_ID, SVHtmlDocument, sv_html_stylesheet,
    sv_stylesheet_for_tokens,
};
pub use payload::{SVHtmlDataPayload, sv_payload_number};
pub use runtime::{
    SV_HTML_ACTION_EVENT, SV_HTML_DELTA_EVENT, SV_HTML_RUNTIME, SV_HTML_UPDATE_EVENT,
    sv_html_runtime_byte_length,
};

#[cfg(test)]
mod crate_tests;
