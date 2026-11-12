//! The backend implementation and its registry provider.

mod provider;
mod svg_backend;

pub use provider::SVSvgBackendProvider;
pub use provider::sv_register_svg_backend;
pub use svg_backend::SVSvgBackend;

#[cfg(test)]
mod backend_tests;
