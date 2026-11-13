//! The aggregate entry point: validation, column reading and orchestration.
//!
//! Every column resolves its representation once, through
//! [`SVObservationValues::from_view`]. The value column drives the fold: only
//! its stored entries are visited, and the implicit zeros of a sparse column
//! are folded as a multiplicity, so an ungrouped aggregate over a sparse view
//! is `O(stored)` rather than `O(logical length)`.

use crate::sv_error::SVError;
use crate::sv_stat::aggregate_options::{
    SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions,
};
use crate::sv_stat::aggregate_result::SVAggregateResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::stat_definition::SVStatResult;
use crate::sv_stat::weights::sv_validate_weights;

use super::buckets::SVBuckets;
use super::zeros::sv_fold_implicit_zeros;

/// Applies the aggregation configuration to the input columns.
///
/// Every column resolves its representation once, through
/// [`SVObservationValues::from_view`]. The value column drives the fold: only
/// its stored entries are visited, and the implicit zeros of a sparse column
/// are folded as a multiplicity, so an ungrouped aggregate over a sparse view
/// is `O(stored)` rather than `O(logical length)`.
pub(crate) fn sv_apply_aggregate(
    options: &SVAggregateOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let values = SVObservationValues::from_view(input.values());
    let groups = input.groups().map(SVObservationValues::from_view);
    let weights = input.weights().map(SVObservationValues::from_view);
    let length = values.length();
    if length == 0 {
        return Err(SVError::EmptyInput {
            operation: "aggregate",
        });
    }

    if let Some(groups) = &groups {
        if groups.length() != length {
            return Err(SVError::MismatchedStatisticLengths {
                operation: "aggregate",
                first: length,
                second: groups.length(),
            });
        }
    }
    if let Some(weights) = &weights {
        if weights.length() != length {
            return Err(SVError::MismatchedStatisticLengths {
                operation: "aggregate",
                first: length,
                second: weights.length(),
            });
        }
        if !matches!(
            options.requested_method(),
            SVAggregateMethod::Mean | SVAggregateMethod::Sum
        ) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "`{}` does not accept observation weights; only Mean and Sum do",
                    options.requested_method().as_str()
                ),
            });
        }
    }
    if let Some(SVAggregateErrorKind::Percentile(probability)) = options.requested_error() {
        if !probability.is_finite() || !(0.0..1.0).contains(&probability) {
            return Err(SVError::InvalidConfiguration {
                message: "an aggregate percentile error must lie in (0, 1)".to_string(),
            });
        }
    }

    values.validate_finite("aggregate")?;
    if let Some(groups) = &groups {
        groups.validate_finite("aggregate")?;
    }
    if let Some(weights) = &weights {
        sv_validate_weights(&values, weights, "aggregate")?;
    }

    let needs_sorted_values = options.requested_method() == SVAggregateMethod::Median
        || matches!(
            options.requested_error(),
            Some(SVAggregateErrorKind::Percentile(_))
        );

    let mut buckets = SVBuckets::new(needs_sorted_values);
    values.for_each_stored_entry(|position, value| {
        let group = groups
            .as_ref()
            .map(|groups| groups.value_at_position(position).unwrap_or(0.0));
        let weight = weights
            .as_ref()
            .map(|weights| weights.value_at_position(position).unwrap_or(0.0));
        buckets.add(group, value, weight);
    });
    sv_fold_implicit_zeros(&values, groups.as_ref(), weights.as_ref(), &mut buckets);

    Ok(SVStatResult::Groups(SVAggregateResult::new(
        buckets.finish(options)?,
    )))
}
