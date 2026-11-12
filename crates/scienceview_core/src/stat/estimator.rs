//! The estimator trait a caller can implement to fit a custom model.

use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_stat::fit::SV_FIT_CURVE_SAMPLE_COUNT;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;

/// A user-supplied fit model.
///
/// The core keeps its dependency-light model set closed; a caller that needs a
/// robust regression or a domain-specific curve implements this trait and calls
/// [`sv_fit_with_estimator`]. The estimator never sees a backend: it receives
/// data-space columns and returns data-space coefficients.
pub trait SVFitEstimator {
    /// Estimates the model coefficients from paired observations.
    ///
    /// # Errors
    ///
    /// The implementation reports a singular or invalid design as [`SVError`].
    fn estimate(&self, independent: &[f64], values: &[f64]) -> Result<Vec<f64>, SVError>;

    /// Evaluates the fitted model at one data-space point.
    fn evaluate(&self, coefficients: &[f64], point: f64) -> f64;
}

/// A fitted curve produced by a user-supplied estimator.
#[derive(Clone, Debug, PartialEq)]
pub struct SVFitCurve {
    coefficients: Vec<f64>,
    grid: Vec<f64>,
    predicted: Vec<f64>,
    windowed: bool,
}

impl SVFitCurve {
    /// Returns the estimated coefficients.
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the independent values the curve was evaluated at.
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }

    /// Returns the fitted value at each grid sample.
    pub fn predicted(&self) -> &[f64] {
        &self.predicted
    }

    /// Returns `true` when the curve describes only the current window of a
    /// streaming column.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }
}

/// Fits a caller-supplied model to a paired dense or streaming input.
///
/// Sparse storage is rejected with [`SVError::UnsupportedRepresentation`],
/// exactly as the built-in fit is, because two independently sparse columns do
/// not describe the same paired observations a dense or streaming input does.
///
/// # Errors
///
/// Returns [`SVError::InvalidConfiguration`] when the independent column is
/// missing or a column is degenerate, [`SVError::EmptyInput`] when the column
/// is empty, [`SVError::MismatchedStatisticLengths`] when the columns disagree
/// in length, [`SVError::NonFiniteValue`] for a non-finite observation and
/// [`SVError::UnsupportedRepresentation`] for sparse storage.
pub fn sv_fit_with_estimator<Estimator: SVFitEstimator + ?Sized>(
    estimator: &Estimator,
    input: SVStatInput<'_>,
) -> Result<SVFitCurve, SVError> {
    let independent = input
        .secondary()
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "a custom fit needs the independent column through `with_secondary`"
                .to_string(),
        })?;
    let values = input.values();

    for column in [independent, values] {
        if column.representation() == SVRepresentation::Sparse {
            return Err(SVError::UnsupportedRepresentation {
                operation: "fit",
                representation: SVRepresentation::Sparse,
                alternative: "bind dense or streaming columns, or pre-aggregate the sparse view before fitting",
            });
        }
    }

    let length = values.length();
    if length == 0 {
        return Err(SVError::EmptyInput { operation: "fit" });
    }
    if independent.length() != length {
        return Err(SVError::MismatchedStatisticLengths {
            operation: "fit",
            first: length,
            second: independent.length(),
        });
    }

    let independent_observations = SVObservationValues::from_view(independent);
    let value_observations = SVObservationValues::from_view(values);
    independent_observations.validate_finite("fit")?;
    value_observations.validate_finite("fit")?;

    let independent_values = independent_observations
        .contiguous()
        .map(<[f64]>::to_vec)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "the custom fit received a sparse column after rejecting sparse storage"
                .to_string(),
        })?;
    let value_values = value_observations
        .contiguous()
        .map(<[f64]>::to_vec)
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "the custom fit received a sparse column after rejecting sparse storage"
                .to_string(),
        })?;

    let coefficients = estimator.estimate(&independent_values, &value_values)?;
    let low = independent_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let high = independent_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if high <= low {
        return Err(SVError::InvalidConfiguration {
            message: "a fit needs at least two distinct independent values".to_string(),
        });
    }

    let grid: Vec<f64> = (0..SV_FIT_CURVE_SAMPLE_COUNT)
        .map(|position| {
            low + (high - low) * position as f64 / (SV_FIT_CURVE_SAMPLE_COUNT - 1) as f64
        })
        .collect();
    let predicted = grid
        .iter()
        .map(|point| estimator.evaluate(&coefficients, *point))
        .collect();

    Ok(SVFitCurve {
        coefficients,
        grid,
        predicted,
        windowed: input.is_windowed(),
    })
}

#[cfg(test)]
#[path = "estimator_tests.rs"]
mod estimator_tests;
