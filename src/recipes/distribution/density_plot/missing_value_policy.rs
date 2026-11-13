//! What a density estimate does with an observation that is not finite.

use scienceview_core::sv_error::SVError;

/// How the estimate treats an observation or a series code that is not finite.
///
/// A missing value is expressed as a non-finite `f64`. The policy makes the
/// choice explicit instead of leaving it to whichever kernel reads the column
/// first: the density kernel itself rejects a non-finite observation, so the
/// recipe decides whether to drop it or to reject the whole render.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVDensityMissingValuePolicy {
    /// Drop the observation before the kernel sees it (default).
    ///
    /// The estimate is taken over the finite observations only, which is the
    /// behaviour of the reference libraries: a missing value is not a zero
    /// observation and must not pull the curve towards the origin.
    #[default]
    Omit,
    /// Reject the whole render with an explicit configuration error.
    Error,
}

impl SVDensityMissingValuePolicy {
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
    /// [`SVDensityMissingValuePolicy::Error`] returns
    /// [`SVError::InvalidConfiguration`] naming the column and the position.
    pub(crate) fn report(self, column: &str, index: usize) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the density column `{column}` has a non-finite value at position {index}; the missing-value policy is Error"
                ),
            }),
        }
    }
}
