//! Backend selection at the Python boundary.
//!
//! The selection is explicit and required, exactly as in Rust: there is no
//! automatic kind, no default value and no fallback. The Python surface accepts
//! the stable lowercase name of `SVBackendKind`.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_error::SVError;

/// Parses the backend name the Python surface accepts.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] naming every accepted backend
/// kind when the name is unknown.
pub fn sv_parse_backend_kind(name: &str) -> Result<SVBackendKind, SVError> {
    SVBackendKind::ALL
        .into_iter()
        .find(|kind| kind.as_str() == name)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: format!(
                "`{name}` is not a backend kind; the accepted names are {}",
                sv_backend_kind_names().join(", ")
            ),
        })
}

/// The stable names of every backend kind, in declaration order.
pub fn sv_backend_kind_names() -> Vec<&'static str> {
    SVBackendKind::ALL
        .iter()
        .map(|kind| kind.as_str())
        .collect()
}

/// Resolves the selection carried by a specification.
///
/// # Errors
///
/// Returns [`SVError::MissingBackendSelection`] when the caller passed no
/// backend, which is the same error the Rust builder produces: the rendering
/// form is always a deliberate choice.
pub fn sv_require_backend_kind(
    backend_kind: Option<SVBackendKind>,
) -> Result<SVBackendKind, SVError> {
    backend_kind.ok_or(SVError::MissingBackendSelection)
}
