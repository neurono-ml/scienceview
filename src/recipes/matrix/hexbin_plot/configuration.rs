//! The validated configuration a built hexbin carries.

use super::SVHexbinAggregation;
use super::SVHexbinMissingValuePolicy;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform};
use crate::sv_stat::SVNormalisation;

pub(super) const SV_HEXBIN_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_HEXBIN_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_HEXBIN_PLOT_LEFT: f64 = 64.0;
pub(super) const SV_HEXBIN_PLOT_TOP: f64 = 56.0;
pub(super) const SV_HEXBIN_PLOT_WIDTH: f64 = 480.0;
pub(super) const SV_HEXBIN_PLOT_HEIGHT: f64 = 304.0;
pub(super) const SV_HEXBIN_COLOUR_BAR_LEFT: f64 = 576.0;
pub(super) const SV_HEXBIN_COLOUR_BAR_WIDTH: f64 = 48.0;

/// The validated configuration a built hexbin carries.
#[derive(Clone)]
pub(super) struct SVHexbinConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) x: String,
    pub(super) y: String,
    pub(super) series: Option<String>,
    pub(super) value: Option<String>,
    pub(super) aggregation: SVHexbinAggregation,
    pub(super) grid_size: usize,
    pub(super) extent: Option<(f64, f64, f64, f64)>,
    pub(super) min_count: usize,
    pub(super) orientation: f64,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_transform: SVScaleTransform,
    pub(super) normalisation: SVNormalisation,
    pub(super) missing_value_policy: SVHexbinMissingValuePolicy,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
}

impl std::fmt::Debug for SVHexbinConfiguration<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVHexbinConfiguration")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("grid_size", &self.grid_size)
            .field("min_count", &self.min_count)
            .field("aggregation", &self.aggregation)
            .finish_non_exhaustive()
    }
}
