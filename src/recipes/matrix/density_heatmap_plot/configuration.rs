//! The validated configuration a built density-heatmap plot carries.

use super::SVEmptyBinPolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::SVColourMap;
use crate::sv_stat::SVNormalisation;

pub(super) const SV_DENSITY_HEATMAP_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_DENSITY_HEATMAP_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_DENSITY_HEATMAP_PLOT_LEFT: f64 = 64.0;
pub(super) const SV_DENSITY_HEATMAP_PLOT_TOP: f64 = 56.0;
pub(super) const SV_DENSITY_HEATMAP_PLOT_WIDTH: f64 = 480.0;
pub(super) const SV_DENSITY_HEATMAP_PLOT_HEIGHT: f64 = 304.0;
pub(super) const SV_DENSITY_HEATMAP_COLOUR_BAR_LEFT: f64 = 576.0;
pub(super) const SV_DENSITY_HEATMAP_COLOUR_BAR_WIDTH: f64 = 48.0;
pub(super) const SV_DENSITY_HEATMAP_COLOUR_BAR_BANDS: usize = 9;
pub(super) const SV_DENSITY_HEATMAP_PIXELS_PER_BIN: f64 = 8.0;

/// The validated configuration a built density-heatmap plot carries.
#[derive(Clone)]
pub(super) struct SVDensityHeatmapConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x: String,
    pub(super) y: String,
    pub(super) series: Option<String>,
    pub(super) weight: Option<String>,
    pub(super) bin_count_x: usize,
    pub(super) bin_count_y: usize,
    pub(super) normalise: SVNormalisation,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_bar: bool,
    pub(super) opacity: f32,
    pub(super) empty_bin_policy: SVEmptyBinPolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
}

impl std::fmt::Debug for SVDensityHeatmapConfiguration<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDensityHeatmapConfiguration")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("bin_count_x", &self.bin_count_x)
            .field("bin_count_y", &self.bin_count_y)
            .field("normalise", &self.normalise)
            .finish_non_exhaustive()
    }
}

/// Returns the bin count derived from a plot extent in pixels.
pub(super) fn sv_derived_bin_count(extent_pixels: f64) -> usize {
    ((extent_pixels / SV_DENSITY_HEATMAP_PIXELS_PER_BIN).round() as usize).max(1)
}
