//! The validated configuration the built band plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_scale::{SVScaleKind, SVTemporalUnit};
use scienceview_core::sv_stat::SVAggregateErrorKind;

use super::missing_value_policy::SVMissingValuePolicy;

/// One validated band-plot configuration.
///
/// The builder produces it; the built figure reads it. It borrows the source
/// and the column names, never the data itself.
#[derive(Clone, Copy)]
pub(crate) struct SVBandPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) x: &'values str,
    pub(crate) lower: Option<&'values str>,
    pub(crate) upper: Option<&'values str>,
    pub(crate) values: Option<&'values str>,
    pub(crate) series: Option<&'values str>,
    pub(crate) derived: bool,
    pub(crate) error: SVAggregateErrorKind,
    pub(crate) opacity: Option<f32>,
    pub(crate) fill: bool,
    pub(crate) border: bool,
    pub(crate) dashed_borders: bool,
    pub(crate) centre_line: bool,
    pub(crate) missing_value_policy: SVMissingValuePolicy,
    pub(crate) title: Option<&'values str>,
    pub(crate) x_title: Option<&'values str>,
    pub(crate) y_title: Option<&'values str>,
    pub(crate) x_scale_kind: SVScaleKind,
    pub(crate) temporal_unit: SVTemporalUnit,
    pub(crate) viewport: SVViewport,
}

/// Builds the configuration error the recipe reports.
pub(crate) fn sv_invalid(message: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: message.to_string(),
    }
}
