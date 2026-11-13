//! What happens to a missing or non-finite observation.

use crate::sv_error::SVError;

/// How the strip plot treats an observation that is not finite.
///
/// A non-finite value or category cannot be placed. `Omit` drops the
/// observation and draws the remaining sample; `Error` rejects the render and
/// names the position, so a caller that expected clean data learns about the
/// gap instead of silently drawing fewer points.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVStripMissingValuePolicy {
    /// Drop every non-finite observation (default).
    #[default]
    Omit,
    /// Reject the operation, naming the offending position.
    Error,
}

impl SVStripMissingValuePolicy {
    /// Returns the policy name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Omit => "Omit",
            Self::Error => "Error",
        }
    }

    /// Reports one non-finite observation according to this policy.
    ///
    /// # Errors
    ///
    /// [`Self::Error`] returns [`SVError::NonFiniteValue`] naming the position.
    pub(super) fn report(&self, position: usize) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::NonFiniteValue {
                operation: "strip plot",
                position,
            }),
        }
    }
}
