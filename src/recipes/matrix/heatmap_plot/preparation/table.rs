//! Pivoting a long table into the prepared matrix.

use super::SVPreparedCell;
use super::SVPreparedHeatmap;
use super::axis_keys::SVAxisKeys;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_stat::{SVAggregateOptions, SVStat, SVStatInput, SVStatResult};
use std::collections::BTreeMap;

pub(super) fn sv_prepare_table(
    data_source: &dyn crate::sv_data::SVDataSource,
    row_name: &str,
    column_name: &str,
    value_name: &str,
) -> Result<SVPreparedHeatmap, SVError> {
    let mut row_snapshot = data_source.column(row_name)?;
    let mut column_snapshot = data_source.column(column_name)?;
    let mut value_snapshot = data_source.column(value_name)?;

    let row_view = row_snapshot.column_view();
    let column_view = column_snapshot.column_view();
    let value_view = value_snapshot.column_view();

    let length = row_view.length();
    sv_check_length(length, column_name, column_view)?;
    sv_check_length(length, value_name, value_view)?;

    let mut row_axis = SVAxisKeys::new();
    let mut column_axis = SVAxisKeys::new();
    let mut row_indices = Vec::with_capacity(length);
    let mut column_indices = Vec::with_capacity(length);
    for index in 0..length {
        let row_key = sv_key(row_view.value_at(index), row_name)?;
        let column_key = sv_key(column_view.value_at(index), column_name)?;
        row_indices.push(row_axis.index_of(row_key));
        column_indices.push(column_axis.index_of(column_key));
    }

    let observations: Vec<(usize, f64)> = value_view.dispatch(
        |values| values.iter().copied().enumerate().collect(),
        |sparse| sparse.stored_entries().collect(),
        |streaming| streaming.values().iter().copied().enumerate().collect(),
    );
    let mut groups: BTreeMap<(usize, usize), Vec<f64>> = BTreeMap::new();
    for (index, value) in observations {
        if let (Some(&row), Some(&column)) = (row_indices.get(index), column_indices.get(index)) {
            groups.entry((row, column)).or_default().push(value);
        }
    }

    let mut cells = Vec::with_capacity(groups.len());
    for ((row, column), values) in groups {
        cells.push(SVPreparedCell {
            row,
            column,
            value: sv_aggregate(&values)?,
        });
    }

    Ok(SVPreparedHeatmap {
        row_labels: row_axis.into_labels(),
        column_labels: column_axis.into_labels(),
        cells,
        representation: value_view.representation(),
    })
}

/// Aggregates duplicate cell observations with the aggregate stat kernel.
fn sv_aggregate(values: &[f64]) -> Result<f64, SVError> {
    if values.is_empty() {
        return Ok(f64::NAN);
    }
    if values.len() == 1 {
        return Ok(values[0]);
    }
    let stat = SVStat::aggregate(SVAggregateOptions::new());
    match stat.apply(SVStatInput::new(SVColumnView::Dense(values)))? {
        SVStatResult::Groups(result) => result
            .groups()
            .first()
            .map(|group| group.value())
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: "the heatmap pivot produced no aggregate group".to_string(),
            }),
        _ => Err(SVError::InvalidConfiguration {
            message: "the heatmap pivot requires the aggregate stat".to_string(),
        }),
    }
}

fn sv_key(value: Option<f64>, axis: &str) -> Result<f64, SVError> {
    let value = value.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!("the heatmap `{axis}` column is shorter than its logical length"),
    })?;
    if !value.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: format!("the heatmap `{axis}` key must be finite, received {value}"),
        });
    }
    Ok(if value == 0.0 { 0.0 } else { value })
}

fn sv_check_length(
    first_length: usize,
    second_name: &str,
    second: SVColumnView<'_>,
) -> Result<(), SVError> {
    if second.length() != first_length {
        return Err(SVError::MismatchedLengths {
            name: second_name.to_string(),
            expected: first_length,
            actual: second.length(),
        });
    }
    Ok(())
}
