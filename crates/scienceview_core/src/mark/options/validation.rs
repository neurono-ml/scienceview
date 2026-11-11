//! Shared validation of the option surface, so every mark reports the same
//! error for the same mistake.

use crate::sv_error::SVError;
use crate::sv_layout::SVLength;

/// Rejects a length a mark cannot resolve.
///
/// A mark has no layout parent of its own, so only an absolute length and a
/// fraction of the plot area are meaningful; the intrinsic, aspect-ratio and
/// fill declarations belong to the layout tree and are reported instead of
/// silently resolving to zero.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the length is not absolute or
/// fractional, is not finite, is negative, or is zero when `allow_zero` is
/// false.
pub(crate) fn sv_validate_length(
    length: SVLength,
    name: &str,
    allow_zero: bool,
) -> Result<(), SVError> {
    match length {
        SVLength::Absolute { pixels } if pixels.is_finite() => {
            sv_validate_bound(pixels, name, allow_zero)
        }
        SVLength::Fraction { fraction } if fraction.is_finite() => {
            sv_validate_bound(fraction, name, allow_zero)
        }
        SVLength::Absolute { .. } | SVLength::Fraction { .. } => {
            Err(SVError::InvalidConfiguration {
                message: format!("{name} must be a finite length, received {length:?}"),
            })
        }
        SVLength::Intrinsic | SVLength::AspectRatio { .. } | SVLength::Fill => {
            Err(SVError::InvalidConfiguration {
                message: format!(
                    "{name} must be absolute or fractional; a mark has no parent to resolve {length:?}"
                ),
            })
        }
    }
}

/// Validates an opacity override.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the opacity is not finite or
/// falls outside `0.0..=1.0`.
pub(crate) fn sv_validate_opacity(opacity: Option<f32>, name: &str) -> Result<(), SVError> {
    match opacity {
        Some(value) if value.is_finite() && (0.0..=1.0).contains(&value) => Ok(()),
        Some(value) => Err(SVError::InvalidConfiguration {
            message: format!("{name} must fall inside 0.0..=1.0, received {value}"),
        }),
        None => Ok(()),
    }
}

/// Validates a finite numeric option.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the value is not finite.
pub(crate) fn sv_validate_finite(value: f64, name: &str) -> Result<(), SVError> {
    if value.is_finite() {
        return Ok(());
    }

    Err(SVError::InvalidConfiguration {
        message: format!("{name} must be finite, received {value}"),
    })
}

/// Validates a fraction option that must fall inside `0.0..=1.0`.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the value is not finite or
/// falls outside the interval.
pub(crate) fn sv_validate_fraction(value: f64, name: &str) -> Result<(), SVError> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        return Ok(());
    }

    Err(SVError::InvalidConfiguration {
        message: format!("{name} must fall inside 0.0..=1.0, received {value}"),
    })
}

fn sv_validate_bound(value: f64, name: &str, allow_zero: bool) -> Result<(), SVError> {
    let acceptable = if allow_zero {
        value >= 0.0
    } else {
        value > 0.0
    };
    if acceptable {
        return Ok(());
    }

    let expectation = if allow_zero {
        "zero or above"
    } else {
        "above zero"
    };
    Err(SVError::InvalidConfiguration {
        message: format!("{name} must be {expectation}, received {value}"),
    })
}
