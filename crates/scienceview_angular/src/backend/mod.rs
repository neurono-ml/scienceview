//! The backend implementation and its registry provider.

mod angular_backend;
mod provider;

pub use angular_backend::SVAngularBackend;
pub use provider::SVAngularBackendProvider;
pub use provider::sv_register_angular_backend;
