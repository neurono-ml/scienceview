//! The percentile options.

/// The configuration of [`SVStat::Percentile`](super::SVStat::Percentile).
///
/// The defaults are the three quartiles, the quantiles a box plot draws.
#[derive(Clone, Debug, PartialEq)]
pub struct SVPercentileOptions {
    quantiles: Vec<f64>,
}

impl Default for SVPercentileOptions {
    fn default() -> Self {
        Self {
            quantiles: vec![0.25, 0.5, 0.75],
        }
    }
}

impl SVPercentileOptions {
    /// Starts with the three quartiles.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the requested quantiles; every one must lie in `[0, 1]`.
    #[must_use]
    pub fn quantiles(mut self, quantiles: Vec<f64>) -> Self {
        self.quantiles = quantiles;
        self
    }

    /// Returns the requested quantiles.
    pub fn requested_quantiles(&self) -> &[f64] {
        &self.quantiles
    }
}
