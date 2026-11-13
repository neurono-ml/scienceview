//! The validated configuration a built area plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkTheme};
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};

use super::missing_value_policy::SVAreaMissingValuePolicy;

/// How an area's ordered boundary is joined between samples.
///
/// Only the straight segment exists in the mark kernel today. The option is
/// still explicit, so a future interpolation kernel can add variants without
/// changing the builder call sites; the default is the documented `Linear`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVAreaInterpolation {
    /// Straight segments between consecutive samples (default).
    #[default]
    Linear,
}

impl SVAreaInterpolation {
    /// The interpolation name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Linear => "Linear",
        }
    }
}

/// One validated area-plot configuration.
///
/// The builder produces it; the built plot reads it. It borrows the source and
/// the column names, never the data itself.
#[derive(Clone)]
pub(crate) struct SVAreaPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) x: &'values str,
    pub(crate) y: &'values str,
    pub(crate) series: Option<&'values str>,
    pub(crate) stack: bool,
    pub(crate) baseline: SVBaselineMode,
    pub(crate) opacity: Option<f32>,
    pub(crate) line: bool,
    pub(crate) interpolation: SVAreaInterpolation,
    pub(crate) missing_value_policy: SVAreaMissingValuePolicy,
    pub(crate) theme: Option<SVMarkTheme>,
    pub(crate) title: Option<&'values str>,
    pub(crate) x_title: Option<&'values str>,
    pub(crate) y_title: Option<&'values str>,
    pub(crate) x_scale_kind: SVScaleKind,
    pub(crate) temporal_unit: SVTemporalUnit,
    pub(crate) viewport: SVViewport,
}
