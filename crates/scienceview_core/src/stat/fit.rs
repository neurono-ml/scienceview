//! The fit kernel.
//!
//! A fit pairs one independent column with one value column. Sparse storage is
//! rejected rather than densified: the implicit zeros of two independently
//! sparse columns would describe a regression through an origin point mass,
//! which is a different question from the one the user asked.

use crate::sv_data::{SVColumnView, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_stat::fit_options::{SVFitModel, SVFitOptions};
use crate::sv_stat::fit_result::SVFitResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::numeric::sv_normal_quantile;
use crate::sv_stat::stat_definition::SVStatResult;

/// The number of samples the predicted curve carries.
pub(crate) const SV_FIT_CURVE_SAMPLE_COUNT: usize = 128;

/// The smallest pivot a normal-equation solve accepts.
const SV_SINGULAR_PIVOT: f64 = 1.0e-12;

/// Applies the fit configuration to the paired input columns.
pub(crate) fn sv_apply_fit(
    options: &SVFitOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let values = input.values();
    sv_reject_sparse(values)?;
    let independent = input
        .secondary()
        .ok_or_else(|| SVError::InvalidConfiguration {
            message: "the fit stat needs the independent column through `with_secondary`"
                .to_string(),
        })?;
    sv_reject_sparse(independent)?;

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

    let independent_values = sv_collect_finite(independent, "fit")?;
    let value_values = sv_collect_finite(values, "fit")?;

    let degree = options.requested_model().degree(options.requested_order());
    if options.requested_model() == SVFitModel::Polynomial && degree == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "a polynomial fit needs a degree of at least one".to_string(),
        });
    }
    if length < degree + 1 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "a fit of degree {degree} needs at least {} observations, received {length}",
                degree + 1
            ),
        });
    }
    if let Some(confidence) = options.requested_confidence() {
        if !confidence.is_finite() || !(0.0..1.0).contains(&confidence) {
            return Err(SVError::InvalidConfiguration {
                message: "a confidence level must lie strictly inside (0, 1)".to_string(),
            });
        }
        if !options.requested_model().supports_confidence_band() {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "`{}` does not carry a computed confidence band",
                    options.requested_model().as_str()
                ),
            });
        }
        if length <= degree + 1 {
            return Err(SVError::InvalidConfiguration {
                message: "a confidence band needs more observations than fitted parameters"
                    .to_string(),
            });
        }
    }

    let (design_x, design_y) = sv_transform_columns(
        options.requested_model(),
        &independent_values,
        &value_values,
    )?;
    let (coefficients, inverse) = sv_least_squares(&design_x, &design_y, degree)?;
    let coefficients = sv_model_coefficients(options.requested_model(), coefficients);

    let (low, high) = sv_curve_range(&independent_values)?;
    let grid: Vec<f64> = (0..SV_FIT_CURVE_SAMPLE_COUNT)
        .map(|position| {
            low + (high - low) * position as f64 / (SV_FIT_CURVE_SAMPLE_COUNT - 1) as f64
        })
        .collect();
    let predicted: Vec<f64> = grid
        .iter()
        .map(|point| sv_evaluate(options.requested_model(), &coefficients, *point))
        .collect();

    let (lower, upper) = match options.requested_confidence() {
        Some(confidence) => sv_confidence_band(
            &grid,
            &predicted,
            &coefficients,
            &inverse,
            &design_x,
            &design_y,
            confidence,
        )?,
        None => (None, None),
    };

    Ok(SVStatResult::Fit(SVFitResult::new(
        options.requested_model(),
        coefficients,
        grid,
        predicted,
        lower,
        upper,
        input.is_windowed(),
    )))
}

/// Rejects a sparse column with the documented alternative path.
fn sv_reject_sparse(view: SVColumnView<'_>) -> Result<(), SVError> {
    if view.representation() == SVRepresentation::Sparse {
        return Err(SVError::UnsupportedRepresentation {
            operation: "fit",
            representation: SVRepresentation::Sparse,
            alternative: "bind dense or streaming columns, or pre-aggregate the sparse view before fitting",
        });
    }
    Ok(())
}

/// Reads one column into a vector, rejecting every non-finite value.
fn sv_collect_finite(view: SVColumnView<'_>, operation: &'static str) -> Result<Vec<f64>, SVError> {
    let mut values = Vec::with_capacity(view.length());
    for position in 0..view.length() {
        let value = view
            .value_at(position)
            .ok_or(SVError::InvalidConfiguration {
                message: "the fit read past the end of a column".to_string(),
            })?;
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation,
                position,
            });
        }
        values.push(value);
    }
    Ok(values)
}

/// Applies the model's link transformation to the observation pairs.
fn sv_transform_columns(
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
fn sv_model_coefficients(model: SVFitModel, coefficients: Vec<f64>) -> Vec<f64> {
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

/// Estimates the polynomial by solving the normal equations once.
///
/// The function returns the coefficients and the inverse of the normal matrix,
/// which the confidence band needs; the inverse is `None` when the solve found
/// the design singular.
fn sv_least_squares(
    independent: &[f64],
    values: &[f64],
    degree: usize,
) -> Result<(Vec<f64>, Vec<Vec<f64>>), SVError> {
    let size = degree + 1;
    let mut normal = vec![vec![0.0; size]; size];
    let mut right_hand_side = vec![0.0; size];

    for (x, y) in independent.iter().copied().zip(values.iter().copied()) {
        let mut power = 1.0;
        for row in 0..size {
            let mut row_power = 1.0;
            for column in 0..size {
                normal[row][column] += power * row_power;
                row_power *= x;
            }
            right_hand_side[row] += power * y;
            power *= x;
        }
    }

    let inverse = sv_invert(&normal).ok_or_else(|| SVError::InvalidConfiguration {
        message: "the fit design is singular; the independent values do not span the model"
            .to_string(),
    })?;
    let coefficients = (0..size)
        .map(|row| {
            (0..size)
                .map(|column| inverse[row][column] * right_hand_side[column])
                .sum()
        })
        .collect();

    Ok((coefficients, inverse))
}

/// Inverts a square matrix with Gauss-Jordan elimination and partial pivoting.
fn sv_invert(matrix: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let size = matrix.len();
    let mut augmented = vec![vec![0.0; size * 2]; size];
    for row in 0..size {
        for column in 0..size {
            augmented[row][column] = matrix[row][column];
        }
        augmented[row][size + row] = 1.0;
    }

    for pivot in 0..size {
        let best = (pivot..size).max_by(|left, right| {
            augmented[*left][pivot]
                .abs()
                .total_cmp(&augmented[*right][pivot].abs())
        })?;
        if augmented[best][pivot].abs() < SV_SINGULAR_PIVOT {
            return None;
        }
        augmented.swap(pivot, best);

        let divisor = augmented[pivot][pivot];
        augmented[pivot]
            .iter_mut()
            .for_each(|value| *value /= divisor);

        for row in 0..size {
            if row == pivot {
                continue;
            }
            let factor = augmented[row][pivot];
            if factor == 0.0 {
                continue;
            }
            let pivot_row = augmented[pivot].clone();
            for (column, value) in augmented[row].iter_mut().enumerate() {
                *value -= factor * pivot_row[column];
            }
        }
    }

    Some(
        augmented
            .into_iter()
            .map(|row| row[size..].to_vec())
            .collect(),
    )
}

/// Returns the range the predicted curve is evaluated over.
fn sv_curve_range(independent: &[f64]) -> Result<(f64, f64), SVError> {
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
fn sv_evaluate(model: SVFitModel, coefficients: &[f64], point: f64) -> f64 {
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
type SVFitConfidenceBand = (Option<Vec<f64>>, Option<Vec<f64>>);

/// Computes the two-sided confidence band of a linear or polynomial fit.
fn sv_confidence_band(
    grid: &[f64],
    predicted: &[f64],
    coefficients: &[f64],
    inverse: &[Vec<f64>],
    independent: &[f64],
    values: &[f64],
    confidence: f64,
) -> Result<SVFitConfidenceBand, SVError> {
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
        let mut power = 1.0;
        for value in design.iter_mut() {
            *value = power;
            power *= point;
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
