//! The backend implementation and its registry provider.

mod provider;
mod react_backend;

pub use provider::SVReactBackendProvider;
pub use provider::sv_register_react_backend;
pub use react_backend::SVReactBackend;
