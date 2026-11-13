//! Choosing the logical positions to read, including sparse compaction.

use super::super::options::SVStepDuplicatePolicy;
use super::cursor::SVStepCursor;
use scienceview_core::sv_data::SVSparseView;

/// Chooses the logical positions to read.
///
/// A sparse ordering column stores its non-zero `x` values explicitly; every
/// other position is an implicit zero. Because all those positions share
/// `x = 0`, the duplicate-`x` policy collapses them to one observation per
/// series, so a single-series reader visits the stored entries plus that one
/// representative position instead of expanding the column. Aggregation and a
/// `series` column need every observation (the group statistic and the per
/// series winner are not derivable from one representative), so they fall back
/// to the full logical length.
pub(super) fn sv_sample_indices(
    x_cursor: &SVStepCursor<'_>,
    y_cursor: &SVStepCursor<'_>,
    length: usize,
    policy: SVStepDuplicatePolicy,
    aggregate: bool,
    has_series: bool,
) -> Vec<usize> {
    let Some(sparse) = x_cursor.sparse() else {
        return (0..length).collect();
    };
    if aggregate || has_series {
        return (0..length).collect();
    }

    let mut indices: Vec<usize> = sparse.stored_entries().map(|(index, _)| index).collect();
    if let Some(representative) =
        sv_finite_zero_representative(x_cursor, y_cursor, sparse, length, policy)
    {
        indices.push(representative);
    }
    indices.sort_unstable();
    indices.dedup();
    indices
}

/// Returns the winning implicit-zero position whose `y` is finite.
fn sv_finite_zero_representative(
    x_cursor: &SVStepCursor<'_>,
    y_cursor: &SVStepCursor<'_>,
    sparse: &SVSparseView<'_>,
    length: usize,
    policy: SVStepDuplicatePolicy,
) -> Option<usize> {
    match policy {
        SVStepDuplicatePolicy::FirstWins => {
            let mut position = sv_first_zero_position(sparse)?;
            loop {
                if x_cursor.value_at(position) != Some(0.0) {
                    return None;
                }
                if y_cursor.value_at(position).is_some_and(f64::is_finite) {
                    return Some(position);
                }
                position += 1;
                if position >= length {
                    return None;
                }
            }
        }
        SVStepDuplicatePolicy::LastWins => {
            let mut position = sv_last_zero_position(sparse, length)?;
            loop {
                if x_cursor.value_at(position) != Some(0.0) {
                    return None;
                }
                if y_cursor.value_at(position).is_some_and(f64::is_finite) {
                    return Some(position);
                }
                if position == 0 {
                    return None;
                }
                position -= 1;
            }
        }
    }
}

fn sv_first_zero_position(sparse: &SVSparseView<'_>) -> Option<usize> {
    let mut position = 0;
    for (index, value) in sparse.stored_entries() {
        if index > position || value == 0.0 {
            return Some(position);
        }
        position = index + 1;
    }
    if position < sparse.length() {
        Some(position)
    } else {
        None
    }
}

fn sv_last_zero_position(sparse: &SVSparseView<'_>, length: usize) -> Option<usize> {
    let mut last_zero = None;
    let mut position = 0;
    for (index, value) in sparse.stored_entries() {
        if index > position {
            last_zero = Some(index - 1);
        }
        if value == 0.0 {
            last_zero = Some(index);
        }
        position = index + 1;
    }
    if position < length {
        last_zero = Some(length - 1);
    }
    last_zero
}
