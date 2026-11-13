//! Aggregating each category (and series) and placing the bars.
//!
//! The estimator and the interval come from
//! [`SVStat::Aggregate`](scienceview_core::sv_stat::SVStat::Aggregate); the
//! grouping layout comes from [`SVMove`](scienceview_core::sv_move::SVMove).
//! The recipe only decides which move to apply and how to label the result.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};
use scienceview_core::sv_stat::{
    SVAggregateErrorInterval, SVAggregateErrorKind, SVAggregateOptions, SVStat, SVStatInput,
    SVStatResult,
};

use super::arrangement::SVBarArrangement;
use super::configuration::SVBarPlotConfiguration;
use super::data_contract::SVBarData;
use super::ordering::SVBarOrdering;

/// One drawn bar: an aggregate value, its interval and its placed slot.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBarBin {
    category: f64,
    series: Option<f64>,
    category_index: usize,
    series_index: usize,
    value: f64,
    error: Option<SVAggregateErrorInterval>,
    number_of_observations: usize,
    position: f64,
    width: f64,
    baseline: f64,
    top: f64,
}

impl SVBarBin {
    /// The category code this bar belongs to.
    pub fn category(&self) -> f64 {
        self.category
    }

    /// The series code this bar belongs to, when a series column was supplied.
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

    /// The aggregate value the bar encodes, before any stacking.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// The uncertainty interval the error bar draws, when one was requested.
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

    /// The bar width as a fraction of the category slot.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// The data-space value the bar grows from (zero when not stacked).
    pub fn baseline(&self) -> f64 {
        self.baseline
    }

    /// The data-space value the bar reaches after stacking.
    pub fn top(&self) -> f64 {
        self.top
    }
}

/// The categories, the series and the bars of one operation.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBarSummary {
    categories: Vec<f64>,
    series: Vec<Option<f64>>,
    bins: Vec<SVBarBin>,
}

impl SVBarSummary {
    /// The category codes in draw order.
    pub fn categories(&self) -> &[f64] {
        &self.categories
    }

    /// The series codes in legend order.
    pub fn series(&self) -> &[Option<f64>] {
        &self.series
    }

    /// The drawn bars, one per category and series.
    pub fn bins(&self) -> &[SVBarBin] {
        &self.bins
    }
}

/// Aggregates the observations and places the bars.
pub(crate) fn sv_summarize(
    data: &SVBarData,
    configuration: &SVBarPlotConfiguration<'_>,
) -> Result<SVBarSummary, SVError> {
    let (categories, series) = sv_axes(data);
    let aggregates = sv_aggregate(data, &categories, &series, configuration)?;
    let category_order =
        sv_category_order(&categories, &series, &aggregates, configuration.ordering);
    sv_place_bars(
        &categories,
        &series,
        &aggregates,
        &category_order,
        configuration,
    )
}

fn sv_axes(data: &SVBarData) -> (Vec<f64>, Vec<Option<f64>>) {
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
    (categories, series)
}

#[derive(Clone, Copy)]
struct SVAggregateCell {
    value: f64,
    error: Option<SVAggregateErrorInterval>,
    number_of_observations: usize,
}

fn sv_aggregate(
    data: &SVBarData,
    categories: &[f64],
    series: &[Option<f64>],
    configuration: &SVBarPlotConfiguration<'_>,
) -> Result<Vec<Option<SVAggregateCell>>, SVError> {
    let category_count = categories.len().max(1) as f64;
    let mut values = Vec::with_capacity(data.observations.len());
    let mut codes = Vec::with_capacity(data.observations.len());
    let mut weights = configuration.weight.map(|_| Vec::new());
    for observation in &data.observations {
        let category_index = sv_index_of_category(categories, observation.category);
        let series_index = sv_index_of_series(series, observation.series);
        values.push(observation.value);
        codes.push(category_index as f64 + series_index as f64 * category_count);
        if let (Some(weights), Some(weight)) = (weights.as_mut(), observation.weight) {
            weights.push(weight);
        }
    }

    let options = SVAggregateOptions::new().method(configuration.estimator);
    let options = match sv_error_kind(configuration) {
        Some(error) => options.error(error),
        None => options,
    };
    let mut input =
        SVStatInput::new(SVColumnView::Dense(&values)).with_groups(SVColumnView::Dense(&codes));
    if let Some(weights) = &weights {
        input = input.with_weights(SVColumnView::Dense(weights));
    }
    let result = SVStat::aggregate(options).apply(input)?;
    let SVStatResult::Groups(result) = result else {
        return Err(super::configuration::sv_invalid(
            "the bar plot needs the group result of the aggregate stat",
        ));
    };

    let mut cells = vec![None; categories.len() * series.len()];
    for group in result.groups() {
        let Some(code) = group.group() else {
            continue;
        };
        let code = code.round() as usize;
        let category_index = code % categories.len().max(1);
        let series_index = code / categories.len().max(1);
        if let Some(cell) = cells.get_mut(category_index + series_index * categories.len()) {
            *cell = Some(SVAggregateCell {
                value: group.value(),
                error: group.error(),
                number_of_observations: group.number_of_observations(),
            });
        }
    }
    Ok(cells)
}

fn sv_error_kind(configuration: &SVBarPlotConfiguration<'_>) -> Option<SVAggregateErrorKind> {
    match configuration.error {
        Some(SVAggregateErrorKind::Percentile(_)) => Some(SVAggregateErrorKind::Percentile(
            configuration.confidence_level,
        )),
        other => other,
    }
}

fn sv_category_order(
    categories: &[f64],
    series: &[Option<f64>],
    aggregates: &[Option<SVAggregateCell>],
    ordering: SVBarOrdering,
) -> Vec<usize> {
    let mut order: Vec<usize> = (0..categories.len()).collect();
    let totals: Vec<f64> = (0..categories.len())
        .map(|category_index| {
            (0..series.len())
                .filter_map(|series_index| {
                    aggregates[category_index + series_index * categories.len()]
                        .map(|cell| cell.value)
                })
                .sum()
        })
        .collect();
    match ordering {
        SVBarOrdering::Data => {}
        SVBarOrdering::Ascending => {
            order.sort_by(|first, second| totals[*first].total_cmp(&totals[*second]));
        }
        SVBarOrdering::Descending => {
            order.sort_by(|first, second| totals[*second].total_cmp(&totals[*first]));
        }
    }
    order
}

#[allow(clippy::too_many_arguments)]
fn sv_place_bars(
    categories: &[f64],
    series: &[Option<f64>],
    aggregates: &[Option<SVAggregateCell>],
    category_order: &[usize],
    configuration: &SVBarPlotConfiguration<'_>,
) -> Result<SVBarSummary, SVError> {
    let category_count = categories.len();
    let series_count = series.len();
    let mut rows: Vec<(f64, usize, usize)> = Vec::with_capacity(category_count * series_count);
    for (ordered_index, category_index) in category_order.iter().copied().enumerate() {
        for series_index in 0..series_count {
            if let Some(cell) = aggregates[category_index + series_index * category_count] {
                rows.push((cell.value, ordered_index, series_index));
            }
        }
    }
    if rows.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "bar plot",
        });
    }

    let values: Vec<f64> = rows.iter().map(|row| row.0).collect();
    let group_keys: Vec<u32> = match configuration.arrangement {
        SVBarArrangement::Dodge => rows.iter().map(|row| row.2 as u32).collect(),
        SVBarArrangement::Stack => rows.iter().map(|row| row.1 as u32).collect(),
    };
    let category_keys: Vec<u32> = rows.iter().map(|row| row.1 as u32).collect();
    let move_option = match configuration.arrangement {
        SVBarArrangement::Dodge => SVMove::dodge().width(configuration.width).into(),
        SVBarArrangement::Stack => SVMove::stack().into(),
    };
    let sequence = SVMoveSequence::new(vec![move_option])?;
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &group_keys)
        .with_category_keys(&category_keys);
    let moved = sequence.apply(&request)?;

    let mut bins = Vec::with_capacity(rows.len());
    for (index, (value, ordered_index, series_index)) in rows.iter().copied().enumerate() {
        let original_category = category_order[ordered_index];
        let cell = aggregates[original_category + series_index * category_count].ok_or(
            SVError::EmptyInput {
                operation: "bar plot",
            },
        )?;
        let (position, width, baseline, top) = match configuration.arrangement {
            SVBarArrangement::Dodge => (
                ordered_index as f64 + moved.horizontal_offset(index).unwrap_or(0.0),
                moved
                    .width_at(index)
                    .unwrap_or(f64::from(configuration.width)),
                0.0,
                value,
            ),
            SVBarArrangement::Stack => (
                ordered_index as f64,
                f64::from(configuration.width),
                moved.baseline_at(index),
                moved.value_at(index).unwrap_or(value),
            ),
        };
        bins.push(SVBarBin {
            category: categories[original_category],
            series: series[series_index],
            category_index: ordered_index,
            series_index,
            value: cell.value,
            error: cell.error,
            number_of_observations: cell.number_of_observations,
            position,
            width,
            baseline,
            top,
        });
    }

    let ordered_categories = category_order
        .iter()
        .map(|index| categories[*index])
        .collect();
    Ok(SVBarSummary {
        categories: ordered_categories,
        series: series.to_vec(),
        bins,
    })
}

fn sv_index_of_category(categories: &[f64], category: f64) -> usize {
    categories
        .iter()
        .position(|candidate| candidate.to_bits() == category.to_bits())
        .unwrap_or(categories.len())
}

fn sv_index_of_series(series: &[Option<f64>], candidate: Option<f64>) -> usize {
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
