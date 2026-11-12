//! The conditioned normal-equation solve.

use crate::sv_error::SVError;

use super::conditioning::SVFitConditioning;

/// The smallest pivot a normal-equation solve accepts.
const SV_SINGULAR_PIVOT: f64 = 1.0e-12;

/// The conditioning, the coefficients and the inverse of the conditioned
/// normal matrix one solve produces.
pub(super) type SVFitSolution = (SVFitConditioning, Vec<f64>, Vec<Vec<f64>>);

/// Estimates the polynomial by solving the conditioned normal equations once.
///
/// The function returns the conditioning, the coefficients in the caller's
/// parameterisation and the inverse of the conditioned normal matrix, which the
/// confidence band needs; the inverse is `None` when the solve found the design
/// singular.
pub(super) fn sv_least_squares(
    independent: &[f64],
    values: &[f64],
    degree: usize,
) -> Result<SVFitSolution, SVError> {
    let conditioning = SVFitConditioning::from_values(independent);
    let size = degree + 1;
    let mut normal = vec![vec![0.0; size]; size];
    let mut right_hand_side = vec![0.0; size];

    for (x, y) in independent.iter().copied().zip(values.iter().copied()) {
        let x = conditioning.condition(x);
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
    let conditioned: Vec<f64> = (0..size)
        .map(|row| {
            (0..size)
                .map(|column| inverse[row][column] * right_hand_side[column])
                .sum()
        })
        .collect();

    Ok((conditioning, conditioning.unscale(&conditioned), inverse))
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
