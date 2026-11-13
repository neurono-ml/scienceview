//! The validated configuration the built box plot carries.

use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBarOrientation, SVDotShape};

use super::whisker::SVWhiskerRule;

/// One validated box-plot configuration.
///
/// The builder produces it; the built figure reads it. It borrows the source
/// and the column names, never the data itself.
#[derive(Clone)]
pub(crate) struct SVBoxPlotConfiguration<'values> {
    pub(crate) data_source: &'values dyn SVDataSource,
    pub(crate) value: &'values str,
    pub(crate) category: &'values str,
    pub(crate) series: Option<&'values str>,
    pub(crate) quartiles: Vec<f64>,
    pub(crate) whisker: SVWhiskerRule,
    pub(crate) show_outliers: bool,
    pub(crate) outlier_marker: SVDotShape,
    pub(crate) notch: bool,
    pub(crate) show_mean: bool,
    pub(crate) width: f32,
    pub(crate) orientation: SVBarOrientation,
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
