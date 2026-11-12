//! The fitted curve: link transformations, evaluation and confidence band.

use crate::sv_error::SVError;
use crate::sv_stat::fit_options::SVFitModel;
use crate::sv_stat::numeric::sv_normal_quantile;

use super::conditioning::SVFitConditioning;

/// The number of samples the predicted curve carries.
pub(crate) const SV_FIT_CURVE_SAMPLE_COUNT: usize = 128;

/// Applies the model's link transformation to the observation pairs.
pub(super) fn sv_transform_columns(
    model: SVFitModel,
    independent: &[f64],
    values: &[f64],
) -> Result<(Vec<f64>, Vec<f64>), SVError> {
    let mut transformed_x = Vec::with_capacity(independent.len());
    let mut transformed_y = Vec::with_capacity(values.len());

    for (position, (x, y)) in independent
        .iter()
        .copied()
        .zip(values.iter().copied())
        .enumerate()
    {
        let x = match model {
            SVFitModel::Logarithmic | SVFitModel::Power => {
                if x <= 0.0 {
                    return Err(SVError::InvalidConfiguration {
                        message: format!(
                            "`{}` needs a positive independent value at position {position}",
                            model.as_str()
                        ),
                    });
                }
                x.ln()
            }
            _ => x,
        };
        let y = match model {
            SVFitModel::Exponential | SVFitModel::Power => {
                if y <= 0.0 {
                    return Err(SVError::InvalidConfiguration {
                        message: format!(
                            "`{}` needs a positive value at position {position}",
                            model.as_str()
                        ),
                    });
                }
                y.ln()
            }
            _ => y,
        };
        transformed_x.push(x);
        transformed_y.push(y);
    }

    Ok((transformed_x, transformed_y))
}

/// Converts the linear-space coefficients into the model's parameterisation.
pub(super) fn sv_model_coefficients(model: SVFitModel, coefficients: Vec<f64>) -> Vec<f64> {
    match model {
        SVFitModel::Exponential | SVFitModel::Power => {
            let mut coefficients = coefficients;
            if let Some(intercept) = coefficients.first_mut() {
                *intercept = intercept.exp();
            }
            coefficients
        }
        _ => coefficients,
    }
}

/// Returns the range the predicted curve is evaluated over.
pub(super) fn sv_curve_range(independent: &[f64]) -> Result<(f64, f64), SVError> {
    let low = independent.iter().copied().fold(f64::INFINITY, f64::min);
    let high = independent
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);

    if high > low {
        Ok((low, high))
    } else {
        Err(SVError::InvalidConfiguration {
            message: "a fit needs at least two distinct independent values".to_string(),
        })
    }
}

/// Evaluates the fitted model at one data-space point.
pub(super) fn sv_evaluate(model: SVFitModel, coefficients: &[f64], point: f64) -> f64 {
    match model {
        SVFitModel::Linear | SVFitModel::Polynomial => coefficients
            .iter()
            .rev()
            .fold(0.0, |accumulator, coefficient| {
                accumulator * point + coefficient
            }),
        SVFitModel::Exponential => coefficients[0] * (coefficients[1] * point).exp(),
        SVFitModel::Logarithmic => coefficients[0] + coefficients[1] * point.ln(),
        SVFitModel::Power => coefficients[0] * point.powf(coefficients[1]),
    }
}

/// The lower and upper confidence samples of a fit, when one was requested.
pub(super) type SVFitConfidenceBand = (Option<Vec<f64>>, Option<Vec<f64>>);

/// The solved model a confidence band is computed around.
pub(super) struct SVFitEstimate<'a> {
    pub(super) coefficients: &'a [f64],
    pub(super) inverse: &'a [Vec<f64>],
    pub(super) conditioning: SVFitConditioning,
}

/// Computes the two-sided confidence band of a linear or polynomial fit.
pub(super) fn sv_confidence_band(
    grid: &[f64],
    predicted: &[f64],
    estimate: &SVFitEstimate<'_>,
    independent: &[f64],
    values: &[f64],
    confidence: f64,
) -> Result<SVFitConfidenceBand, SVError> {
    let coefficients = estimate.coefficients;
    let inverse = estimate.inverse;
    let conditioning = &estimate.conditioning;
    let size = coefficients.len();
    let degrees_of_freedom = independent.len().saturating_sub(size);
    let residual_sum: f64 = independent
        .iter()
        .copied()
        .zip(values.iter().copied())
        .map(|(x, y)| {
            let residual = y - sv_evaluate(SVFitModel::Polynomial, coefficients, x);
            residual * residual
        })
        .sum();
    let residual_variance = residual_sum / degrees_of_freedom as f64;
    let quantile = sv_normal_quantile((1.0 + confidence) / 2.0);

    let mut lower = Vec::with_capacity(grid.len());
    let mut upper = Vec::with_capacity(grid.len());
    for (point, fitted) in grid.iter().copied().zip(predicted.iter().copied()) {
        let mut design = vec![0.0; size];
        let conditioned_point = conditioning.condition(point);
        let mut power = 1.0;
        for value in design.iter_mut() {
            *value = power;
            power *= conditioned_point;
        }

        let mut variance = 0.0;
        for row in 0..size {
            for column in 0..size {
                variance += design[row] * inverse[row][column] * design[column];
            }
        }
        let margin = quantile * (residual_variance * variance).max(0.0).sqrt();
        lower.push(fitted - margin);
        upper.push(fitted + margin);
    }

    Ok((Some(lower), Some(upper)))
}
