//! Configuration validators shared by the build path.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;

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

/// Rejects a line width that cannot resolve to a positive stroke.
pub(crate) fn sv_validate_line_width(line_width: Option<SVLength>) -> Result<(), SVError> {
    let valid = match line_width {
        None => true,
        Some(SVLength::Absolute { pixels }) => pixels.is_finite() && pixels > 0.0,
        Some(SVLength::Fraction { fraction }) => {
            fraction.is_finite() && (0.0..=1.0).contains(&fraction)
        }
        Some(SVLength::AspectRatio { ratio }) => ratio.is_finite() && ratio > 0.0,
        Some(SVLength::Intrinsic | SVLength::Fill) => false,
    };
    if valid {
        Ok(())
    } else {
        Err(sv_invalid(
            "the density line width must be a positive absolute or fractional length",
        ))
    }
}

pub(crate) fn sv_invalid(message: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: message.to_string(),
    }
}
