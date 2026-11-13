//! The validated configuration a built density-contour plot carries.

use super::SVContourStyle;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform};

pub(super) const SV_DENSITY_CONTOUR_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_DENSITY_CONTOUR_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_DENSITY_CONTOUR_PLOT_LEFT: f64 = 64.0;
pub(super) const SV_DENSITY_CONTOUR_PLOT_TOP: f64 = 56.0;
pub(super) const SV_DENSITY_CONTOUR_PLOT_WIDTH: f64 = 480.0;
pub(super) const SV_DENSITY_CONTOUR_PLOT_HEIGHT: f64 = 304.0;
pub(super) const SV_DENSITY_CONTOUR_COLOUR_BAR_LEFT: f64 = 576.0;
pub(super) const SV_DENSITY_CONTOUR_COLOUR_BAR_WIDTH: f64 = 48.0;

/// The validated configuration a built density-contour plot carries.
#[derive(Clone)]
pub(super) struct SVDensityContourConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x: String,
    pub(super) y: String,
    pub(super) series: Option<String>,
    pub(super) weight: Option<String>,
    pub(super) levels: usize,
    pub(super) level_range: Option<(f64, f64)>,
    pub(super) bandwidth: Option<f64>,
    pub(super) bw_adjust: f64,
    pub(super) grid_count: usize,
    pub(super) style: SVContourStyle,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_scale: SVScaleTransform,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
    pub(super) line_width: f64,
}

impl std::fmt::Debug for SVDensityContourConfiguration<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVDensityContourConfiguration")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("levels", &self.levels)
            .field("grid_count", &self.grid_count)
            .field("style", &self.style)
            .finish_non_exhaustive()
    }
}
