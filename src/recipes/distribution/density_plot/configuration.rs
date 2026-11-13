//! The validated configuration a built density plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::SVMarkTheme;
use scienceview_core::sv_stat::{SVKernelShape, SVNormalisation};

use super::missing_value_policy::SVDensityMissingValuePolicy;

/// One validated density-plot configuration.
///
/// The builder produces it; the built plot reads it. It borrows the source and
/// the column names, never the data itself.
#[derive(Clone)]
pub(crate) struct SVDensityPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) value: &'values str,
    pub(crate) series: Option<&'values str>,
    pub(crate) weight: Option<&'values str>,
    pub(crate) bandwidth: Option<f64>,
    pub(crate) bandwidth_adjustment: f64,
    pub(crate) kernel: SVKernelShape,
    pub(crate) grid_count: usize,
    pub(crate) cut: f64,
    pub(crate) fill: bool,
    pub(crate) line_width: Option<SVLength>,
    pub(crate) normalisation: SVNormalisation,
    pub(crate) missing_value_policy: SVDensityMissingValuePolicy,
    pub(crate) theme: Option<SVMarkTheme>,
    pub(crate) title: Option<&'values str>,
    pub(crate) x_title: Option<&'values str>,
    pub(crate) y_title: Option<&'values str>,
    pub(crate) viewport: SVViewport,
}
