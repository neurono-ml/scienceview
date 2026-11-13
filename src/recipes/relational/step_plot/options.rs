//! The step-specific option vocabulary.

use core::fmt;

/// Where the vertical part of a step is drawn between two samples.
///
/// The value holds the previous sample until the riser and the next sample
/// afterwards, so no interpolation is implied:
///
/// - [`SVStepPosition::Before`] raises the riser at the current `x` (a
///   *pre* step);
/// - [`SVStepPosition::After`] keeps the current value until the next `x` and
///   raises the riser there (a *post* step, the default);
/// - [`SVStepPosition::Center`] raises the riser halfway between the two `x`
///   values (a *mid* step).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVStepPosition {
    /// The riser sits at the current `x`.
    Before,
    /// The riser sits at the next `x` (default).
    #[default]
    After,
    /// The riser sits halfway between the two `x` values.
    Center,
}

impl SVStepPosition {
    /// Every position, in declaration order.
    pub const ALL: [Self; 3] = [Self::Before, Self::After, Self::Center];

    /// Returns the position name as written in documentation and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVStepPosition::Before => "Before",
            SVStepPosition::After => "After",
            SVStepPosition::Center => "Center",
        }
    }
}

impl fmt::Display for SVStepPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Which observation wins when two rows share the same `x` and `series`.
///
/// A step is a function of `x`, so duplicate positions are collapsed before
/// the geometry is expanded. Ordering is stable, which makes the policy
/// deterministic for equal `x` values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVStepDuplicatePolicy {
    /// Keep the last observation in input order (default), matching a source
    /// that overwrites a sample at the same position.
    #[default]
    LastWins,
    /// Keep the first observation in input order.
    FirstWins,
}

impl SVStepDuplicatePolicy {
    /// Returns the policy name as written in documentation and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVStepDuplicatePolicy::LastWins => "LastWins",
            SVStepDuplicatePolicy::FirstWins => "FirstWins",
        }
    }
}

impl fmt::Display for SVStepDuplicatePolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
