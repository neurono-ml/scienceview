//! The closed move enumeration and its behaviour.

use super::kind::SVMoveKind;
use super::validation::{
    sv_validate_dodge_options, sv_validate_jitter_options, sv_validate_shift_options,
    sv_validate_swarm_options,
};
use crate::sv_error::SVError;
use crate::sv_move::stage::SVMoveStage;
use crate::sv_move::{
    SVDodgeOptions, SVJitterOptions, SVNormalizeOptions, SVShiftOptions, SVStackOptions,
    SVSwarmOptions,
};

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
