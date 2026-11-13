//! The validated configuration the built violin plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVBarOrientation;

use super::missing_value_policy::SVViolinMissingValuePolicy;
use super::scale_mode::SVViolinScale;

/// One validated violin-plot configuration.
///
/// The builder produces it; the built figure reads it. It borrows the source
/// and the column names, never the data itself.
#[derive(Clone)]
pub(crate) struct SVViolinPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) value: &'values str,
    pub(crate) category: &'values str,
    pub(crate) series: Option<&'values str>,
    pub(crate) bandwidth: Option<f64>,
    pub(crate) bandwidth_adjustment: f64,
    pub(crate) width: f32,
    pub(crate) scale: SVViolinScale,
    pub(crate) show_box: bool,
    pub(crate) show_points: bool,
    pub(crate) split: bool,
    pub(crate) orientation: SVBarOrientation,
    pub(crate) missing_value_policy: SVViolinMissingValuePolicy,
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
