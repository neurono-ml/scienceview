//! The per-operation dispatch seam (spec `backend-selection`).
//!
//! `build` never calls this module: it only records the kind. Every operation
//! calls [`sv_resolve_render_backend`] at the moment it runs, so a figure can
//! even change its backend between calls.

use super::kind::SVBackendKind;
use super::registry::SVBackendRegistry;
use super::render_backend::SVRenderBackend;
use crate::sv_error::SVError;
use std::sync::Arc;

/// Resolves the concrete backend for one operation.
///
/// The returned implementation is checked against the requested kind: a
/// provider that answers with a different backend is a configuration bug, and
/// silently rendering with the wrong backend would be worse than failing.
pub fn sv_resolve_render_backend(
    registry: &SVBackendRegistry,
    backend_kind: SVBackendKind,
) -> Result<Arc<dyn SVRenderBackend>, SVError> {
    let backend = registry.resolve(backend_kind)?;

    if backend.kind() != backend_kind {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the provider registered for `{backend_kind}` answered with `{}`",
                backend.kind()
            ),
        });
    }

    Ok(backend)
}
