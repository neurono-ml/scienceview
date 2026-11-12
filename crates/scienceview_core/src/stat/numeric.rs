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

/// Returns the standard-normal quantile of a probability.
///
/// This is Acklam's rational approximation: it is accurate to roughly one part
/// in `1e-9`, which is more than a confidence band can express, and it keeps
/// the core free of a statistics dependency.
// The published Acklam coefficients are written with their full precision on
// purpose: truncating them would silently change the approximation error.
#[allow(clippy::excessive_precision)]
pub(crate) fn sv_normal_quantile(probability: f64) -> f64 {
    const A: [f64; 6] = [
        -3.969_683_028_665_376e+01,
        2.209_460_984_245_205e+02,
        -2.759_285_104_469_687e+02,
        1.383_577_518_672_690e+02,
        -3.066_479_806_614_716e+01,
        2.506_628_277_459_239e+00,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e+01,
        1.615_858_368_580_409e+02,
        -1.556_989_798_598_866e+02,
        6.680_131_188_771_972e+01,
        -1.328_068_155_288_572e+01,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-03,
        -3.223_964_580_411_365e-01,
        -2.400_758_277_161_838e+00,
        -2.549_732_539_343_734e+00,
        4.374_664_141_464_968e+00,
        2.938_163_982_698_783e+00,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-03,
        3.224_671_290_700_398e-01,
        2.445_134_137_142_996e+00,
        3.754_408_661_907_416e+00,
    ];
    const LOWER_TAIL: f64 = 0.024_25;
    const UPPER_TAIL: f64 = 1.0 - LOWER_TAIL;

    if probability <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if probability >= 1.0 {
        return f64::INFINITY;
    }

    if probability < LOWER_TAIL {
        let q = (-2.0 * probability.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if probability <= UPPER_TAIL {
        let q = probability - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - probability).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}

#[cfg(test)]
#[path = "numeric_tests.rs"]
mod numeric_tests;
