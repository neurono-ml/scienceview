//! Backend availability at the Python boundary.
//!
//! Registering a provider only makes a backend *available*; the kind is still
//! selected explicitly by every figure (spec `backend-selection`). The Python
//! module calls this once at import time, and the pure-layer tests call it
//! before rendering.

/// Registers every backend compiled into this build.
///
/// The call is idempotent: each backend guards its own registration. A build
/// without a backend feature registers nothing and rendering through that kind
/// reports `SVError::BackendUnavailable`, never an automatic fallback.
pub fn sv_register_available_backends() {
    #[cfg(feature = "render-svg")]
    scienceview::scienceview_svg::sv_register_svg_backend();
}
