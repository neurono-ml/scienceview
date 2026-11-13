//! The embedded runtime source, its events and its size.
//!
//! The runtime never computes geometry. It reads the payload the Rust model
//! serialized, swaps the rendered markup, and turns gestures into actions whose
//! deltas only move the view transform. Requesting an animation frame is the
//! clock the browser offers; the playback position always comes from the
//! payload (design D4).

/// The event a host dispatches to deliver a new payload to a figure.
pub const SV_HTML_UPDATE_EVENT: &str = "scienceview:update";
/// The event the runtime dispatches when a gesture becomes an action.
pub const SV_HTML_ACTION_EVENT: &str = "scienceview:action";
/// The event the runtime dispatches after it applies a view delta.
pub const SV_HTML_DELTA_EVENT: &str = "scienceview:delta";

/// The inlined runtime source.
pub const SV_HTML_RUNTIME: &str = include_str!("runtime.js");

/// Why the runtime is small enough to inline in every document.
///
/// The constant function exists so the size is asserted in one place instead
/// of in the middle of an unrelated test.
pub fn sv_html_runtime_byte_length() -> usize {
    SV_HTML_RUNTIME.len()
}
