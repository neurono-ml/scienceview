//! What happens to a missing or non-finite observation.

use scienceview_core::sv_error::SVError;

/// The policy a bar plot applies to a missing or non-finite value.
///
/// A non-finite observation cannot be aggregated. `Omit` drops it and counts
/// the remaining observations; `Error` rejects the render and names the
/// position, so a caller that expected clean data learns about the gap instead
/// of silently drawing a shorter bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVBarMissingValuePolicy {
    /// Drop every non-finite observation (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVBarMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVBarMissingValuePolicy::Omit => "Omit",
            SVBarMissingValuePolicy::Error => "Error",
        }
    }

    /// Reports one non-finite observation according to this policy.
    pub(crate) fn report(&self, position: usize) -> Result<(), SVError> {
        match self {
            SVBarMissingValuePolicy::Omit => Ok(()),
            SVBarMissingValuePolicy::Error => Err(SVError::NonFiniteValue {
                operation: "bar plot",
                position,
            }),
        }
    }
}
