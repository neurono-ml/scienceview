//! The option setters of [`SVHexbinPlotBuilder`].
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::{SVHexbinAggregation, SVHexbinMissingValuePolicy, SVHexbinPlotBuilder};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform};
use crate::sv_stat::SVNormalisation;

impl<'data> SVHexbinPlotBuilder<'data> {
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

    /// Names the optional grouping column; one lattice is drawn per value.
    #[must_use]
    pub fn series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    /// Names the optional value column consumed by mean aggregation.
    #[must_use]
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Chooses what each hexagon's colour encodes.
    #[must_use]
    pub fn aggregation(mut self, aggregation: SVHexbinAggregation) -> Self {
        self.aggregation = aggregation;
        self
    }

    /// Sets how many hexagons span the horizontal extent; defaults to `20`.
    #[must_use]
    pub fn grid_size(mut self, grid_size: usize) -> Self {
        self.grid_size = grid_size;
        self
    }

    /// Fixes the mapped domain as `(x_lower, x_upper, y_lower, y_upper)`.
    #[must_use]
    pub fn extent(mut self, x_lower: f64, x_upper: f64, y_lower: f64, y_upper: f64) -> Self {
        self.extent = Some((x_lower, x_upper, y_lower, y_upper));
        self
    }

    /// Drops every hexagon with fewer than `min_count` observations.
    #[must_use]
    pub fn min_count(mut self, min_count: usize) -> Self {
        self.min_count = min_count;
        self
    }

    /// Rotates each hexagon glyph by `orientation` radians.
    #[must_use]
    pub fn orientation(mut self, orientation: f64) -> Self {
        self.orientation = orientation;
        self
    }

    /// Selects the continuous colour map the counts are painted through.
    #[must_use]
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = colour_map;
        self
    }

    /// Selects the transform applied to the count before the colour map.
    #[must_use]
    pub fn colour_scale(mut self, colour_scale: SVScaleTransform) -> Self {
        self.colour_transform = colour_scale;
        self
    }

    /// Chooses how raw counts are normalised; defaults to `Count`.
    #[must_use]
    pub fn normalise(mut self, normalisation: SVNormalisation) -> Self {
        self.normalisation = normalisation;
        self
    }

    /// Chooses what happens to a non-finite observation.
    #[must_use]
    pub fn missing_value_policy(
        mut self,
        missing_value_policy: SVHexbinMissingValuePolicy,
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
