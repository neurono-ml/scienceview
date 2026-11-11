//! A palette derived from the data: the rule decides which fraction of the
//! colour map each value receives.
//!
//! Derivation is deterministic and allocation-light: one sorted index vector,
//! no dependency, no panic on empty data, duplicated values or `NaN`.

use crate::palettes::SVContinuousPalette;
use crate::tokens::SVColour;

/// How a value becomes a colour-map fraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SVDerivationRule {
    /// Linear across the observed minimum and maximum.
    EqualInterval,
    /// By position in the sorted series, so outliers do not skew the scale.
    Rank,
    /// By empirical cumulative distribution, spreading colours evenly across
    /// the sample.
    Quantile,
}

/// A colour map plus the rule that turns data into positions on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVDerivedPalette {
    rule: SVDerivationRule,
    colour_map: SVContinuousPalette,
}

impl SVDerivedPalette {
    pub fn new(rule: SVDerivationRule, colour_map: SVContinuousPalette) -> Self {
        Self { rule, colour_map }
    }

    pub fn rule(&self) -> SVDerivationRule {
        self.rule
    }

    pub fn colour_map(&self) -> &SVContinuousPalette {
        &self.colour_map
    }

    /// Assigns one colour per value, preserving the input order.
    ///
    /// Empty input yields empty output. Non-finite values resolve to the first
    /// stop rather than poisoning the range of the finite values.
    pub fn colours_for_values(&self, values: &[f64]) -> Vec<SVColour> {
        if values.is_empty() {
            return Vec::new();
        }
        match self.rule {
            SVDerivationRule::EqualInterval => {
                let (minimum, maximum) = sv_finite_range(values);
                values
                    .iter()
                    .map(|value| {
                        let fraction = if !value.is_finite() {
                            0.0
                        } else if maximum <= minimum {
                            0.5
                        } else {
                            (value - minimum) / (maximum - minimum)
                        };
                        self.colour_map.colour_at(fraction)
                    })
                    .collect()
            }
            SVDerivationRule::Rank => {
                let order = sv_sorted_order(values);
                let last_rank = (values.len() - 1).max(1) as f64;
                let mut fractions = vec![0.0; values.len()];
                for (rank, index) in order.into_iter().enumerate() {
                    fractions[index] = rank as f64 / last_rank;
                }
                fractions
                    .into_iter()
                    .map(|fraction| self.colour_map.colour_at(fraction))
                    .collect()
            }
            SVDerivationRule::Quantile => {
                let order = sv_sorted_order(values);
                let count = values.len() as f64;
                let mut fractions = vec![0.0; values.len()];
                for (rank, index) in order.into_iter().enumerate() {
                    fractions[index] = (rank as f64 + 0.5) / count;
                }
                fractions
                    .into_iter()
                    .map(|fraction| self.colour_map.colour_at(fraction))
                    .collect()
            }
        }
    }
}

/// The minimum and maximum of the finite values, or `(0.0, 0.0)` when there
/// are none.
fn sv_finite_range(values: &[f64]) -> (f64, f64) {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut found = false;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        minimum = minimum.min(value);
        maximum = maximum.max(value);
        found = true;
    }
    if found {
        (minimum, maximum)
    } else {
        (0.0, 0.0)
    }
}

/// Indices ordered by value with a total order, so `NaN` is ordered rather
/// than dropped.
fn sv_sorted_order(values: &[f64]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|left, right| values[*left].total_cmp(&values[*right]));
    order
}
