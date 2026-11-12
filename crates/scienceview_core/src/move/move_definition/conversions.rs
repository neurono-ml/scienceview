//! Lossless conversions from each option surface into its move.

use super::move_enum::SVMove;
use crate::sv_move::{
    SVDodgeOptions, SVJitterOptions, SVNormalizeOptions, SVShiftOptions, SVStackOptions,
    SVSwarmOptions,
};

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
