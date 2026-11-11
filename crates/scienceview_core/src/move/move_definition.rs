//! The closed move enumeration and its kind.

use crate::sv_error::SVError;
use crate::sv_move::stage::SVMoveStage;
use crate::sv_move::{
    SVDodgeOptions, SVJitterOptions, SVNormalizeOptions, SVShiftOptions, SVStackOptions,
    SVSwarmOptions,
};

/// The kind of a move, independent of its options.
///
/// Observability and the stage table are keyed by this value, never by an
/// option struct, so a new variant fails both until it declares itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVMoveKind {
    /// The stack move.
    Stack,
    /// The dodge move.
    Dodge,
    /// The jitter move.
    Jitter,
    /// The shift move.
    Shift,
    /// The swarm move.
    Swarm,
    /// The normalize move.
    Normalize,
}

impl SVMoveKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stack,
        Self::Dodge,
        Self::Jitter,
        Self::Shift,
        Self::Swarm,
        Self::Normalize,
    ];

    /// Returns a stable lowercase name for spans and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stack => "stack",
            Self::Dodge => "dodge",
            Self::Jitter => "jitter",
            Self::Shift => "shift",
            Self::Swarm => "swarm",
            Self::Normalize => "normalize",
        }
    }
}

/// A positional move: the closed vocabulary of `FDR-003`.
#[derive(Clone, Debug, PartialEq)]
pub enum SVMove {
    /// Accumulates a group's values on a shared baseline.
    Stack(SVStackOptions),
    /// Places a group's values side by side within a category slot.
    Dodge(SVDodgeOptions),
    /// Displaces every value deterministically within its category.
    Jitter(SVJitterOptions),
    /// Shifts every value by a fixed data-space amount.
    Shift(SVShiftOptions),
    /// Lays out a category's values so that no two overlap.
    Swarm(SVSwarmOptions),
    /// Rescales every value to a fraction of its group total.
    Normalize(SVNormalizeOptions),
}

impl SVMove {
    /// Starts a stack, returning its option surface.
    pub fn stack() -> SVStackOptions {
        SVStackOptions::new()
    }

    /// Starts a dodge, returning its option surface.
    pub fn dodge() -> SVDodgeOptions {
        SVDodgeOptions::new()
    }

    /// Starts a jitter, returning its option surface.
    pub fn jitter() -> SVJitterOptions {
        SVJitterOptions::new()
    }

    /// Starts a shift, returning its option surface.
    pub fn shift() -> SVShiftOptions {
        SVShiftOptions::new()
    }

    /// Starts a swarm, returning its option surface.
    pub fn swarm() -> SVSwarmOptions {
        SVSwarmOptions::new()
    }

    /// Starts a normalize, returning its option surface.
    pub fn normalize() -> SVNormalizeOptions {
        SVNormalizeOptions::new()
    }

    /// Returns the kind of this move.
    pub fn kind(&self) -> SVMoveKind {
        match self {
            Self::Stack(_) => SVMoveKind::Stack,
            Self::Dodge(_) => SVMoveKind::Dodge,
            Self::Jitter(_) => SVMoveKind::Jitter,
            Self::Shift(_) => SVMoveKind::Shift,
            Self::Swarm(_) => SVMoveKind::Swarm,
            Self::Normalize(_) => SVMoveKind::Normalize,
        }
    }

    /// Returns the stage this move occupies in the fixed composition order.
    pub fn stage(&self) -> SVMoveStage {
        match self {
            Self::Stack(_) | Self::Dodge(_) | Self::Jitter(_) | Self::Swarm(_) => {
                SVMoveStage::Displacement
            }
            Self::Shift(_) => SVMoveStage::Shift,
            Self::Normalize(_) => SVMoveStage::Normalisation,
        }
    }

    /// Validates the options before a sequence is applied.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when an option is outside its
    /// documented range; the message names the option and the range.
    pub fn validate(&self) -> Result<(), SVError> {
        match self {
            Self::Stack(_) | Self::Normalize(_) => Ok(()),
            Self::Dodge(options) => sv_validate_dodge_options(options),
            Self::Jitter(options) => sv_validate_jitter_options(options),
            Self::Shift(options) => sv_validate_shift_options(options),
            Self::Swarm(options) => sv_validate_swarm_options(options),
        }
    }
}

/// Validates the dodge fractions once, when the sequence is assembled.
fn sv_validate_dodge_options(options: &SVDodgeOptions) -> Result<(), SVError> {
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
fn sv_validate_jitter_options(options: &SVJitterOptions) -> Result<(), SVError> {
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
fn sv_validate_shift_options(options: &SVShiftOptions) -> Result<(), SVError> {
    let amount = options.amount_value();
    if !amount.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: format!("shift amount must be finite, received {amount}"),
        });
    }
    Ok(())
}

/// Validates the swarm geometry once, when the sequence is assembled.
fn sv_validate_swarm_options(options: &SVSwarmOptions) -> Result<(), SVError> {
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
    }
    Ok(())
}

impl From<SVStackOptions> for SVMove {
    fn from(options: SVStackOptions) -> Self {
        Self::Stack(options)
    }
}

impl From<SVDodgeOptions> for SVMove {
    fn from(options: SVDodgeOptions) -> Self {
        Self::Dodge(options)
    }
}

impl From<SVJitterOptions> for SVMove {
    fn from(options: SVJitterOptions) -> Self {
        Self::Jitter(options)
    }
}

impl From<SVShiftOptions> for SVMove {
    fn from(options: SVShiftOptions) -> Self {
        Self::Shift(options)
    }
}

impl From<SVSwarmOptions> for SVMove {
    fn from(options: SVSwarmOptions) -> Self {
        Self::Swarm(options)
    }
}

impl From<SVNormalizeOptions> for SVMove {
    fn from(options: SVNormalizeOptions) -> Self {
        Self::Normalize(options)
    }
}
