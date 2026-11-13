//! Placing the points inside their category slots.
//!
//! When a series column splits a category, the series are dodged through
//! [`SVMove::Dodge`](scienceview_core::sv_move::SVMove::dodge): the recipe asks
//! the move kernel for the horizontal offset and never computes it itself.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

use super::aggregation::{SVAggregateCell, SVAxes};
use super::configuration::SVPointPlotConfiguration;
use super::statistics::{SVPointEstimate, SVPointSummary};

/// The fraction of the category slot a dodged series spreads over.
pub(crate) const SV_POINT_DODGE_WIDTH: f32 = 0.8;

/// Arranges the aggregated cells into drawn point estimates.
pub(crate) fn sv_place_estimates(
    axes: &SVAxes,
    aggregates: &[Option<SVAggregateCell>],
    category_order: &[usize],
    _configuration: &SVPointPlotConfiguration<'_>,
) -> Result<SVPointSummary, SVError> {
    let category_count = axes.categories.len();
    let series_count = axes.series.len();
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
            operation: "point plot",
        });
    }

    let values: Vec<f64> = rows.iter().map(|row| row.0).collect();
    let group_keys: Vec<u32> = rows.iter().map(|row| row.2 as u32).collect();
    let category_keys: Vec<u32> = rows.iter().map(|row| row.1 as u32).collect();
    let move_option = SVMove::dodge().width(SV_POINT_DODGE_WIDTH).into();
    let sequence = SVMoveSequence::new(vec![move_option])?;
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &group_keys)
        .with_category_keys(&category_keys);
    let moved = sequence.apply(&request)?;

    let mut estimates = Vec::with_capacity(rows.len());
    for (index, (_, ordered_index, series_index)) in rows.iter().copied().enumerate() {
        let original_category = category_order[ordered_index];
        let cell = aggregates[original_category + series_index * category_count].ok_or(
            SVError::EmptyInput {
                operation: "point plot",
            },
        )?;
        estimates.push(SVPointEstimate {
            category: axes.categories[original_category],
            series: axes.series[series_index],
            category_index: ordered_index,
            series_index,
            value: cell.value,
            error: cell.error,
            number_of_observations: cell.number_of_observations,
            position: ordered_index as f64 + moved.horizontal_offset(index).unwrap_or(0.0),
        });
    }

    let ordered_categories = category_order
        .iter()
        .map(|index| axes.categories[*index])
        .collect();
    Ok(SVPointSummary::new(
        ordered_categories,
        axes.series.clone(),
        estimates,
    ))
}
