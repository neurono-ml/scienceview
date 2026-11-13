//! The option setters of [`SVContourPlotBuilder`](super::SVContourPlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVContourMissingPolicy;
use super::SVContourPlotBuilder;
use super::SVContourStyle;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform};

impl<'data> SVContourPlotBuilder<'data> {
    /// Attaches the borrowed multi-column source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the required scalar-value column.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Names the horizontal column of a point cloud.
    #[must_use]
    pub fn x(mut self, x: impl Into<String>) -> Self {
        self.x = Some(x.into());
        self
    }

    /// Names the vertical column of a point cloud.
    #[must_use]
    pub fn y(mut self, y: impl Into<String>) -> Self {
        self.y = Some(y.into());
        self
    }

    /// Reads the value column as a row-major matrix of `rows` by `columns`
    /// samples. Defaults to a point cloud when not called.
    #[must_use]
    pub fn matrix(mut self, rows: usize, columns: usize) -> Self {
        self.matrix_shape = Some((rows, columns));
        self
    }

    /// Fixes the horizontal coordinates of a matrix layout.
    #[must_use]
    pub fn x_range(mut self, lower: f64, upper: f64) -> Self {
        self.x_range = Some((lower, upper));
        self
    }

    /// Fixes the vertical coordinates of a matrix layout.
    #[must_use]
    pub fn y_range(mut self, lower: f64, upper: f64) -> Self {
        self.y_range = Some((lower, upper));
        self
    }

    /// Sets the number of interpolation cells per axis for a point cloud;
    /// defaults to `64`.
    #[must_use]
    pub fn grid_count(mut self, grid_count: usize) -> Self {
        self.grid_count = grid_count;
        self
    }

    /// Sets how many contour levels are drawn between the field bounds.
    #[must_use]
    pub fn levels(mut self, levels: usize) -> Self {
        self.levels = levels;
        self
    }

    /// Fixes the interval the levels are spread over instead of the observed
    /// field range.
    #[must_use]
    pub fn level_range(mut self, lower: f64, upper: f64) -> Self {
        self.level_range = Some((lower, upper));
        self
    }

    /// Spaces the levels by a fixed step instead of a fixed count.
    #[must_use]
    pub fn level_size(mut self, level_size: f64) -> Self {
        self.level_size = Some(level_size);
        self
    }

    /// Chooses between filled bands and stroked lines.
    #[must_use]
    pub fn style(mut self, style: SVContourStyle) -> Self {
        self.style = style;
        self
    }

    /// Chooses what happens to a non-finite field sample.
    #[must_use]
    pub fn missing_policy(mut self, missing_policy: SVContourMissingPolicy) -> Self {
        self.missing_policy = missing_policy;
        self
    }

    /// Draws the threshold value beside every contour level.
    #[must_use]
    pub fn labels(mut self, labels: bool) -> Self {
        self.labels = labels;
        self
    }

    /// Selects the continuous colour map the levels are painted through.
    #[must_use]
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = colour_map;
        self
    }

    /// Selects the transform applied before the colour map.
    #[must_use]
    pub fn colour_scale(mut self, colour_scale: SVScaleTransform) -> Self {
        self.colour_scale = colour_scale;
        self
    }

    /// Sets the contour line width.
    #[must_use]
    pub fn line_width(mut self, line_width: f64) -> Self {
        self.line_width = line_width;
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
