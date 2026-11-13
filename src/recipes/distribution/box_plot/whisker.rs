//! The whisker rule of a box plot (FDR-116).
//!
//! The rule never computes a statistic itself: it selects which observations
//! become the whisker ends and which become outliers, after
//! [`SVStat::Percentile`](scienceview_core::sv_stat::SVStat::Percentile) has
//! produced the quartiles and any percentile the rule asks for.

/// How the ends of the whiskers are chosen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVWhiskerRule {
    /// The most extreme observations inside 1.5 times the interquartile range.
    Tukey,
    /// The minimum and the maximum observations; nothing is an outlier.
    MinMax,
    /// The given lower percentile and its upper complement in `(0, 0.5)`.
    Percentile(f64),
}

impl Default for SVWhiskerRule {
    fn default() -> Self {
        Self::Tukey
    }
}

impl SVWhiskerRule {
    /// Builds a percentile rule from a lower percentile such as `0.05`.
    pub fn percentile(percentile: f64) -> Self {
        Self::Percentile(percentile)
    }

    /// Returns the lower percentile a `Percentile` rule uses, if any.
    pub fn percentile_value(&self) -> Option<f64> {
        match self {
            Self::Percentile(percentile) => Some(*percentile),
            Self::Tukey | Self::MinMax => None,
        }
    }
}
