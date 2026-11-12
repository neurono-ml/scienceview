//! The empirical-CDF options.

/// The configuration of
/// [`SVStat::EmpiricalCdf`](super::SVStat::EmpiricalCdf).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVEmpiricalCdfOptions {
    normalise: bool,
}

impl Default for SVEmpiricalCdfOptions {
    fn default() -> Self {
        Self { normalise: true }
    }
}

impl SVEmpiricalCdfOptions {
    /// Starts a cumulative distribution normalised to `[0, 1]`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Chooses between probabilities and cumulative counts.
    #[must_use]
    pub fn normalise(mut self, normalise: bool) -> Self {
        self.normalise = normalise;
        self
    }

    /// Returns `true` when the curve is normalised to probabilities.
    pub fn is_normalised(&self) -> bool {
        self.normalise
    }
}
