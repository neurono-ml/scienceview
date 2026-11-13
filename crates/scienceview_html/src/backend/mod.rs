//! The backend implementation and its registry provider.

mod html_backend;
mod provider;

pub use html_backend::SVHtmlBackend;
pub use provider::SVHtmlBackendProvider;
pub use provider::sv_register_html_backend;

#[cfg(test)]
#[path = "backend_tests.rs"]
mod backend_tests;
