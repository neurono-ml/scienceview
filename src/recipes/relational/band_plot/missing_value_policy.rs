//! What a band does with a sample that has no finite boundary.

use scienceview_core::sv_error::SVError;

/// How the band treats a sample whose `x`, `lower` or `upper` is not finite.
///
/// A missing value is expressed as a non-finite `f64` (`NaN`, or an infinity
/// that the scale could not place). The policy makes the choice explicit
/// instead of leaving it to whichever kernel reads the column first.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVBandMissingValuePolicy {
    /// Drop the sample and split the band at the gap (default).
    ///
    /// Consecutive finite samples remain one polygon; a missing sample closes
    /// the polygon before it and opens a new one after it, so the band never
    /// bridges over absent data.
    #[default]
    Omit,
    /// Reject the whole render with an explicit configuration error.
    Error,
}

impl SVBandMissingValuePolicy {
    /// Reports a missing sample according to this policy.
    ///
    /// # Errors
    ///
    /// [`SVBandMissingValuePolicy::Error`] returns [`SVError::InvalidConfiguration`]
    /// naming the column and the sample position.
    pub(crate) fn report(self, column: &str, index: usize) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the band column `{column}` has a non-finite value at position {index}; the missing-value policy is Error"
                ),
            }),
        }
    }
}
