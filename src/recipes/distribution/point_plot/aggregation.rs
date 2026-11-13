//! Aggregating each category (and series) into one point estimate.
//!
//! The estimator and the interval come from
//! [`SVStat::Aggregate`](scienceview_core::sv_stat::SVStat::Aggregate); the
//! recipe only decides which groups to request and in which order the
//! categories are drawn.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{
    SVAggregateErrorInterval, SVAggregateErrorKind, SVAggregateOptions, SVStat, SVStatInput,
    SVStatResult,
};

use super::configuration::SVPointPlotConfiguration;
use super::data_contract::SVPointData;
use super::ordering::SVPointOrdering;

/// The aggregate of one category and one series.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVAggregateCell {
    pub(crate) value: f64,
    pub(crate) error: Option<SVAggregateErrorInterval>,
    pub(crate) number_of_observations: usize,
}

/// The categories and series of one operation, in first-appearance order.
pub(crate) struct SVAxes {
    pub(crate) categories: Vec<f64>,
    pub(crate) series: Vec<Option<f64>>,
}

/// Collects the category and series codes the observations carry.
pub(crate) fn sv_axes(data: &SVPointData) -> SVAxes {
    let mut categories: Vec<f64> = Vec::new();
    let mut series: Vec<Option<f64>> = Vec::new();
    for observation in &data.observations {
        if !categories
            .iter()
            .any(|candidate| candidate.to_bits() == observation.category.to_bits())
        {
            categories.push(observation.category);
        }
        if !series
            .iter()
            .any(|candidate| sv_same_series(*candidate, observation.series))
        {
            series.push(observation.series);
        }
    }
    SVAxes { categories, series }
}

/// Aggregates every category-and-series cell into one estimate.
pub(crate) fn sv_aggregate(
    data: &SVPointData,
    axes: &SVAxes,
    configuration: &SVPointPlotConfiguration<'_>,
) -> Result<Vec<Option<SVAggregateCell>>, SVError> {
    let category_count = axes.categories.len().max(1) as f64;
    let mut values = Vec::with_capacity(data.observations.len());
    let mut codes = Vec::with_capacity(data.observations.len());
    for observation in &data.observations {
        let category_index = sv_index_of_category(&axes.categories, observation.category);
        let series_index = sv_index_of_series(&axes.series, observation.series);
        values.push(observation.value);
        codes.push(category_index as f64 + series_index as f64 * category_count);
    }

    let options = SVAggregateOptions::new()
        .method(configuration.estimator)
        .error(sv_error_kind(configuration));
    let input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_groups(SVColumnView::Dense(&codes));
    let result = SVStat::aggregate(options).apply(input)?;
    let SVStatResult::Groups(result) = result else {
        return Err(super::configuration::sv_invalid(
            "the point plot needs the group result of the aggregate stat",
        ));
    };

    let mut cells = vec![None; axes.categories.len() * axes.series.len()];
    for group in result.groups() {
        let Some(code) = group.group() else {
            continue;
        };
        let code = code.round() as usize;
        let category_index = code % axes.categories.len().max(1);
        let series_index = code / axes.categories.len().max(1);
        if let Some(cell) = cells.get_mut(category_index + series_index * axes.categories.len()) {
            *cell = Some(SVAggregateCell {
                value: group.value(),
                error: group.error(),
                number_of_observations: group.number_of_observations(),
            });
        }
    }
    Ok(cells)
}

fn sv_error_kind(configuration: &SVPointPlotConfiguration<'_>) -> SVAggregateErrorKind {
    match configuration.error {
        SVAggregateErrorKind::Percentile(_) => {
            SVAggregateErrorKind::Percentile(configuration.confidence_level)
        }
        other => other,
    }
}

/// Orders the category indices according to the requested ordering.
pub(crate) fn sv_category_order(
    axes: &SVAxes,
    aggregates: &[Option<SVAggregateCell>],
    ordering: SVPointOrdering,
) -> Vec<usize> {
    let mut order: Vec<usize> = (0..axes.categories.len()).collect();
    let estimates: Vec<f64> = (0..axes.categories.len())
        .map(|category_index| {
            let values: Vec<f64> = (0..axes.series.len())
                .filter_map(|series_index| {
                    aggregates[category_index + series_index * axes.categories.len()]
                        .map(|cell| cell.value)
                })
                .collect();
            if values.is_empty() {
                0.0
            } else {
                values.iter().sum::<f64>() / values.len() as f64
            }
        })
        .collect();
    match ordering {
        SVPointOrdering::Data => {}
        SVPointOrdering::Ascending => {
            order.sort_by(|first, second| estimates[*first].total_cmp(&estimates[*second]));
        }
        SVPointOrdering::Descending => {
            order.sort_by(|first, second| estimates[*second].total_cmp(&estimates[*first]));
        }
    }
    order
}

pub(crate) fn sv_index_of_category(categories: &[f64], category: f64) -> usize {
    categories
        .iter()
        .position(|candidate| candidate.to_bits() == category.to_bits())
        .unwrap_or(categories.len())
}

pub(crate) fn sv_index_of_series(series: &[Option<f64>], candidate: Option<f64>) -> usize {
    series
        .iter()
        .position(|value| sv_same_series(*value, candidate))
        .unwrap_or(series.len())
}

pub(crate) fn sv_same_series(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => first.to_bits() == second.to_bits(),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}
