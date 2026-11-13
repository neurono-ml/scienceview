//! What happens to a non-finite category or series value.

use crate::sv_error::SVError;

/// The policy a count plot applies to a missing or non-finite category.
///
/// A category is the value that identifies one bar, so it cannot be counted
/// when it is not finite. `Omit` drops the observation and counts the
/// remaining ones; `Error` rejects the operation and names the position, so a
/// caller who expected clean categories learns about the gap instead of seeing
/// a smaller total.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVCountMissingValuePolicy {
    /// Drop every non-finite observation (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVCountMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVCountMissingValuePolicy::Omit => "Omit",
            SVCountMissingValuePolicy::Error => "Error",
        }
    }

    /// Reports one non-finite observation according to this policy.
    pub(super) fn report(&self, operation: &'static str, position: usize) -> Result<(), SVError> {
        match self {
            SVCountMissingValuePolicy::Omit => Ok(()),
            SVCountMissingValuePolicy::Error => Err(SVError::NonFiniteValue {
                operation,
                position,
            }),
        }
    }

    /// Returns `true` when a non-finite observation is dropped.
    pub(super) fn omits(&self) -> bool {
        *self == SVCountMissingValuePolicy::Omit
    }
}
