//! What an area does with a sample that has no finite position or value.

use scienceview_core::sv_error::SVError;

/// How the area treats a sample whose `x` or `y` is not finite.
///
/// A missing value is expressed as a non-finite `f64`. The policy makes the
/// choice explicit instead of leaving it to whichever kernel reads the column
/// first.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVMissingValuePolicy {
    /// Drop the sample and split the area at the gap (default).
    ///
    /// Consecutive finite samples stay one region; a missing sample closes the
    /// region before it and opens a new one after it, so the area never bridges
    /// over absent data. When the chart is stacked the missing value
    /// contributes nothing to the stack, and the series keeps its position in
    /// the cumulative baseline.
    #[default]
    Omit,
    /// Reject the whole render with an explicit configuration error.
    Error,
}

impl SVMissingValuePolicy {
    /// The policy name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Omit => "Omit",
            Self::Error => "Error",
        }
    }

    /// Reports a missing sample according to this policy.
    ///
    /// # Errors
    ///
    /// [`SVMissingValuePolicy::Error`] returns [`SVError::InvalidConfiguration`]
    /// naming the column and the sample position.
    pub(crate) fn report(self, column: &str, index: usize) -> Result<(), SVError> {
        match self {
            Self::Omit => Ok(()),
            Self::Error => Err(SVError::InvalidConfiguration {
                message: format!(
                    "the area column `{column}` has a non-finite value at position {index}; the missing-value policy is Error"
                ),
            }),
        }
    }
}
