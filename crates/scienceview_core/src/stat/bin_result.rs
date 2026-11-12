//! The bins a binning kernel returns.

use crate::sv_stat::SVNormalisation;

/// The derived view of [`SVStat::Bin`](super::SVStat::Bin).
///
/// `edges` holds one more value than `counts`; every interval is closed on the
/// left and the last one is closed on the right.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBinResult {
    edges: Vec<f64>,
    counts: Vec<f64>,
    normalisation: SVNormalisation,
    cumulative: bool,
    number_of_observed_values: usize,
}

impl SVBinResult {
    /// Assembles a binning result.
    pub(crate) fn new(
        edges: Vec<f64>,
        counts: Vec<f64>,
        normalisation: SVNormalisation,
        cumulative: bool,
        number_of_observed_values: usize,
    ) -> Self {
        Self {
            edges,
            counts,
            normalisation,
            cumulative,
            number_of_observed_values,
        }
    }

    /// Returns the bin edges, one more than the number of bins.
    pub fn edges(&self) -> &[f64] {
        &self.edges
    }

    /// Returns the normalised count of every bin.
    pub fn counts(&self) -> &[f64] {
        &self.counts
    }

    /// Returns the normalisation that was applied.
    pub fn normalisation(&self) -> SVNormalisation {
        self.normalisation
    }

    /// Returns `true` when the counts are cumulative.
    pub fn is_cumulative(&self) -> bool {
        self.cumulative
    }

    /// Returns the number of observations the column held, inside and outside
    /// the edges.
    pub fn number_of_observed_values(&self) -> usize {
        self.number_of_observed_values
    }

    /// Returns the width of the bin at `index`.
    pub fn bin_width(&self, index: usize) -> Option<f64> {
        let left = self.edges.get(index)?;
        let right = self.edges.get(index + 1)?;
        Some(right - left)
    }
}
