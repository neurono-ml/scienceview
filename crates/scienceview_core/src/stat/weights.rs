//! Observation weights, resolved once and read without densifying.
//!
//! A weighted kernel validates its weight column exactly once and then folds it
//! into the same `O(stored)` walk the unweighted kernel takes: the stored
//! observations are visited with their weights, and the implicit zeros of a
//! sparse column are visited as one run whose weight is the sum of the weights
//! over that run. A sparse weight column stays sparse — a position without a
//! stored weight carries zero weight, so it never needs materialising.

use crate::sv_error::SVError;
use crate::sv_stat::numeric::SVCompensatedSum;
use crate::sv_stat::observations::SVObservationValues;

/// Validates a weight column against the value column it weights.
///
/// The length is checked first, so a caller never reads a weight that does not
/// belong to its observation. A non-finite weight and a negative weight are
/// both rejected at their logical position, naming the operation.
pub(crate) fn sv_validate_weights(
    values: &SVObservationValues<'_>,
    weights: &SVObservationValues<'_>,
    operation: &'static str,
) -> Result<(), SVError> {
    if values.length() != weights.length() {
        return Err(SVError::MismatchedStatisticLengths {
            operation,
            first: values.length(),
            second: weights.length(),
        });
    }

    let mut failure = None;
    weights.for_each_stored_entry(|position, weight| {
        if failure.is_none() {
            if !weight.is_finite() {
                failure = Some(SVError::NonFiniteValue {
                    operation,
                    position,
                });
            } else if weight < 0.0 {
                failure = Some(SVError::NegativeWeight {
                    operation,
                    position,
                });
            }
        }
    });

    match failure {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

/// Returns the total weight the observations carry.
///
/// Without a weight column every observation weighs one, so the logical length
/// is the total — a sparse column's implicit zeros included.
pub(crate) fn sv_total_weight(
    values: &SVObservationValues<'_>,
    weights: Option<&SVObservationValues<'_>>,
) -> f64 {
    match weights {
        None => values.length() as f64,
        Some(weights) => {
            let mut total = SVCompensatedSum::new();
            weights.for_each_stored(|weight| total.add(weight));
            total.value()
        }
    }
}

/// Visits every observation with the weight it contributes.
///
/// Contiguous weights are read by logical position; the implicit zeros of a
/// sparse value column are visited once per maximal run, weighted by the sum of
/// the weights over that run. Sparse weights are read from their stored
/// entries, and a position without a stored weight contributes nothing.
pub(crate) fn sv_for_each_weighted_observation(
    values: &SVObservationValues<'_>,
    weights: Option<&SVObservationValues<'_>>,
    mut visit: impl FnMut(f64, f64),
) {
    match weights {
        None => {
            values.for_each_stored(|value| visit(value, 1.0));
            let implicit_zeros = values.implicit_zero_count() as f64;
            if implicit_zeros > 0.0 {
                visit(0.0, implicit_zeros);
            }
        }
        Some(SVObservationValues::Contiguous(weights)) => {
            values.for_each_stored_entry(|position, value| {
                let weight = weights.get(position).copied().unwrap_or(0.0);
                visit(value, weight);
            });
            values.for_each_implicit_zero_run(|start, run_length| {
                let run = weights.get(start..start + run_length).unwrap_or_default();
                let mut total = SVCompensatedSum::new();
                run.iter().copied().for_each(|weight| total.add(weight));
                let total = total.value();
                if total > 0.0 {
                    visit(0.0, total);
                }
            });
        }
        Some(SVObservationValues::Stored {
            indices,
            values: weight_values,
            ..
        }) => {
            indices
                .iter()
                .copied()
                .zip(weight_values.iter().copied())
                .for_each(|(position, weight)| {
                    let value = values.value_at_position(position).unwrap_or(0.0);
                    visit(value, weight);
                });
        }
    }
}
