//! Numerically stable primitives shared by the statistical kernels.
//!
//! The kernel never accumulates a long float sum with the naive `+=` operator:
//! [`SVCompensatedSum`] keeps a Kahan compensation term and [`SVWelford`]
//! updates the mean and the sum of squared deviations in one pass, so the
//! variance of a large sample does not depend on the magnitude of its values
//! and an order-independent statistic stays independent of the input order.

use crate::sv_error::SVError;

/// Validates that every value is finite, reporting the first failure.
pub(crate) fn sv_validate_finite<'a>(
    operation: &'static str,
    values: impl Iterator<Item = (usize, f64)> + 'a,
) -> Result<(), SVError> {
    for (position, value) in values {
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation,
                position,
            });
        }
    }
    Ok(())
}

/// A Neumaier-compensated running sum.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SVCompensatedSum {
    sum: f64,
    compensation: f64,
}

impl SVCompensatedSum {
    /// Builds an empty compensated sum.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds one value.
    pub(crate) fn add(&mut self, value: f64) {
        let updated = self.sum + value;
        if self.sum.abs() >= value.abs() {
            self.compensation += (self.sum - updated) + value;
        } else {
            self.compensation += (value - updated) + self.sum;
        }
        self.sum = updated;
    }

    /// Returns the accumulated sum.
    pub(crate) fn value(&self) -> f64 {
        self.sum + self.compensation
    }
}

/// Welford's one-pass mean and variance accumulator.
///
/// `add_repeated` folds a run of identical values in constant time, which is
/// how a sparse column contributes its implicit zeros without being expanded.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SVWelford {
    count: usize,
    mean: f64,
    sum_of_squared_deviations: f64,
}

impl SVWelford {
    /// Builds an empty accumulator.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds one observation.
    pub(crate) fn add(&mut self, value: f64) {
        self.add_repeated(value, 1);
    }

    /// Adds `repetitions` identical observations.
    pub(crate) fn add_repeated(&mut self, value: f64, repetitions: usize) {
        if repetitions == 0 {
            return;
        }

        let previous_count = self.count as f64;
        let added_count = repetitions as f64;
        let updated_count = previous_count + added_count;
        let updated_mean = (previous_count * self.mean + added_count * value) / updated_count;
        self.sum_of_squared_deviations +=
            added_count * (value - self.mean) * (value - updated_mean);
        self.count = self.count.saturating_add(repetitions);
        self.mean = updated_mean;
    }

    /// Returns the number of accumulated observations.
    pub(crate) fn count(&self) -> usize {
        self.count
    }

    /// Returns the mean, or `None` when nothing was accumulated.
    pub(crate) fn mean(&self) -> Option<f64> {
        (self.count > 0).then_some(self.mean)
    }

    /// Returns the bias-corrected sample variance, or `None` when fewer than
    /// two observations were accumulated.
    pub(crate) fn sample_variance(&self) -> Option<f64> {
        (self.count > 1).then(|| self.sum_of_squared_deviations / (self.count - 1) as f64)
    }

    /// Returns the sample standard deviation.
    pub(crate) fn sample_standard_deviation(&self) -> Option<f64> {
        self.sample_variance().map(f64::sqrt)
    }
}

#[cfg(test)]
#[path = "numeric_tests.rs"]
mod numeric_tests;
