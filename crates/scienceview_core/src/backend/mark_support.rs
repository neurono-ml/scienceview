//! Which mark variants each backend kind can render.
//!
//! The mark vocabulary is closed and a backend renders every variant it
//! declares here. A backend that cannot express a variant says so once, in
//! this matrix, and a figure that composes the variant fails with
//! [`SVError::UnsupportedMark`](crate::sv_error::SVError::UnsupportedMark)
//! while it is configured — never by silently drawing nothing.

use super::kind::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkKind;

/// Returns whether `backend_kind` can render `mark_kind`.
///
/// Every backend renders the eight vector marks. The raster image is the one
/// exception: a terminal cell grid has no surface for a borrowed bitmap, so
/// `Tui` rejects `Image` and every backend that can embed a raster accepts it.
pub const fn sv_mark_supported(mark_kind: SVMarkKind, backend_kind: SVBackendKind) -> bool {
    match mark_kind {
        SVMarkKind::Image => !matches!(backend_kind, SVBackendKind::Tui),
        SVMarkKind::Dot
        | SVMarkKind::Line
        | SVMarkKind::Area
        | SVMarkKind::Bar
        | SVMarkKind::Ribbon
        | SVMarkKind::Text
        | SVMarkKind::Wedge
        | SVMarkKind::ErrorBar => true,
    }
}

/// Validates that `backend_kind` can render `mark_kind`.
///
/// # Errors
///
/// Returns [`SVError::UnsupportedMark`] when the backend's matrix row does not
/// include the mark, naming both the backend and the variant.
pub fn sv_verify_mark_supported(
    mark_kind: SVMarkKind,
    backend_kind: SVBackendKind,
) -> Result<(), SVError> {
    if sv_mark_supported(mark_kind, backend_kind) {
        return Ok(());
    }

    Err(SVError::UnsupportedMark {
        backend_kind,
        mark_kind,
    })
}
