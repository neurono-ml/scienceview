//! The binning options.

/// How the raw bin counts are normalised before they leave the kernel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVNormalisation {
    /// Raw observation counts.
    #[default]
    Count,
    /// Counts divided by the number of observations inside the edges.
    Probability,
    /// Counts divided by the observation total and the bin width.
    Density,
}

impl SVNormalisation {
    /// Returns the normalisation name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVNormalisation::Count => "Count",
            SVNormalisation::Probability => "Probability",
            SVNormalisation::Density => "Density",
        }
    }
}

/// The configuration of [`SVStat::Bin`](super::SVStat::Bin).
///
/// Explicit edges win over a width, and a width wins over a bin count. When
/// none of the three is given the kernel splits the derived range into the
/// documented default of thirty bins.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBinOptions {
    bin_count: Option<usize>,
    bin_width: Option<f64>,
    bin_edges: Option<Vec<f64>>,
    range: Option<(f64, f64)>,
    normalisation: SVNormalisation,
    cumulative: bool,
}

impl Default for SVBinOptions {
    fn default() -> Self {
        Self {
            bin_count: None,
            bin_width: None,
            bin_edges: None,
            range: None,
            normalisation: SVNormalisation::Count,
            cumulative: false,
        }
    }
}

impl SVBinOptions {
    /// Starts an empty binning configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests an equal split of the range into `count` bins.
    #[must_use]
    pub fn bin_count(mut self, count: usize) -> Self {
        self.bin_count = Some(count);
        self
    }

    /// Requests contiguous bins of exactly `width`.
    #[must_use]
    pub fn bin_width(mut self, width: f64) -> Self {
        self.bin_width = Some(width);
        self
    }

    /// Supplies explicit, strictly increasing edges; they win over every other
    /// boundary option.
    #[must_use]
    pub fn bin_edges(mut self, edges: Vec<f64>) -> Self {
        self.bin_edges = Some(edges);
        self
    }

    /// Fixes the domain instead of deriving it from the observations.
    #[must_use]
    pub fn range(mut self, low: f64, high: f64) -> Self {
        self.range = Some((low, high));
        self
    }

    /// Chooses the normalisation of the returned counts.
    #[must_use]
    pub fn normalise(mut self, normalisation: SVNormalisation) -> Self {
        self.normalisation = normalisation;
        self
    }

    /// Accumulates the normalised counts bin by bin.
    #[must_use]
    pub fn cumulative(mut self, cumulative: bool) -> Self {
        self.cumulative = cumulative;
        self
    }

    /// Returns the requested bin count.
    pub fn requested_bin_count(&self) -> Option<usize> {
        self.bin_count
    }

    /// Returns the requested bin width.
    pub fn requested_bin_width(&self) -> Option<f64> {
        self.bin_width
    }

    /// Returns the explicit edges, when supplied.
    pub fn explicit_edges(&self) -> Option<&[f64]> {
        self.bin_edges.as_deref()
    }

    /// Returns the explicit domain, when supplied.
    pub fn explicit_range(&self) -> Option<(f64, f64)> {
        self.range
    }

    /// Returns the normalisation.
    pub fn normalisation(&self) -> SVNormalisation {
        self.normalisation
    }

    /// Returns `true` when the counts are accumulated.
    pub fn is_cumulative(&self) -> bool {
        self.cumulative
    }
}
