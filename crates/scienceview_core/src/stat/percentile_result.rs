//! The quantiles a percentile kernel returns.

/// The derived view of [`SVStat::Percentile`](super::SVStat::Percentile).
#[derive(Clone, Debug, PartialEq)]
pub struct SVPercentileResult {
    quantiles: Vec<f64>,
    values: Vec<f64>,
    windowed: bool,
}

impl SVPercentileResult {
    /// Assembles a percentile result.
    pub(crate) fn new(quantiles: Vec<f64>, values: Vec<f64>, windowed: bool) -> Self {
        Self {
            quantiles,
            values,
            windowed,
        }
    }

    /// Returns the requested quantiles, in request order.
    pub fn quantiles(&self) -> &[f64] {
        &self.quantiles
    }

    /// Returns one value per requested quantile.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Returns `true` when the result describes only the current window.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }
}
