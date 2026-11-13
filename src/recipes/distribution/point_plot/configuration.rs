//! The validated configuration a built point plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVLength, SVViewport};
use scienceview_core::sv_mark::{SVBarOrientation, SVDashPattern, SVDotShape};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use super::missing_value_policy::SVPointMissingValuePolicy;
use super::ordering::SVPointOrdering;

/// One validated point-plot configuration.
///
/// The builder produces it; the built figure reads it. It borrows the source
/// and the column names, never the data itself.
#[derive(Clone)]
pub(crate) struct SVPointPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) category: &'values str,
    pub(crate) value: &'values str,
    pub(crate) series: Option<&'values str>,
    pub(crate) estimator: SVAggregateMethod,
    pub(crate) error: SVAggregateErrorKind,
    pub(crate) confidence_level: f64,
    pub(crate) ordering: SVPointOrdering,
    pub(crate) missing_value_policy: SVPointMissingValuePolicy,
    pub(crate) orientation: SVBarOrientation,
    pub(crate) marker: SVDotShape,
    pub(crate) caps: bool,
    pub(crate) cap_width: f32,
    pub(crate) join: bool,
    pub(crate) line_width: Option<SVLength>,
    pub(crate) dash: Option<SVDashPattern>,
    pub(crate) colour: Option<SVColour>,
    pub(crate) title: Option<&'values str>,
    pub(crate) x_title: Option<&'values str>,
    pub(crate) y_title: Option<&'values str>,
    pub(crate) viewport: SVViewport,
}

/// Builds the configuration error the recipe reports.
pub(crate) fn sv_invalid(message: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: message.to_string(),
    }
}
