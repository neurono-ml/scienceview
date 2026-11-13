//! The validated configuration a built contour plot carries.

use super::options::{SVContourMissingPolicy, SVContourStyle};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_figure::SVExecutionMode;
use crate::sv_scale::{SVColourMap, SVScaleTransform};

pub(super) const SV_CONTOUR_VIEWPORT_WIDTH: f64 = 640.0;
pub(super) const SV_CONTOUR_VIEWPORT_HEIGHT: f64 = 400.0;
pub(super) const SV_CONTOUR_PLOT_LEFT: f64 = 64.0;
pub(super) const SV_CONTOUR_PLOT_TOP: f64 = 56.0;
pub(super) const SV_CONTOUR_PLOT_WIDTH: f64 = 480.0;
pub(super) const SV_CONTOUR_PLOT_HEIGHT: f64 = 304.0;
pub(super) const SV_CONTOUR_COLOUR_BAR_LEFT: f64 = 576.0;
pub(super) const SV_CONTOUR_COLOUR_BAR_WIDTH: f64 = 48.0;

/// How the value column is folded into a lattice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SVContourLayout {
    /// `x`, `y` and `value` are equal-length observation columns, interpolated
    /// onto a `grid_count` lattice through the bin kernel.
    Points,
    /// The value column is a row-major matrix of `rows` by `columns` samples.
    Matrix { rows: usize, columns: usize },
}

/// The validated configuration a built contour plot carries.
#[derive(Clone)]
pub(super) struct SVContourConfiguration<'data> {
    pub(super) data_source: &'data dyn SVDataSource,
    pub(super) value: String,
    pub(super) x: Option<String>,
    pub(super) y: Option<String>,
    pub(super) layout: SVContourLayout,
    pub(super) x_range: Option<(f64, f64)>,
    pub(super) y_range: Option<(f64, f64)>,
    pub(super) grid_count: usize,
    pub(super) levels: usize,
    pub(super) level_range: Option<(f64, f64)>,
    pub(super) level_size: Option<f64>,
    pub(super) style: SVContourStyle,
    pub(super) missing_policy: SVContourMissingPolicy,
    pub(super) labels: bool,
    pub(super) colour_map: SVColourMap,
    pub(super) colour_scale: SVScaleTransform,
    pub(super) line_width: f64,
    pub(super) title: Option<String>,
    pub(super) x_label: Option<String>,
    pub(super) y_label: Option<String>,
    pub(super) backend_kind: SVBackendKind,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) number_of_points: usize,
    pub(super) representation: SVRepresentation,
}

impl std::fmt::Debug for SVContourConfiguration<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVContourConfiguration")
            .field("value", &self.value)
            .field("layout", &self.layout)
            .field("levels", &self.levels)
            .field("style", &self.style)
            .finish_non_exhaustive()
    }
}
