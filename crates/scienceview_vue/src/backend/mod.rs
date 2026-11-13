//! The backend implementation and its registry provider.

mod provider;
mod vue_backend;

pub use provider::SVVueBackendProvider;
pub use provider::sv_register_vue_backend;
pub use vue_backend::SVVueBackend;
