//! Registration of the SVG backend in the process-wide registry.
//!
//! The core never names this crate (design D1 of the figure substrate). The
//! backend crate plugs itself in, and a program that enables the feature calls
//! [`sv_register_svg_backend`] once before rendering. Registration is
//! idempotent, so several call sites — and several tests — can request it.

use super::SVSvgBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderBackendProvider;
use scienceview_core::sv_backend::sv_backend_registry;
use std::sync::Arc;
use std::sync::OnceLock;

/// Supplies [`SVSvgBackend`] for the `Svg` kind and nothing else.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVSvgBackendProvider;

impl SVSvgBackendProvider {
    /// Creates the provider.
    pub const fn new() -> Self {
        Self
    }
}

impl SVRenderBackendProvider for SVSvgBackendProvider {
    fn provide(&self, backend_kind: SVBackendKind) -> Option<Arc<dyn SVRenderBackend>> {
        (backend_kind == SVBackendKind::Svg)
            .then(|| Arc::new(SVSvgBackend::new()) as Arc<dyn SVRenderBackend>)
    }
}

/// Registers the SVG backend in the process-wide registry.
///
/// The first call wins; later calls are no-ops, so registration never
/// accumulates duplicate providers.
pub fn sv_register_svg_backend() {
    static REGISTRATION: OnceLock<()> = OnceLock::new();

    REGISTRATION.get_or_init(|| {
        sv_backend_registry().register(Arc::new(SVSvgBackendProvider::new()));
    });
}
