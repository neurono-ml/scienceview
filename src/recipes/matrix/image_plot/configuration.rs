//! The validated configuration a built image plot carries.

use super::{
    SVImageAspectRatio, SVImageExtent, SVImageInterpolation, SVImageMissingPolicy, SVImageOrigin,
    SVImageSource,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::SVColourMap;

pub(super) const SV_IMAGE_VIEWPORT_WIDTH: f64 = 680.0;
pub(super) const SV_IMAGE_VIEWPORT_HEIGHT: f64 = 512.0;
pub(super) const SV_IMAGE_PLOT_LEFT: f64 = 80.0;
pub(super) const SV_IMAGE_PLOT_TOP: f64 = 56.0;
pub(super) const SV_IMAGE_PLOT_WIDTH: f64 = 480.0;
pub(super) const SV_IMAGE_PLOT_HEIGHT: f64 = 384.0;
pub(super) const SV_IMAGE_COLOUR_BAR_GAP: f64 = 12.0;

/// The validated configuration a built image plot carries.
#[derive(Clone)]
pub(super) struct SVImageConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) source: SVImageSource<'data>,
    pub(super) colour_map: Option<SVColourMap>,
    pub(super) interpolation: SVImageInterpolation,
    pub(super) minimum_intensity: Option<f64>,
    pub(super) maximum_intensity: Option<f64>,
    pub(super) opacity: f32,
    pub(super) origin: SVImageOrigin,
    pub(super) aspect_ratio: SVImageAspectRatio,
    pub(super) missing_policy: SVImageMissingPolicy,
    pub(super) colour_bar: bool,
    pub(super) extent: Option<SVImageExtent>,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) representation: SVRepresentation,
    pub(super) number_of_pixels: usize,
}

/// Returns the extent a configuration draws, defaulting to the raster grid.
pub(super) fn sv_resolved_extent(configuration: &SVImageConfiguration<'_>) -> SVImageExtent {
    configuration
        .extent
        .unwrap_or_else(|| SVImageExtent::unit(configuration.source.dimensions()))
}
