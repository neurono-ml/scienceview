//! The aggregation kernel.

use std::collections::HashMap;

use crate::sv_error::SVError;
use crate::sv_stat::aggregate_options::{
    SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions,
};
use crate::sv_stat::aggregate_result::{
    SVAggregateErrorInterval, SVAggregateGroup, SVAggregateResult,
};
use crate::sv_stat::count::sv_category_key;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::numeric::{SVCompensatedSum, SVWelford};
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::stat_definition::SVStatResult;

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
        weights.validate_finite("aggregate")?;
        sv_validate_non_negative(weights)?;
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

/// Rejects a negative weight wherever it is stored.
fn sv_validate_non_negative(weights: &SVObservationValues<'_>) -> Result<(), SVError> {
    let mut failure = None;
    weights.for_each_stored_entry(|position, weight| {
        if weight < 0.0 && failure.is_none() {
            failure = Some(SVError::InvalidConfiguration {
                message: format!("the observation weight at position {position} is negative"),
            });
        }
    });

    match failure {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

/// Folds the implicit zeros of the value column into the matching buckets.
///
/// Without a grouping column the zeros are one multiplicity on the single
/// bucket, which keeps the fold `O(stored)`. With a grouping column each zero
/// carries the group code at its position: a sparse group column answers that
/// code from its stored entries plus the implicit zero code, and a contiguous
/// one is read position by position.
fn sv_fold_implicit_zeros(
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

/// The group buckets of one aggregation, in first-appearance order.
struct SVBuckets {
    entries: Vec<(Option<f64>, SVGroupAccumulator)>,
    positions: HashMap<u64, usize>,
    needs_sorted_values: bool,
}

impl SVBuckets {
    fn new(needs_sorted_values: bool) -> Self {
        Self {
            entries: Vec::new(),
            positions: HashMap::new(),
            needs_sorted_values,
        }
    }

    /// Returns the bucket of a group, creating it on first appearance.
    fn accumulator(&mut self, group: Option<f64>) -> &mut SVGroupAccumulator {
        let key = group.map(sv_category_key).unwrap_or(0);
        let index = match self.positions.get(&key).copied() {
            Some(index) => index,
            None => {
                let index = self.entries.len();
                self.positions.insert(key, index);
                self.entries
                    .push((group, SVGroupAccumulator::new(self.needs_sorted_values)));
                index
            }
        };
        &mut self.entries[index].1
    }

    fn add(&mut self, group: Option<f64>, value: f64, weight: Option<f64>) {
        self.accumulator(group).add(value, weight);
    }

    fn add_implicit_zeros(&mut self, group: Option<f64>, count: usize, weight_total: Option<f64>) {
        self.accumulator(group)
            .add_implicit_zeros(count, weight_total);
    }

    fn finish(self, options: &SVAggregateOptions) -> Result<Vec<SVAggregateGroup>, SVError> {
        self.entries
            .into_iter()
            .map(|(group, accumulator)| accumulator.finish(group, options))
            .collect()
    }
}

/// Accumulates one group in a single pass.
struct SVGroupAccumulator {
    welford: SVWelford,
    sum: SVCompensatedSum,
    weighted_sum: SVCompensatedSum,
    weight_total: SVCompensatedSum,
    minimum: f64,
    maximum: f64,
    sorted_values: Option<Vec<f64>>,
    implicit_zero_count: usize,
    weighted: bool,
}

impl SVGroupAccumulator {
    fn new(needs_sorted_values: bool) -> Self {
        Self {
            welford: SVWelford::new(),
            sum: SVCompensatedSum::new(),
            weighted_sum: SVCompensatedSum::new(),
            weight_total: SVCompensatedSum::new(),
            minimum: f64::INFINITY,
            maximum: f64::NEG_INFINITY,
            sorted_values: needs_sorted_values.then(Vec::new),
            implicit_zero_count: 0,
            weighted: false,
        }
    }

    fn add(&mut self, value: f64, weight: Option<f64>) {
        self.welford.add(value);
        self.sum.add(value);
        self.minimum = self.minimum.min(value);
        self.maximum = self.maximum.max(value);

        if let Some(weight) = weight {
            self.weighted = true;
            self.weighted_sum.add(weight * value);
            self.weight_total.add(weight);
        }
        if let Some(values) = &mut self.sorted_values {
            values.push(value);
        }
    }

    /// Folds a run of implicit zeros without visiting each position.
    fn add_implicit_zeros(&mut self, count: usize, weight_total: Option<f64>) {
        if count == 0 {
            return;
        }

        self.welford.add_repeated(0.0, count);
        self.minimum = self.minimum.min(0.0);
        self.maximum = self.maximum.max(0.0);
        self.implicit_zero_count = self.implicit_zero_count.saturating_add(count);

        if let Some(weight_total) = weight_total {
            self.weighted = true;
            // A zero value contributes nothing to the weighted sum, but its
            // weight still counts towards the weighted mean's denominator.
            self.weight_total.add(weight_total);
        }
    }

    fn finish(
        mut self,
        group: Option<f64>,
        options: &SVAggregateOptions,
    ) -> Result<SVAggregateGroup, SVError> {
        if let Some(values) = &mut self.sorted_values {
            values.sort_by(f64::total_cmp);
        }

        if self.weighted && self.weight_total.value() == 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the observation weights of a group sum to zero".to_string(),
            });
        }

        let weighted = self.weighted;
        let value = match options.requested_method() {
            SVAggregateMethod::Mean if weighted => {
                self.weighted_sum.value() / self.weight_total.value()
            }
            SVAggregateMethod::Sum if weighted => self.weighted_sum.value(),
            SVAggregateMethod::Mean => self.welford.mean().unwrap_or(0.0),
            SVAggregateMethod::Median => self
                .sorted_values
                .as_deref()
                .and_then(|values| sv_quantile_with_zeros(values, self.implicit_zero_count, 0.5))
                .unwrap_or(0.0),
            SVAggregateMethod::Sum => self.sum.value(),
            SVAggregateMethod::Minimum => self.minimum,
            SVAggregateMethod::Maximum => self.maximum,
            SVAggregateMethod::StandardDeviation => {
                self.welford.sample_standard_deviation().unwrap_or(0.0)
            }
            SVAggregateMethod::Count => self.welford.count() as f64,
        };

        let error = sv_error_interval(&self, value, options.requested_error());

        Ok(SVAggregateGroup::new(
            group,
            value,
            error,
            self.welford.count(),
        ))
    }
}

/// Builds the requested uncertainty band around the aggregated value.
fn sv_error_interval(
    accumulator: &SVGroupAccumulator,
    value: f64,
    kind: Option<SVAggregateErrorKind>,
) -> Option<SVAggregateErrorInterval> {
    match kind? {
        SVAggregateErrorKind::StandardDeviation => accumulator
            .welford
            .sample_standard_deviation()
            .map(|deviation| SVAggregateErrorInterval::new(value - deviation, value + deviation)),
        SVAggregateErrorKind::StandardError => {
            let deviation = accumulator.welford.sample_standard_deviation()?;
            let standard_error = deviation / (accumulator.welford.count() as f64).sqrt();
            Some(SVAggregateErrorInterval::new(
                value - standard_error,
                value + standard_error,
            ))
        }
        SVAggregateErrorKind::Percentile(probability) => {
            let values = accumulator.sorted_values.as_deref()?;
            let lower = sv_quantile_with_zeros(
                values,
                accumulator.implicit_zero_count,
                (1.0 - probability) / 2.0,
            )?;
            let upper = sv_quantile_with_zeros(
                values,
                accumulator.implicit_zero_count,
                (1.0 + probability) / 2.0,
            )?;
            Some(SVAggregateErrorInterval::new(lower, upper))
        }
    }
}

/// Returns the quantile of an ascending slice plus a run of implicit zeros.
///
/// The zeros are a multiplicity between the negative and the positive segment,
/// the same shape `SVSortedObservations` uses, so a billion implicit zeros cost
/// one comparison rather than one rank each.
fn sv_quantile_with_zeros(sorted: &[f64], implicit_zeros: usize, quantile: f64) -> Option<f64> {
    let length = sorted.len().saturating_add(implicit_zeros);
    if length == 0 {
        return None;
    }

    let position = quantile * (length - 1) as f64;
    let lower_rank = position.floor() as usize;
    let upper_rank = position.ceil() as usize;
    let lower = sv_value_at_rank_with_zeros(sorted, implicit_zeros, lower_rank)?;
    let upper = sv_value_at_rank_with_zeros(sorted, implicit_zeros, upper_rank)?;

    if lower_rank == upper_rank {
        Some(lower)
    } else {
        Some(lower + (position - lower_rank as f64) * (upper - lower))
    }
}

/// Returns the value at a logical rank of a slice plus its implicit zeros.
fn sv_value_at_rank_with_zeros(sorted: &[f64], implicit_zeros: usize, rank: usize) -> Option<f64> {
    let negative_end = sorted.partition_point(|value| *value < 0.0);
    if rank < negative_end {
        return sorted.get(rank).copied();
    }

    let stored_zero_end = sorted.partition_point(|value| *value <= 0.0);
    let zeros = (stored_zero_end - negative_end).saturating_add(implicit_zeros);
    if rank < negative_end + zeros {
        return Some(0.0);
    }

    let index = stored_zero_end + (rank - negative_end - zeros);
    sorted.get(index).copied()
}
