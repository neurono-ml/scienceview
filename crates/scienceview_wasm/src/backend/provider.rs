//! Registration of the WebAssembly backend in the process-wide registry.
//!
//! The core never names this crate: the backend plugs itself in, and a program
//! that enables the feature calls [`sv_register_wasm_backend`] once before
//! rendering. Registration is idempotent.

use super::wasm_backend::SVWasmBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderBackendProvider;
use scienceview_core::sv_backend::sv_backend_registry;
use std::sync::Arc;
use std::sync::OnceLock;

/// Supplies [`SVWasmBackend`] for the `Wasm` kind and nothing else.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVWasmBackendProvider;

impl SVWasmBackendProvider {
    /// Creates the provider.
    pub const fn new() -> Self {
        Self
    }
}

impl SVRenderBackendProvider for SVWasmBackendProvider {
    fn provide(&self, backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>> {
        (backend_kind == SVBackendKind::Wasm)
            .then(|| Arc::new(SVWasmBackend::new()) as Arc<dyn SVRenderBackend>)
    }
}

/// Registers the WebAssembly backend in the process-wide registry.
///
/// The first call wins; later calls are no-ops.
pub fn sv_register_wasm_backend() {
    static REGISTRATION: OnceLock<()> = OnceLock::new();

    REGISTRATION.get_or_init(|| {
        sv_backend_registry().register(Arc::new(SVWasmBackendProvider::new()));
    });
}
