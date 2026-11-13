//! What a violin does with an observation that is not finite.

use scienceview_core::sv_error::SVError;

/// How the recipe treats a non-finite value, category or series code.
///
/// The density kernel rejects a non-finite observation itself, so the recipe
/// decides before the kernel sees one whether to drop it or reject the whole
/// render.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVViolinMissingValuePolicy {
    /// Drop the observation before the kernel sees it (default).
    #[default]
    Omit,
    /// Reject the whole render with an explicit configuration error.
    Error,
}

impl SVViolinMissingValuePolicy {
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
    /// [`SVViolinMissingValuePolicy::Error`] returns
    /// [`SVError::InvalidConfiguration`] naming the column and the position.
    pub(crate) fn report(self, column: &str, index: usize) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the violin column `{column}` has a non-finite value at position {index}; the missing-value policy is Error"
                ),
            }),
        }
    }
}
