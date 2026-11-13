//! The backend implementation, its registry provider, and their companion tests.

mod provider;
mod wasm_backend;

pub use provider::SVWasmBackendProvider;
pub use provider::sv_register_wasm_backend;
pub use wasm_backend::SVWasmBackend;

#[cfg(test)]
#[path = "wasm_backend_tests.rs"]
mod wasm_backend_tests;
