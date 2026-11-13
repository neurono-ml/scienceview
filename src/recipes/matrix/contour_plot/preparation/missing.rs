//! Applying the missing-value policy to a sampled lattice.

use super::super::options::SVContourMissingPolicy;
use crate::sv_error::SVError;

/// Replaces or rejects every non-finite sample of a row-major lattice.
///
/// `Error` reports the first offending position. `Omit` replaces a gap with
/// the mean of its finite orthogonal neighbours, then with the mean of every
/// finite sample, and finally with zero when the lattice has no finite sample
/// at all, so the marching square always sees a complete lattice.
pub(super) fn sv_fill_missing(
    samples: &[f64],
    columns: usize,
    rows: usize,
    policy: SVContourMissingPolicy,
    operation: &'static str,
) -> Result<Vec<f64>, SVError> {
    if let Some(position) = samples.iter().position(|value| !value.is_finite()) {
        if policy == SVContourMissingPolicy::Error {
            return Err(SVError::NonFiniteValue {
                operation,
                position,
            });
        }
    }
    let mut filled = samples.to_vec();
    let finite_mean = sv_finite_mean(samples);
    for row in 0..rows {
        for column in 0..columns {
            let position = row * columns + column;
            if samples.get(position).is_some_and(|value| value.is_finite()) {
                continue;
            }
            filled[position] =
                sv_neighbour_mean(samples, columns, rows, column, row).unwrap_or(finite_mean);
        }
    }
    Ok(filled)
}

fn sv_finite_mean(samples: &[f64]) -> f64 {
    let (sum, count) = samples
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold((0.0, 0usize), |(sum, count), value| {
            (sum + value, count + 1)
        });
    if count == 0 { 0.0 } else { sum / count as f64 }
}

fn sv_neighbour_mean(
    samples: &[f64],
    columns: usize,
    rows: usize,
    column: usize,
    row: usize,
) -> Option<f64> {
    let candidates = [
        (column.wrapping_sub(1), row),
        (column + 1, row),
        (column, row.wrapping_sub(1)),
        (column, row + 1),
    ];
    let (sum, count) = candidates
        .into_iter()
        .filter(|(candidate_column, candidate_row)| {
            *candidate_column < columns && *candidate_row < rows
        })
        .filter_map(|(candidate_column, candidate_row)| {
            samples.get(candidate_row * columns + candidate_column)
        })
        .copied()
        .filter(|value| value.is_finite())
        .fold((0.0, 0usize), |(sum, count), value| {
            (sum + value, count + 1)
        });
    if count == 0 {
        None
    } else {
        Some(sum / count as f64)
    }
}
