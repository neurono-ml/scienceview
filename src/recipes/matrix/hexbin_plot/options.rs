//! The recipe's public option vocabulary.

use crate::sv_error::SVError;

/// What each hexagon's colour encodes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVHexbinAggregation {
    /// The number of observations that fell into the hexagon (default).
    #[default]
    Count,
    /// The arithmetic mean of the optional `value` column per hexagon.
    Mean,
}

impl SVHexbinAggregation {
    /// Returns the aggregation name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVHexbinAggregation::Count => "Count",
            SVHexbinAggregation::Mean => "Mean",
        }
    }
}

/// What happens to a non-finite observation.
///
/// A non-finite coordinate has no place on the lattice. `Omit` drops it and
/// counts the remaining observations; `Error` rejects the render and names the
/// position, so a caller that expected clean data learns about the gap.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVHexbinMissingValuePolicy {
    /// Drop every non-finite observation (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVHexbinMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVHexbinMissingValuePolicy::Omit => "Omit",
            SVHexbinMissingValuePolicy::Error => "Error",
        }
    }

    pub(super) fn report(&self, position: usize) -> Result<(), SVError> {
        match self {
            SVHexbinMissingValuePolicy::Omit => Ok(()),
            SVHexbinMissingValuePolicy::Error => Err(SVError::NonFiniteValue {
                operation: "hexbin",
                position,
            }),
        }
    }
}
