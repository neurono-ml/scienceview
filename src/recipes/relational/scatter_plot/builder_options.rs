//! The option setters of [`SVScatterPlotBuilder`](super::SVScatterPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVScatterPlotBuilder;
use super::{SVScatterLegendPlacement, SVScatterSizeScale};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVDotShape;
use crate::sv_move::SVMove;
use crate::sv_scale::SVColour;

impl<'data> SVScatterPlotBuilder<'data> {
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

    /// Names the optional grouping column.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Names the optional continuous size column.
    #[must_use]
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Names the optional continuous colour column.
    #[must_use]
    pub fn colour(mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self
    }

    /// Supplies human labels for the series values, in first-appearance order.
    #[must_use]
    pub fn series_labels(mut self, labels: &[&str]) -> Self {
        self.series_labels = labels.iter().map(|label| (*label).to_string()).collect();
        self
    }

    /// Selects the marker shape.
    #[must_use]
    pub fn marker(mut self, marker: SVDotShape) -> Self {
        self.marker = marker;
        self
    }

    /// Sets the display interval of the continuous size column.
    #[must_use]
    pub fn size_scale(mut self, size_scale: SVScatterSizeScale) -> Self {
        self.size_scale = size_scale;
        self
    }

    /// Sets the constant opacity, from `0.0` to `1.0`.
    #[must_use]
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Paints a stroke in this colour around every marker.
    #[must_use]
    pub fn edge_colour(mut self, edge_colour: SVColour) -> Self {
        self.edge_colour = Some(edge_colour);
        self
    }

    /// Sets the marker stroke width.
    #[must_use]
    pub fn edge_width(mut self, edge_width: SVLength) -> Self {
        self.edge_width = edge_width;
        self
    }

    /// Applies a displacement move before the scales; the scatter supports jitter and dodge.
    #[must_use]
    pub fn jitter(mut self, jitter: SVMove) -> Self {
        self.jitter = Some(jitter);
        self
    }

    /// Selects where the series legend is placed.
    #[must_use]
    pub fn legend(mut self, legend: SVScatterLegendPlacement) -> Self {
        self.legend = legend;
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
