//! The option surface of the parallel-coordinates builder, one method per setting.

use super::super::options::{
    SVDimension, SVParallelCoordinatesMissingPolicy, SVParallelCoordinatesNormalization,
};
use super::builder_state::SVParallelCoordinatesPlotBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;

impl<'values> SVParallelCoordinatesPlotBuilder<'values> {
    /// Attaches the named-column data source the chart reads.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Sets the variables, in axis order. At least two are required.
    #[must_use]
    pub fn variables(mut self, variables: &'values [&'values str]) -> Self {
        self.variables = variables.to_vec();
        self
    }

    /// Binds the optional grouping column that colours each polyline.
    #[must_use]
    pub fn colour(mut self, colour: &'values str) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Sets the stroke width of every polyline.
    #[must_use]
    pub fn line_width(mut self, line_width: SVLength) -> Self {
        self.line_width = line_width;
        self
    }

    /// Sets the opacity of every polyline. Defaults to `0.5`.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Replaces the axis list, and therefore the axis order and visibility.
    #[must_use]
    pub fn dimensions(mut self, dimensions: &'values [SVDimension<'values>]) -> Self {
        self.dimensions = dimensions.to_vec();
        self
    }

    /// Overrides the domain of every visible axis, in axis order.
    #[must_use]
    pub fn range(mut self, range: &'values [(f64, f64)]) -> Self {
        self.range = Some(range.to_vec());
        self
    }

    /// Rotates the variable labels above the axes, in degrees.
    #[must_use]
    pub fn label_angle(mut self, label_angle: f32) -> Self {
        self.label_angle = label_angle;
        self
    }

    /// Sets the label of the whole axis band.
    #[must_use]
    pub fn axis_label(mut self, axis_label: impl Into<String>) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Selects how each axis derives its domain. Defaults to `PerAxis`.
    #[must_use]
    pub fn normalization(mut self, normalization: SVParallelCoordinatesNormalization) -> Self {
        self.normalization = normalization;
        self
    }

    /// Selects what happens to a non-finite value. Defaults to `Gap`.
    #[must_use]
    pub fn missing(mut self, missing: SVParallelCoordinatesMissingPolicy) -> Self {
        self.missing = missing;
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
