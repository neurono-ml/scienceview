//! The point estimates one operation reports.
//!
//! The estimator and the interval come from
//! [`SVStat::Aggregate`](scienceview_core::sv_stat::SVStat::Aggregate); the
//! grouping layout comes from [`SVMove`](scienceview_core::sv_move::SVMove).
//! The recipe only decides which move to apply and how to label the result.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::SVAggregateErrorInterval;

use super::aggregation::{sv_aggregate, sv_axes, sv_category_order};
use super::configuration::SVPointPlotConfiguration;
use super::data_contract::SVPointData;
use super::placement::sv_place_estimates;

/// One drawn point: an aggregate estimate, its interval and its placed slot.
#[derive(Clone, Debug, PartialEq)]
pub struct SVPointEstimate {
    pub(crate) category: f64,
    pub(crate) series: Option<f64>,
    pub(crate) category_index: usize,
    pub(crate) series_index: usize,
    pub(crate) value: f64,
    pub(crate) error: Option<SVAggregateErrorInterval>,
    pub(crate) number_of_observations: usize,
    pub(crate) position: f64,
}

impl SVPointEstimate {
    /// The category code this point belongs to.
    pub fn category(&self) -> f64 {
        self.category
    }

    /// The series code this point belongs to, when a series column was supplied.
    pub fn series(&self) -> Option<f64> {
        self.series
    }

    /// The index of the category on the ordered axis.
    pub fn category_index(&self) -> usize {
        self.category_index
    }

    /// The index of the series in the legend.
    pub fn series_index(&self) -> usize {
        self.series_index
    }

    /// The aggregate value the point encodes.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// The uncertainty interval the error bar draws, when the estimator has one.
    pub fn error(&self) -> Option<SVAggregateErrorInterval> {
        self.error
    }

    /// The number of observations the aggregate consumed.
    pub fn number_of_observations(&self) -> usize {
        self.number_of_observations
    }

    /// The category slot plus the dodge offset, in slot units.
    pub fn position(&self) -> f64 {
        self.position
    }
}

/// The categories, the series and the estimates of one operation.
#[derive(Clone, Debug, PartialEq)]
pub struct SVPointSummary {
    categories: Vec<f64>,
    series: Vec<Option<f64>>,
    estimates: Vec<SVPointEstimate>,
}

impl SVPointSummary {
    /// The category codes in draw order.
    pub fn categories(&self) -> &[f64] {
        &self.categories
    }

    /// The series codes in legend order.
    pub fn series(&self) -> &[Option<f64>] {
        &self.series
    }

    /// The drawn points, one per category and series.
    pub fn estimates(&self) -> &[SVPointEstimate] {
        &self.estimates
    }

    pub(crate) fn new(
        categories: Vec<f64>,
        series: Vec<Option<f64>>,
        estimates: Vec<SVPointEstimate>,
    ) -> Self {
        Self {
            categories,
            series,
            estimates,
        }
    }
}

/// Aggregates the observations and places the points.
pub(crate) fn sv_summarize(
    data: &SVPointData,
    configuration: &SVPointPlotConfiguration<'_>,
) -> Result<SVPointSummary, SVError> {
    let axes = sv_axes(data);
    let aggregates = sv_aggregate(data, &axes, configuration)?;
    let category_order = sv_category_order(&axes, &aggregates, configuration.ordering);
    sv_place_estimates(&axes, &aggregates, &category_order, configuration)
}
