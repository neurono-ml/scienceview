//! Centring and scaling of the fit's independent column.

use crate::sv_stat::numeric::SVCompensatedSum;

/// The centring and scaling applied to the independent column before solving.
///
/// The normal equations are uncentred by nature: a column around `1e9` makes
/// the matrix entries span eighteen orders of magnitude, and the small
/// curvature information is lost in the rounding of the large moments. Solving
/// in `z = (x - offset) / scale` keeps every moment of order one; the solution
/// is then mapped back into the caller's coefficients.
#[derive(Clone, Copy, Debug)]
pub(super) struct SVFitConditioning {
    offset: f64,
    scale: f64,
}

impl SVFitConditioning {
    /// Derives the offset and scale of one independent column.
    pub(super) fn from_values(independent: &[f64]) -> Self {
        let anchor = independent.first().copied().unwrap_or(0.0);
        let mut deviation = SVCompensatedSum::new();
        independent
            .iter()
            .copied()
            .for_each(|value| deviation.add(value - anchor));
        let offset = anchor + deviation.value() / independent.len() as f64;
        let scale = independent
            .iter()
            .copied()
            .map(|value| (value - offset).abs())
            .fold(0.0, f64::max);

        Self {
            offset,
            scale: if scale > 0.0 { scale } else { 1.0 },
        }
    }

    /// Maps one independent value onto its conditioned coordinate.
    pub(super) fn condition(&self, value: f64) -> f64 {
        (value - self.offset) / self.scale
    }

    /// Expands conditioned coefficients back into the caller's parameterisation.
    ///
    /// With `z = (x - offset) / scale`, the coefficient of `x^i` is
    /// `Σ_{j ≥ i} c_j · C(j, i) · (−offset)^(j−i) / scale^j`.
    pub(super) fn unscale(&self, coefficients: &[f64]) -> Vec<f64> {
        let degree = coefficients.len().saturating_sub(1);
        (0..=degree)
            .map(|power| {
                let mut coefficient = 0.0;
                let mut binomial = 1.0;
                for (index, scaled) in coefficients.iter().copied().enumerate().skip(power) {
                    if index > power {
                        // C(index, power) = C(index - 1, power) * index / (index - power)
                        binomial *= index as f64 / (index - power) as f64;
                    }
                    coefficient += scaled * binomial * (-self.offset).powi((index - power) as i32)
                        / self.scale.powi(index as i32);
                }
                coefficient
            })
            .collect()
    }
}
