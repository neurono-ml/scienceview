//! The option setters of [`SVDensityContourPlotBuilder`](super::SVDensityContourPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVContourStyle;
use super::SVDensityContourPlotBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform};

impl<'data> SVDensityContourPlotBuilder<'data> {
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

    /// Names the optional grouping column; one density field is drawn per value.
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

    /// Sets how many contour levels are drawn between the density bounds.
    #[must_use]
    pub fn levels(mut self, levels: usize) -> Self {
        self.levels = levels;
        self
    }

    /// Fixes the interval the levels are spread over instead of the observed
    /// density range.
    #[must_use]
    pub fn level_range(mut self, lower: f64, upper: f64) -> Self {
        self.level_range = Some((lower, upper));
        self
    }

    /// Requests a Gaussian smoothing pass with this bandwidth in data units.
    #[must_use]
    pub fn bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Scales the requested bandwidth; defaults to `1.0`.
    #[must_use]
    pub fn bw_adjust(mut self, bw_adjust: f64) -> Self {
        self.bw_adjust = bw_adjust;
        self
    }

    /// Sets the number of grid cells per axis; defaults to `256`.
    #[must_use]
    pub fn grid_count(mut self, grid_count: usize) -> Self {
        self.grid_count = grid_count;
        self
    }

    /// Chooses between filled bands and stroked lines.
    #[must_use]
    pub fn style(mut self, style: SVContourStyle) -> Self {
        self.style = style;
        self
    }

    /// Selects the continuous colour map the levels are painted through.
    #[must_use]
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = colour_map;
        self
    }

    /// Selects the transform applied to the density before the colour map.
    #[must_use]
    pub fn colour_scale(mut self, colour_scale: SVScaleTransform) -> Self {
        self.colour_scale = colour_scale;
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
