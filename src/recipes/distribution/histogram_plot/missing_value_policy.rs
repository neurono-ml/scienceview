//! What happens to a non-finite observation.

use crate::sv_error::SVError;

/// The policy a histogram applies to a missing or non-finite value.
///
/// A non-finite observation cannot be placed in a bin. `Omit` drops it and
/// counts the remaining observations; `Error` rejects the render and names the
/// position, so a caller that expected clean data learns about the gap instead
/// of silently drawing a smaller histogram.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVHistogramMissingValuePolicy {
    /// Drop every non-finite observation (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVHistogramMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVHistogramMissingValuePolicy::Omit => "Omit",
            SVHistogramMissingValuePolicy::Error => "Error",
        }
    }

    /// Reports one non-finite observation according to this policy.
    pub(super) fn report(&self, operation: &'static str, position: usize) -> Result<(), SVError> {
        match self {
            SVHistogramMissingValuePolicy::Omit => Ok(()),
            SVHistogramMissingValuePolicy::Error => Err(SVError::NonFiniteValue {
                operation,
                position,
            }),
        }
    }

    /// Returns `true` when a non-finite observation is dropped.
    pub(super) fn omits(&self) -> bool {
        *self == SVHistogramMissingValuePolicy::Omit
    }
}
