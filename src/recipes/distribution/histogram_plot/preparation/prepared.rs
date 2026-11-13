//! The owned, data-space result of one histogram preparation.

use crate::sv_scale::SVColour;

/// One series of binned values, ready to place.
pub(crate) struct SVPreparedSeries {
    pub(crate) label: String,
    pub(crate) colour: SVColour,
    pub(crate) edges: Vec<f64>,
    pub(crate) tops: Vec<f64>,
    pub(crate) baselines: Vec<f64>,
    pub(crate) width_fractions: Vec<f64>,
    pub(crate) offsets: Vec<f64>,
}

impl SVPreparedSeries {
    /// Returns the number of bins in this series.
    pub(crate) fn number_of_bins(&self) -> usize {
        self.tops.len()
    }
}

/// The prepared histogram: the series and the arrangement applied to them.
pub(crate) struct SVPreparedHistogram {
    pub(crate) series: Vec<SVPreparedSeries>,
}

impl SVPreparedHistogram {
    /// Returns the number of drawn bars: one per bin of every series.
    pub(crate) fn number_of_bars(&self) -> usize {
        self.series
            .iter()
            .map(SVPreparedSeries::number_of_bins)
            .sum()
    }
}
