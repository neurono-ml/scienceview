//! The option setters of [`SVHistogramPlotBuilder`].
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVHistogramPlotBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_mark::SVBarOrientation;
use crate::sv_recipes::sv_distribution::sv_histogram_plot::{
    SVBarElement, SVHistogramMissingValuePolicy, SVHistogramMultiple,
};
use crate::sv_stat::SVNormalisation;

impl<'data> SVHistogramPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required continuous column to bin.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Names the optional grouping column; every series gets its own bins.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Supplies human labels for the series values, in first-appearance order.
    #[must_use]
    pub fn series_labels(mut self, labels: &[&str]) -> Self {
        self.series_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Names the optional observation weight column.
    ///
    /// The shipped `SVStat::Bin` kernel does not consume weights, so a
    /// configured weight is rejected at build time with a typed configuration
    /// error naming the kernel gap; the recipe never reimplements weighted
    /// binning.
    #[must_use]
    pub fn weight(mut self, weight: impl Into<String>) -> Self {
        self.weight = Some(weight.into());
        self
    }

    /// Requests an equal split of the range into `bin_count` bins.
    #[must_use]
    pub fn bin_count(mut self, bin_count: usize) -> Self {
        self.bin_count = Some(bin_count);
        self
    }

    /// Requests contiguous bins of exactly `bin_width`.
    #[must_use]
    pub fn bin_width(mut self, bin_width: f64) -> Self {
        self.bin_width = Some(bin_width);
        self
    }

    /// Supplies explicit, strictly increasing bin edges.
    #[must_use]
    pub fn bin_edges(mut self, bin_edges: Vec<f64>) -> Self {
        self.bin_edges = Some(bin_edges);
        self
    }

    /// Selects the normalisation of the bin heights.
    #[must_use]
    pub fn normalise(mut self, normalisation: SVNormalisation) -> Self {
        self.normalisation = normalisation;
        self
    }

    /// Accumulates the bin heights bin by bin.
    #[must_use]
    pub fn cumulative(mut self, cumulative: bool) -> Self {
        self.cumulative = cumulative;
        self
    }

    /// Normalises every series against the combined observation total.
    #[must_use]
    pub fn common_normalisation(mut self, common_normalisation: bool) -> Self {
        self.common_normalisation = common_normalisation;
        self
    }

    /// Shares one set of bin edges across every series (default `true`).
    #[must_use]
    pub fn common_bins(mut self, common_bins: bool) -> Self {
        self.common_bins = common_bins;
        self
    }

    /// Sets the fraction of a bin slot the bar occupies, from `0` to `1`.
    #[must_use]
    pub fn shrink(mut self, shrink: f64) -> Self {
        self.shrink = shrink;
        self
    }

    /// Selects how the bins are drawn: bars, a stepped outline or a polygon.
    #[must_use]
    pub fn element(mut self, element: SVBarElement) -> Self {
        self.element = element;
        self
    }

    /// Selects whether the bins grow upwards or rightwards.
    #[must_use]
    pub fn orientation(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Selects how several series share one bin: layered, stacked or dodged.
    #[must_use]
    pub fn multiple(mut self, multiple: SVHistogramMultiple) -> Self {
        self.multiple = multiple;
        self
    }

    /// Selects what happens to a missing or non-finite observation.
    #[must_use]
    pub fn missing_value_policy(
        mut self,
        missing_value_policy: SVHistogramMissingValuePolicy,
    ) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the horizontal axis title.
    #[must_use]
    pub fn x_label(mut self, x_label: impl Into<String>) -> Self {
        self.x_label = Some(x_label.into());
        self
    }

    /// Sets the vertical axis title.
    #[must_use]
    pub fn y_label(mut self, y_label: impl Into<String>) -> Self {
        self.y_label = Some(y_label.into());
        self
    }

    /// Selects the rendering backend. Required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Selects the execution mode. Defaults to `Automatic`.
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
