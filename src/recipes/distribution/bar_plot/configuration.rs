//! The validated configuration a built bar plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBarOrientation, SVFormatSpec};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use super::arrangement::SVBarArrangement;
use super::missing_value_policy::SVBarMissingValuePolicy;
use super::ordering::SVBarOrdering;

/// One validated bar-plot configuration.
///
/// The builder produces it; the built figure reads it. It borrows the source
/// and the column names, never the data itself.
#[derive(Clone)]
pub(crate) struct SVBarPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) category: &'values str,
    pub(crate) value: &'values str,
    pub(crate) series: Option<&'values str>,
    pub(crate) weight: Option<&'values str>,
    pub(crate) estimator: SVAggregateMethod,
    pub(crate) error: Option<SVAggregateErrorKind>,
    pub(crate) confidence_level: f64,
    pub(crate) arrangement: SVBarArrangement,
    pub(crate) ordering: SVBarOrdering,
    pub(crate) missing_value_policy: SVBarMissingValuePolicy,
    pub(crate) orientation: SVBarOrientation,
    pub(crate) width: f32,
    pub(crate) colour: Option<SVColour>,
    pub(crate) show_labels: bool,
    pub(crate) label_format: SVFormatSpec,
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
