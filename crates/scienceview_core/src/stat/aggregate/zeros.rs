//! Folding the implicit zeros of a sparse value column.

use crate::sv_stat::numeric::SVCompensatedSum;
use crate::sv_stat::observations::SVObservationValues;

use super::buckets::SVBuckets;

/// Folds the implicit zeros of the value column into the matching buckets.
///
/// Without a grouping column the zeros are one multiplicity on the single
/// bucket, which keeps the fold `O(stored)`. With a grouping column each zero
/// carries the group code at its position: a sparse group column answers that
/// code from its stored entries plus the implicit zero code, and a contiguous
/// one is read position by position.
pub(super) fn sv_fold_implicit_zeros(
    values: &SVObservationValues<'_>,
    groups: Option<&SVObservationValues<'_>>,
    weights: Option<&SVObservationValues<'_>>,
    buckets: &mut SVBuckets,
) {
    let total = values.implicit_zero_count();
    if total == 0 {
        return;
    }

    let Some(groups) = groups else {
        let weight_total = weights.map(|weights| sv_sum_zero_weights(values, weights));
        buckets.add_implicit_zeros(None, total, weight_total);
        return;
    };

    match groups {
        SVObservationValues::Contiguous(codes) => {
            values.for_each_implicit_zero_run(|start, run_length| {
                for position in start..start + run_length {
                    let weight =
                        weights.map(|weights| weights.value_at_position(position).unwrap_or(0.0));
                    buckets.add_implicit_zeros(Some(codes[position]), 1, weight);
                }
            });
        }
        SVObservationValues::Stored { .. } => {
            let mut assigned = 0usize;
            groups.for_each_stored_entry(|position, code| {
                if position < values.length() && values.value_at_position(position).is_none() {
                    let weight =
                        weights.map(|weights| weights.value_at_position(position).unwrap_or(0.0));
                    buckets.add_implicit_zeros(Some(code), 1, weight);
                    assigned += 1;
                }
            });

            let remaining = total.saturating_sub(assigned);
            if remaining > 0 {
                let weight_total =
                    weights.map(|weights| sv_sum_unassigned_zero_weights(values, groups, weights));
                buckets.add_implicit_zeros(Some(0.0), remaining, weight_total);
            }
        }
    }
}

/// Sums the weights at the implicit zeros of the value column.
fn sv_sum_zero_weights(values: &SVObservationValues<'_>, weights: &SVObservationValues<'_>) -> f64 {
    let mut total = SVCompensatedSum::new();
    match weights {
        SVObservationValues::Contiguous(weights) => {
            values.for_each_implicit_zero_run(|start, run_length| {
                weights[start..start + run_length]
                    .iter()
                    .copied()
                    .for_each(|weight| total.add(weight));
            });
        }
        SVObservationValues::Stored { .. } => {
            weights.for_each_stored_entry(|position, weight| {
                if position < values.length() && values.value_at_position(position).is_none() {
                    total.add(weight);
                }
            });
        }
    }

    total.value()
}

/// Sums the weights at the zeros whose group code is the sparse implicit zero.
fn sv_sum_unassigned_zero_weights(
    values: &SVObservationValues<'_>,
    groups: &SVObservationValues<'_>,
    weights: &SVObservationValues<'_>,
) -> f64 {
    let mut total = SVCompensatedSum::new();
    match weights {
        SVObservationValues::Contiguous(weights) => {
            values.for_each_implicit_zero_run(|start, run_length| {
                for position in start..start + run_length {
                    if groups.value_at_position(position).is_none() {
                        total.add(weights[position]);
                    }
                }
            });
        }
        SVObservationValues::Stored { .. } => {
            weights.for_each_stored_entry(|position, weight| {
                if position < values.length()
                    && values.value_at_position(position).is_none()
                    && groups.value_at_position(position).is_none()
                {
                    total.add(weight);
                }
            });
        }
    }

    total.value()
}
