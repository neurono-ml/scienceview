//! Configuration validators shared by the build path.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_scale::SVScaleKind;

/// Rejects a backend the recipe has no scene serializer for.
///
/// The recipe composes one backend-agnostic scene and hands it to a concrete
/// serializer: the SVG document and the native raster frame exist today; every
/// other kind reports the missing export capability instead of receiving bytes
/// it did not produce.
pub(crate) fn sv_validate_recipe_backend(backend_kind: SVBackendKind) -> Result<(), SVError> {
    if matches!(backend_kind, SVBackendKind::Svg | SVBackendKind::Native) {
        Ok(())
    } else {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        })
    }
}

pub(crate) fn sv_validate_scale_kind(x_scale_kind: SVScaleKind) -> Result<(), SVError> {
    if matches!(
        x_scale_kind,
        SVScaleKind::Continuous | SVScaleKind::Temporal
    ) {
        Ok(())
    } else {
        Err(sv_invalid(
            "the ordered axis of an area plot is continuous or temporal",
        ))
    }
}

pub(crate) fn sv_invalid(message: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: message.to_string(),
    }
}
