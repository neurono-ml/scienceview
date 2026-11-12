//! Normalize options.

/// What a normalised group totals to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVNormalisationTarget {
    /// A fraction, so a group totals to one (default).
    #[default]
    Fraction,
    /// A percentage, so a group totals to one hundred.
    Percent,
}

/// The option surface of the normalize move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVNormalizeOptions {
    target: SVNormalisationTarget,
}

impl Default for SVNormalizeOptions {
    fn default() -> Self {
        Self {
            target: SVNormalisationTarget::Fraction,
        }
    }
}

impl SVNormalizeOptions {
    /// Builds the default normalize.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets what a normalised group totals to.
    pub fn target(mut self, target: SVNormalisationTarget) -> Self {
        self.target = target;
        self
    }

    /// Returns what a normalised group totals to.
    pub fn target_value(&self) -> SVNormalisationTarget {
        self.target
    }
}
