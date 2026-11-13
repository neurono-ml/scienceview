//! What the joint plot does with a non-finite observation.

use crate::sv_error::SVError;

/// How the joint plot treats an observation that is not finite.
///
/// A missing value is expressed as a non-finite `f64`. The central panel and
/// the two marginals share one policy, so a reader never sees the same
/// observation dropped from one panel and kept in another.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVJointMissingValuePolicy {
    /// Drop the observation from every panel (default).
    #[default]
    Omit,
    /// Reject the whole render, naming the offending column and position.
    Error,
}

impl SVJointMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Omit => "Omit",
            Self::Error => "Error",
        }
    }

    /// Reports one missing observation according to this policy.
    ///
    /// # Errors
    ///
    /// [`SVJointMissingValuePolicy::Error`] returns
    /// [`SVError::InvalidConfiguration`] naming the column and the position.
    pub(crate) fn report(self, column: &str, index: usize) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the joint-plot column `{column}` has a non-finite value at position {index}; the missing-value policy is Error"
                ),
            }),
        }
    }
}
