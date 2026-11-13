//! The backend implementation and its registry provider.

mod provider;
mod tui_backend;

pub use provider::SVTuiBackendProvider;
pub use provider::sv_register_tui_backend;
pub use tui_backend::SVTuiBackend;
pub use tui_backend::sv_tui_default_viewport;
pub use tui_backend::sv_tui_viewport;

#[cfg(test)]
mod backend_tests;
