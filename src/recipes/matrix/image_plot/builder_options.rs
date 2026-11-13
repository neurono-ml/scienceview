//! The option setters of [`SVImagePlotBuilder`](super::SVImagePlotBuilder).
//!
//! Every setter is a consuming method that changes one option; all validation
//! happens in `build`.

use super::SVImagePlotBuilder;
use super::{
    SVImageAspectRatio, SVImageExtent, SVImageInterpolation, SVImageMissingPolicy, SVImageOrigin,
    SVImageSource,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::SVColourMap;

impl<'data> SVImagePlotBuilder<'data> {
    /// Attaches the borrowed data source an intensity raster is read from.
    #[must_use]
    pub fn data_source(mut self, data_source: &'data dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Selects the raster the chart draws.
    #[must_use]
    pub fn raster(mut self, source: SVImageSource<'data>) -> Self {
        self.source = Some(source);
        self
    }

    /// Maps a numeric raster through this colour map instead of greyscale.
    ///
    /// A theme-resolved map such as `SVColourMap::from_theme(&theme, "viridis")`
    /// keeps its token, so a theme recompilation restyles the pixels on the next
    /// render.
    #[must_use]
    pub fn colour_map(mut self, colour_map: SVColourMap) -> Self {
        self.colour_map = Some(colour_map);
        self
    }

    /// Selects how the raster is sampled when it is scaled (default nearest).
    #[must_use]
    pub fn interpolation(mut self, interpolation: SVImageInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Sets the intensity mapped to the colour map's first stop (plotly `zmin`).
    #[must_use]
    pub fn minimum_intensity(mut self, minimum_intensity: f64) -> Self {
        self.minimum_intensity = Some(minimum_intensity);
        self
    }

    /// Sets the intensity mapped to the colour map's last stop (plotly `zmax`).
    #[must_use]
    pub fn maximum_intensity(mut self, maximum_intensity: f64) -> Self {
        self.maximum_intensity = Some(maximum_intensity);
        self
    }

    /// Sets the constant opacity, from `0.0` to `1.0`.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Selects which edge of the extent the first raster row touches.
    #[must_use]
    pub fn origin(mut self, origin: SVImageOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Selects whether pixels stay square or stretch to the extent.
    #[must_use]
    pub fn aspect_ratio(mut self, aspect_ratio: SVImageAspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Selects what a missing pixel becomes.
    #[must_use]
    pub fn missing_policy(mut self, missing_policy: SVImageMissingPolicy) -> Self {
        self.missing_policy = missing_policy;
        self
    }

    /// Draws the colour bar beside the plot area when the raster is numeric.
    #[must_use]
    pub fn colour_bar(mut self, colour_bar: bool) -> Self {
        self.colour_bar = colour_bar;
        self
    }

    /// Places the raster on explicit data extents instead of `0..columns` by `0..rows`.
    #[must_use]
    pub fn extent(mut self, extent: SVImageExtent) -> Self {
        self.extent = Some(extent);
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
