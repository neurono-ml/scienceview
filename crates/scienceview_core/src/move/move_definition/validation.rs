//! Option validation, run once when a sequence is assembled.

use crate::sv_error::SVError;
use crate::sv_move::{SVDodgeOptions, SVJitterOptions, SVShiftOptions, SVSwarmOptions};

/// Validates the dodge fractions once, when the sequence is assembled.
pub(super) fn sv_validate_dodge_options(options: &SVDodgeOptions) -> Result<(), SVError> {
    let width = options.width_value();
    let gap = options.gap_value();
    if !width.is_finite() || width <= 0.0 || width > 1.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!("dodge width must be within (0.0, 1.0], received {width}"),
        });
    }
    if !gap.is_finite() || !(0.0..1.0).contains(&gap) {
        return Err(SVError::InvalidConfiguration {
            message: format!("dodge gap must be within [0.0, 1.0), received {gap}"),
        });
    }
    Ok(())
}

/// Validates the jitter amount once, when the sequence is assembled.
pub(super) fn sv_validate_jitter_options(options: &SVJitterOptions) -> Result<(), SVError> {
    let amount = options.amount_value();
    if !amount.is_finite() || amount < 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "jitter amount must be a finite fraction of the category width and at least zero, received {amount}"
            ),
        });
    }
    Ok(())
}

/// Validates the shift amount once, when the sequence is assembled.
pub(super) fn sv_validate_shift_options(options: &SVShiftOptions) -> Result<(), SVError> {
    let amount = options.amount_value();
    if !amount.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: format!("shift amount must be finite, received {amount}"),
        });
    }
    Ok(())
}

/// Validates the swarm geometry once, when the sequence is assembled.
pub(super) fn sv_validate_swarm_options(options: &SVSwarmOptions) -> Result<(), SVError> {
    let marker_extent = options.marker_extent_value();
    if !marker_extent.is_finite() || marker_extent <= 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "swarm marker_extent must be a finite length greater than zero, received {marker_extent}"
            ),
        });
    }
    if let Some(width) = options.width_value() {
        if !width.is_finite() || width <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "swarm width must be a finite length greater than zero, received {width}"
                ),
            });
        }
        if f64::from(width) < 2.0 * marker_extent {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "swarm width must be at least two marker extents so that a colliding point has a candidate on each side of zero; received width {width} for the marker extent {marker_extent}"
                ),
            });
        }
    }
    Ok(())
}
