//! The aggregation kernel.

use std::collections::HashMap;

use crate::sv_data::SVColumnView;
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
use crate::sv_stat::stat_definition::SVStatResult;

/// Applies the aggregation configuration to the input columns.
pub(crate) fn sv_apply_aggregate(
    options: &SVAggregateOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let values = input.values();
    let length = values.length();
    if length == 0 {
        return Err(SVError::EmptyInput {
            operation: "aggregate",
        });
    }

    if let Some(groups) = input.groups() {
        if groups.length() != length {
            return Err(SVError::MismatchedStatisticLengths {
                operation: "aggregate",
                first: length,
                second: groups.length(),
            });
        }
    }
    if let Some(weights) = input.weights() {
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

    let needs_sorted_values = options.requested_method() == SVAggregateMethod::Median
        || matches!(
            options.requested_error(),
            Some(SVAggregateErrorKind::Percentile(_))
        );
    let mut entries: Vec<(Option<f64>, SVGroupAccumulator)> = Vec::new();
    let mut positions: HashMap<u64, usize> = HashMap::new();

    for position in 0..length {
        let value = sv_value_at(values, position)?;
        let group = match input.groups() {
            Some(groups) => Some(sv_value_at(groups, position)?),
            None => None,
        };
        let weight = match input.weights() {
            Some(weights) => Some(sv_weight_at(weights, position)?),
            None => None,
        };

        let key = group.map(sv_category_key).unwrap_or(0);
        let index = match positions.get(&key).copied() {
            Some(index) => index,
            None => {
                let index = entries.len();
                positions.insert(key, index);
                entries.push((group, SVGroupAccumulator::new(needs_sorted_values)));
                index
            }
        };
        entries[index].1.add(value, weight);
    }

    let groups = entries
        .into_iter()
        .map(|(group, accumulator)| accumulator.finish(group, options))
        .collect::<Result<Vec<_>, SVError>>()?;

    Ok(SVStatResult::Groups(SVAggregateResult::new(groups)))
}

/// Reads one logical value and rejects a non-finite one.
fn sv_value_at(view: SVColumnView<'_>, position: usize) -> Result<f64, SVError> {
    let value = view
        .value_at(position)
        .ok_or(SVError::InvalidConfiguration {
            message: "the aggregate read past the end of a column".to_string(),
        })?;
    if !value.is_finite() {
        return Err(SVError::NonFiniteValue {
            operation: "aggregate",
            position,
        });
    }
    Ok(value)
}

/// Reads one logical weight and rejects a negative or non-finite one.
fn sv_weight_at(view: SVColumnView<'_>, position: usize) -> Result<f64, SVError> {
    let weight = sv_value_at(view, position)?;
    if weight < 0.0 {
        return Err(SVError::InvalidConfiguration {
            message: format!("the observation weight at position {position} is negative"),
        });
    }
    Ok(weight)
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
                .and_then(|values| sv_quantile_of_sorted(values, 0.5))
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
            let lower = sv_quantile_of_sorted(values, (1.0 - probability) / 2.0)?;
            let upper = sv_quantile_of_sorted(values, (1.0 + probability) / 2.0)?;
            Some(SVAggregateErrorInterval::new(lower, upper))
        }
    }
}

/// Returns the linearly interpolated quantile of an ascending slice.
pub(crate) fn sv_quantile_of_sorted(sorted: &[f64], quantile: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }

    let position = quantile * (sorted.len() - 1) as f64;
    let lower_rank = position.floor() as usize;
    let upper_rank = position.ceil() as usize;
    let lower = sorted[lower_rank];
    let upper = sorted[upper_rank.min(sorted.len() - 1)];

    if lower_rank == upper_rank {
        Some(lower)
    } else {
        Some(lower + (position - lower_rank as f64) * (upper - lower))
    }
}
