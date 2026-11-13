//! The option setters of
//! [`SVDensityHeatmapPlotBuilder`](super::SVDensityHeatmapPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVDensityHeatmapPlotBuilder;
use super::options::SVEmptyBinPolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::SVColourMap;
use crate::sv_stat::SVNormalisation;

impl<'data> SVDensityHeatmapPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required horizontal column.
    #[must_use]
    pub fn x(mut self, x: impl Into<String>) -> Self {
        self.x = Some(x.into());
        self
    }

    /// Names the required vertical column.
    #[must_use]
    pub fn y(mut self, y: impl Into<String>) -> Self {
        self.y = Some(y.into());
        self
    }

    /// Names the optional grouping column; one raster is drawn per value.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Names the optional observation-weight column.
    #[must_use]
    pub fn weight(mut self, weight: impl Into<String>) -> Self {
        self.weight = Some(weight.into());
        self
    }

    /// Sets the number of horizontal bins; defaults to one per eight pixels of
    /// the plot width.
    #[must_use]
    pub fn bin_count_x(mut self, bin_count_x: usize) -> Self {
        self.bin_count_x = Some(bin_count_x);
        self
    }

    /// Sets the number of vertical bins; defaults to one per eight pixels of
    /// the plot height.
    #[must_use]
    pub fn bin_count_y(mut self, bin_count_y: usize) -> Self {
        self.bin_count_y = Some(bin_count_y);
        self
    }

    /// Chooses how the accumulated cell weight is normalised.
    #[must_use]
    pub fn normalise(mut self, normalisation: SVNormalisation) -> Self {
        self.normalise = normalisation;
        self
    }

    /// Selects the continuous colour map the cells are painted through.
    #[must_use]
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = colour_map;
        self
    }

    /// Shows or hides the colour bar; shown by default.
    #[must_use]
    pub fn colour_bar(mut self, colour_bar: bool) -> Self {
        self.colour_bar = colour_bar;
        self
    }

    /// Sets the cell opacity, which must lie in `0.0..=1.0`; defaults to `1.0`.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Chooses how an empty bin is painted.
    #[must_use]
    pub fn empty_bin_policy(mut self, empty_bin_policy: SVEmptyBinPolicy) -> Self {
        self.empty_bin_policy = empty_bin_policy;
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
