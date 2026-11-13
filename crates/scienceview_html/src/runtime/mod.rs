//! The embedded runtime, its events and its inlined source.

mod runtime_source;

pub use runtime_source::SV_HTML_ACTION_EVENT;
pub use runtime_source::SV_HTML_DELTA_EVENT;
pub use runtime_source::SV_HTML_RUNTIME;
pub use runtime_source::SV_HTML_UPDATE_EVENT;
pub use runtime_source::sv_html_runtime_byte_length;

#[cfg(test)]
#[path = "embedded_runtime_tests.rs"]
mod embedded_runtime_tests;
