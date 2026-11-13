//! The tail-stopping rule that derives the nesting depth of a boxen plot
//! (FDR-118).
//!
//! The rule never computes a statistic itself: it only decides how many
//! letter-value levels the nest has. The letter values themselves always come
//! from [`SVStat::Percentile`](scienceview_core::sv_stat::SVStat::Percentile).

/// How the nesting depth is derived from the sample size.
///
/// The names follow the letter-value plot literature (Henderson and Tukey):
/// the depth is the number of boxes on each side of the median, so a depth of
/// `k` produces `2k - 1` boxes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVBoxenTailRule {
    /// Henderson's rule of thumb: `floor(log2(n)) - 3`, at least one level.
    ///
    /// It keeps roughly the same number of observations in the outermost tail
    /// as a conventional box plot leaves as outliers.
    Tukey,
    /// `floor(log2(n)) - floor(log2(n * proportion)) + 1`, at least one level.
    ///
    /// The proportion is the expected fraction of observations beyond the
    /// outermost letter value and must lie strictly inside `(0, 1)`.
    Proportion(f64),
    /// Extends the nest to the full observed range: `floor(log2(n)) + 1`
    /// levels, with the outermost letter values at the sample minimum and
    /// maximum. Nothing is drawn as an outlier.
    Full,
}

impl Default for SVBoxenTailRule {
    fn default() -> Self {
        Self::Tukey
    }
}

impl SVBoxenTailRule {
    /// Builds a proportion rule from an expected outlier proportion.
    pub fn proportion(proportion: f64) -> Self {
        Self::Proportion(proportion)
    }

    /// The expected outlier proportion of a `Proportion` rule, if any.
    pub fn proportion_value(&self) -> Option<f64> {
        match self {
            Self::Proportion(proportion) => Some(*proportion),
            Self::Tukey | Self::Full => None,
        }
    }

    /// Derives the nesting depth for a sample of `sample_size` observations.
    pub fn depth(&self, sample_size: usize) -> usize {
        let logarithm = (sample_size.max(1) as f64).log2().floor();
        let depth = match self {
            Self::Tukey => logarithm - 3.0,
            Self::Proportion(proportion) => {
                logarithm - (sample_size.max(1) as f64 * proportion).log2().floor() + 1.0
            }
            Self::Full => logarithm + 1.0,
        };
        if depth.is_finite() && depth >= 1.0 {
            depth as usize
        } else {
            1
        }
    }
}
