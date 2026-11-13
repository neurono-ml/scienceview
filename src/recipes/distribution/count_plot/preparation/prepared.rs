//! The owned, data-space result of one count preparation.

use crate::sv_scale::SVColour;

/// One series of counted categories, ready to place.
pub(crate) struct SVPreparedSeries {
    pub(crate) label: String,
    pub(crate) colour: SVColour,
    /// The counted value of every category, before any stacking.
    pub(crate) counts: Vec<f64>,
    /// The drawn top of every bar, after the configured move.
    pub(crate) tops: Vec<f64>,
    pub(crate) baselines: Vec<f64>,
    pub(crate) width_fractions: Vec<f64>,
    pub(crate) offsets: Vec<f64>,
}

/// The prepared count plot: the shared axis and the arranged series.
pub(crate) struct SVPreparedCount {
    pub(crate) categories: Vec<f64>,
    pub(crate) series: Vec<SVPreparedSeries>,
}

impl SVPreparedCount {
    /// Returns the number of drawn bars: one per category of every series.
    pub(crate) fn number_of_bars(&self) -> usize {
        self.series.len() * self.categories.len()
    }
}
