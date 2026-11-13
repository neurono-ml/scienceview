//! Choosing the logical positions to read, including sparse compaction.

use super::super::options::SVStepDuplicatePolicy;
use super::cursor::SVStepCursor;

/// Chooses the logical positions to read.
///
/// A sparse ordering column stores its non-zero `x` values explicitly; every
/// other position is an implicit zero. Because all those positions share
/// `x = 0`, the duplicate-`x` policy collapses them to one observation, so a
/// single-series reader visits the stored entries plus that one representative
/// position instead of expanding the column. Aggregation and a `series` column
/// need every observation (the group statistic and the per-series winner are
/// not derivable from one representative), so they fall back to the full
/// logical length.
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
    if let Some(representative) = sv_finite_zero_representative(x_cursor, y_cursor, length, policy)
    {
        indices.push(representative);
    }
    indices.sort_unstable();
    indices.dedup();
    indices
}

/// Returns the winning zero-`x` position whose `y` is finite.
///
/// The scan touches no storage beyond one index at a time; only the chosen
/// representative is kept. A non-zero stored `x` is skipped, so a zero run on
/// either side of a non-zero sample is still considered.
fn sv_finite_zero_representative(
    x_cursor: &SVStepCursor<'_>,
    y_cursor: &SVStepCursor<'_>,
    length: usize,
    policy: SVStepDuplicatePolicy,
) -> Option<usize> {
    let is_finite_zero = |index: usize| {
        x_cursor.value_at(index) == Some(0.0)
            && y_cursor.value_at(index).is_some_and(f64::is_finite)
    };
    match policy {
        SVStepDuplicatePolicy::FirstWins => (0..length).find(|index| is_finite_zero(*index)),
        SVStepDuplicatePolicy::LastWins => (0..length).rev().find(|index| is_finite_zero(*index)),
    }
}
