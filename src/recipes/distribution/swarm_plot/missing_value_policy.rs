//! What a swarm does with an observation that is not finite.

use crate::sv_error::SVError;

/// How the swarm treats an observation or a code that is not finite.
///
/// A missing value is expressed as a non-finite `f64`. The policy makes the
/// choice explicit instead of leaving it to whichever kernel reads the column
/// first: the recipe decides whether to drop the observation or to reject the
/// whole render, so the move never sees a non-finite value it cannot sort.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVSwarmMissingValuePolicy {
    /// Drop the observation before the move sees it (default).
    #[default]
    Omit,
    /// Reject the whole render with an explicit configuration error.
    Error,
}

impl SVSwarmMissingValuePolicy {
    /// The policy name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Omit => "Omit",
            Self::Error => "Error",
        }
    }

    /// Reports a missing observation according to this policy.
    ///
    /// # Errors
    ///
    /// [`SVSwarmMissingValuePolicy::Error`] returns
    /// [`SVError::InvalidConfiguration`] naming the column and the position.
    pub(super) fn report(self, column: &str, position: usize, role: &str) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the swarm `{role}` column `{column}` has a non-finite value at position {position}; the missing-value policy is Error"
                ),
            }),
        }
    }
}
