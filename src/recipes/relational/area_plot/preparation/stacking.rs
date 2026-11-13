//! Stacking the prepared series through the move kernel.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

use super::super::configuration::SVAreaPlotConfiguration;
use super::column_access::SVColumnAccess;
use super::prepared_area::{SVPreparedSegment, SVPreparedSeries};

/// Stacks every series over the reduced horizontal grid with
/// [`SVMove::Stack`], then writes one cumulative-baseline segment per series.
pub(crate) fn sv_stack_series(
    configuration: &SVAreaPlotConfiguration<'_>,
    groups: &[Vec<usize>],
    indices: &[usize],
    x: &SVColumnAccess<'_>,
    y: &SVColumnAccess<'_>,
    palette: &scienceview_core::sv_scale::SVScalePalette,
) -> Result<Vec<SVPreparedSeries>, SVError> {
    // The stack aligns every series on a shared grid, so the missing-value
    // policy is applied here too: `Omit` drops the sample from the grid and
    // `Error` rejects the render before any alignment happens.
    for index in indices.iter().copied() {
        if !x.value_at(index).is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.x, index)?;
        } else if !y.value_at(index).is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.y, index)?;
        }
    }

    let mut grid: Vec<f64> = indices
        .iter()
        .map(|index| x.value_at(*index))
        .filter(|value| value.is_finite())
        .collect();
    grid.sort_by(f64::total_cmp);
    grid.dedup_by(|left, right| *left == *right);

    if grid.is_empty() || grid.len() > u32::MAX as usize {
        return Ok(Vec::new());
    }

    let ordered: Vec<Vec<(f64, f64)>> = groups
        .iter()
        .map(|group| {
            let mut points: Vec<(f64, f64)> = group
                .iter()
                .filter_map(|index| {
                    let x_value = x.value_at(*index);
                    let y_value = y.value_at(*index);
                    (x_value.is_finite() && y_value.is_finite()).then_some((x_value, y_value))
                })
                .collect();
            points.sort_by(|left, right| left.0.total_cmp(&right.0));
            points
        })
        .collect();

    let number_of_series = groups.len();
    let mut values = Vec::with_capacity(grid.len() * number_of_series);
    let mut group_keys = Vec::with_capacity(grid.len() * number_of_series);
    for (x_index, x_value) in grid.iter().copied().enumerate() {
        for points in &ordered {
            values.push(sv_value_at(points, x_value));
            group_keys.push(x_index as u32);
        }
    }

    let sequence = SVMoveSequence::single(SVMove::stack().into())?;
    let moved = sequence.apply(&SVMoveRequest::new(
        SVColumnView::Dense(&values),
        &group_keys,
    ))?;

    let mut stacked = Vec::new();
    for (position, points) in ordered.iter().enumerate() {
        if points.is_empty() {
            continue;
        }

        let mut top = Vec::with_capacity(grid.len());
        let mut baseline = Vec::with_capacity(grid.len());
        for x_index in 0..grid.len() {
            let index = x_index * number_of_series + position;
            top.push(moved.value_at(index).unwrap_or(0.0));
            baseline.push(moved.baseline_at(index));
        }
        stacked.push(SVPreparedSeries {
            colour: palette.colour_at_index(position),
            segments: vec![SVPreparedSegment {
                x: grid.clone(),
                top,
                baseline: Some(baseline),
            }],
        });
    }

    Ok(stacked)
}

/// Returns the value of one series at one horizontal position, or its missing
/// contribution of zero.
fn sv_value_at(points: &[(f64, f64)], x: f64) -> f64 {
    match points.binary_search_by(|(point_x, _)| point_x.total_cmp(&x)) {
        Ok(position) => points[position].1,
        Err(_) => 0.0,
    }
}
