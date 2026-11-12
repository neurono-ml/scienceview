//! The accumulator behind one aggregated value and its uncertainty band.

use crate::sv_error::SVError;
use crate::sv_stat::aggregate_options::{
    SVAggregateErrorKind, SVAggregateMethod, SVAggregateOptions,
};
use crate::sv_stat::aggregate_result::{SVAggregateErrorInterval, SVAggregateGroup};
use crate::sv_stat::numeric::{SVCompensatedSum, SVWelford};

use super::quantile::sv_quantile_with_zeros;

/// Accumulates one group in a single pass.
pub(super) struct SVGroupAccumulator {
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
    pub(super) fn new(needs_sorted_values: bool) -> Self {
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

    pub(super) fn add(&mut self, value: f64, weight: Option<f64>) {
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
    pub(super) fn add_implicit_zeros(&mut self, count: usize, weight_total: Option<f64>) {
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

    pub(super) fn finish(
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
