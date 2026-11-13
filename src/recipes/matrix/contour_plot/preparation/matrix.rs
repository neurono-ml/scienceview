//! Folding a row-major matrix column onto a regular lattice.

use super::super::configuration::SVContourConfiguration;
use super::super::field::SVContourField;
use super::missing::sv_fill_missing;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

/// Builds a lattice from a value column read as `rows` by `columns` samples.
///
/// The sparse representation answers implicit positions with zero and is never
/// materialised, so a sparse matrix keeps its unoccupied cells at zero.
pub(super) fn sv_matrix_field(
    configuration: &SVContourConfiguration<'_>,
    value: SVColumnView<'_>,
    rows: usize,
    columns: usize,
) -> Result<SVContourField, SVError> {
    let expected = rows * columns;
    if value.length() != expected {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "a {rows} by {columns} contour matrix needs {expected} values, got {}",
                value.length()
            ),
        });
    }
    let x_coordinates = sv_axis(columns, configuration.x_range);
    let y_coordinates = sv_axis(rows, configuration.y_range);
    let samples: Vec<f64> = (0..expected)
        .map(|position| value.value_at(position).unwrap_or(f64::NAN))
        .collect();
    let values = sv_fill_missing(
        &samples,
        columns,
        rows,
        configuration.missing_policy,
        "contour_field",
    )?;
    Ok(SVContourField::new(x_coordinates, y_coordinates, values))
}

fn sv_axis(length: usize, range: Option<(f64, f64)>) -> Vec<f64> {
    let (lower, upper) = range.unwrap_or((0.0, length.saturating_sub(1) as f64));
    let denominator = length.saturating_sub(1).max(1) as f64;
    (0..length)
        .map(|index| lower + (upper - lower) * index as f64 / denominator)
        .collect()
}
