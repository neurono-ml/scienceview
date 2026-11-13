//! Every optional builder setter.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVBarOrientation;

use super::super::missing_value_policy::SVViolinMissingValuePolicy;
use super::super::scale_mode::SVViolinScale;
use super::SVViolinPlotBuilder;

impl<'values> SVViolinPlotBuilder<'values> {
    /// Attaches the data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the observed numeric column.
    #[must_use]
    pub fn value(mut self, value: &'values str) -> Self {
        self.value = Some(value);
        self
    }

    /// Names the categorical axis column.
    #[must_use]
    pub fn category(mut self, category: &'values str) -> Self {
        self.category = Some(category);
        self
    }

    /// Names the optional series column: one violin per series in a category.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Fixes the bandwidth instead of deriving it with Silverman's rule.
    #[must_use]
    pub fn bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Scales the bandwidth: the estimate uses `bandwidth * adjustment`.
    #[must_use]
    pub fn bandwidth_adjustment(mut self, bandwidth_adjustment: f64) -> Self {
        self.bandwidth_adjustment = bandwidth_adjustment;
        self
    }

    /// Sets the violin width as a fraction of the category slot (default `0.8`).
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Chooses how the density is scaled into the slot (default
    /// [`SVViolinScale::Area`]).
    #[must_use]
    pub fn scale(mut self, scale: SVViolinScale) -> Self {
        self.scale = scale;
        self
    }

    /// Draws the inner box, quartile edges and median (default `false`).
    #[must_use]
    pub fn show_box(mut self, show_box: bool) -> Self {
        self.show_box = show_box;
        self
    }

    /// Draws the sample points inside every violin (default `false`).
    #[must_use]
    pub fn show_points(mut self, show_points: bool) -> Self {
        self.show_points = show_points;
        self
    }

    /// Splits two series into the two halves of one violin (default `false`).
    #[must_use]
    pub fn split(mut self, split: bool) -> Self {
        self.split = split;
        self
    }

    /// Chooses the value orientation (default
    /// [`SVBarOrientation::Vertical`]).
    #[must_use]
    pub fn orient(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Chooses what happens to a non-finite observation.
    #[must_use]
    pub fn missing_value_policy(
        mut self,
        missing_value_policy: SVViolinMissingValuePolicy,
    ) -> Self {
        self.missing_value_policy = missing_value_policy;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: &'values str) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the title of the categorical axis.
    #[must_use]
    pub fn x_title(mut self, x_title: &'values str) -> Self {
        self.x_title = Some(x_title);
        self
    }

    /// Sets the title of the value axis.
    #[must_use]
    pub fn y_title(mut self, y_title: &'values str) -> Self {
        self.y_title = Some(y_title);
        self
    }

    /// Sets the logical viewport.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Selects the rendering backend, which is required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Sets the execution mode (default `Automatic`).
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}
