//! Registration of the HTML backend in the process-wide registry.
//!
//! The core never names this crate; the backend plugs itself in, and a program
//! that enables the matching feature calls [`sv_register_html_backend`] once
//! before rendering. Registration is idempotent.

use super::SVHtmlBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderBackendProvider;
use scienceview_core::sv_backend::sv_backend_registry;
use std::sync::Arc;
use std::sync::OnceLock;

/// Supplies [`SVHtmlBackend`] for the `Html` kind and nothing else.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVHtmlBackendProvider;

impl SVHtmlBackendProvider {
    /// Creates the provider.
    pub const fn new() -> Self {
        Self
    }
}

impl SVRenderBackendProvider for SVHtmlBackendProvider {
    fn provide(&self, backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>> {
        (backend_kind == SVBackendKind::Html)
            .then(|| Arc::new(SVHtmlBackend::new()) as Arc<dyn SVRenderBackend>)
    }
}

/// Registers the HTML backend in the process-wide registry.
///
/// The first call wins; later calls are no-ops, so registration never
/// accumulates duplicate providers.
pub fn sv_register_html_backend() {
    static REGISTRATION: OnceLock<()> = OnceLock::new();

    REGISTRATION.get_or_init(|| {
        sv_backend_registry().register(Arc::new(SVHtmlBackendProvider::new()));
    });
}
