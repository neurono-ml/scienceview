//! The process-wide registry that lets backend crates plug into the core
//! without the core depending on them.

use super::kind::SVBackendKind;
use super::render_backend::SVRenderBackend;
use super::render_backend::SVRenderBackendProvider;
use crate::sv_error::SVError;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::RwLock;

/// A set of backend providers, asked in registration order.
///
/// Tests build their own registry so the process-wide one stays untouched; the
/// figure builder defaults to [`sv_backend_registry`].
#[derive(Default)]
pub struct SVBackendRegistry {
    providers: RwLock<Vec<Arc<dyn SVRenderBackendProvider>>>,
}

impl SVBackendRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a provider. Later providers are consulted after earlier ones.
    pub fn register(&self, provider: Arc<dyn SVRenderBackendProvider>) {
        let mut providers = self
            .providers
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        providers.push(provider);
    }

    /// Resolves `backend_kind` to the first implementation a provider offers.
    ///
    /// A kind nothing serves is `SVError::BackendUnavailable`, never a panic
    /// and never an automatic fallback.
    pub fn resolve(
        &self,
        backend_kind: SVBackendKind,
    ) -> Result<Arc<dyn SVRenderBackend>, SVError> {
        let providers = {
            let providers = self
                .providers
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            providers.clone()
        };

        providers
            .iter()
            .find_map(|provider| provider.provide(backend_kind))
            .ok_or(SVError::BackendUnavailable { backend_kind })
    }
}

/// The process-wide registry backend crates register into.
///
/// Returned as a shared handle so a figure can hold it without borrowing a
/// static and a test can substitute its own registry.
pub fn sv_backend_registry() -> Arc<SVBackendRegistry> {
    static REGISTRY: OnceLock<Arc<SVBackendRegistry>> = OnceLock::new();

    Arc::clone(REGISTRY.get_or_init(|| Arc::new(SVBackendRegistry::new())))
}
