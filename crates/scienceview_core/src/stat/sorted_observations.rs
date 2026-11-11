//! The sorted observations of a column, implicit zeros included analytically.
//!
//! Sorting a column costs one copy of its *stored* values, never of its
//! implicit zeros: the zeros are kept as a multiplicity between the negative
//! and the positive segment, so a sparse column with a billion logical zeros
//! still answers a quantile in constant extra space.

use crate::sv_stat::observations::SVObservationValues;

/// The logical values of a column in ascending order.
#[derive(Clone, Debug, Default)]
pub(crate) struct SVSortedObservations {
    negative: Vec<f64>,
    zero_count: usize,
    positive: Vec<f64>,
    length: usize,
}

impl SVSortedObservations {
    /// Sorts the stored values and folds the implicit zeros into a multiplicity.
    pub(crate) fn from_observations(observations: &SVObservationValues<'_>) -> Self {
        let mut negative = Vec::new();
        let mut positive = Vec::new();
        let mut zero_count = observations.implicit_zero_count();

        observations.for_each_stored(|value| {
            if value < 0.0 {
                negative.push(value);
            } else if value > 0.0 {
                positive.push(value);
            } else {
                zero_count += 1;
            }
        });

        negative.sort_by(f64::total_cmp);
        positive.sort_by(f64::total_cmp);

        Self {
            negative,
            zero_count,
            positive,
            length: observations.length(),
        }
    }

    /// Returns the value at a logical rank.
    pub(crate) fn value_at_rank(&self, rank: usize) -> Option<f64> {
        if rank >= self.length {
            return None;
        }

        let negative_length = self.negative.len();
        if rank < negative_length {
            return Some(self.negative[rank]);
        }

        if rank < negative_length + self.zero_count {
            return Some(0.0);
        }

        let position = rank - negative_length - self.zero_count;
        self.positive.get(position).copied()
    }

    /// Returns one `(value, cumulative count)` pair per distinct value.
    ///
    /// The implicit zeros contribute a single breakpoint with their whole
    /// multiplicity, so the curve is built without expanding them.
    pub(crate) fn cumulative_breakpoints(&self) -> Vec<(f64, usize)> {
        let mut breakpoints: Vec<(f64, usize)> = Vec::new();
        let mut cumulative = 0usize;

        sv_push_breakpoints(&self.negative, &mut breakpoints, &mut cumulative);
        if self.zero_count > 0 {
            cumulative += self.zero_count;
            breakpoints.push((0.0, cumulative));
        }
        sv_push_breakpoints(&self.positive, &mut breakpoints, &mut cumulative);

        breakpoints
    }

    /// Returns the linearly interpolated quantile of the logical sequence.
    pub(crate) fn quantile(&self, quantile: f64) -> Option<f64> {
        if self.length == 0 {
            return None;
        }

        let position = quantile * (self.length - 1) as f64;
        let lower_rank = position.floor() as usize;
        let upper_rank = position.ceil() as usize;
        let lower = self.value_at_rank(lower_rank)?;
        let upper = self.value_at_rank(upper_rank)?;

        if lower_rank == upper_rank {
            Some(lower)
        } else {
            Some(lower + (position - lower_rank as f64) * (upper - lower))
        }
    }
}

#[cfg(test)]
#[path = "sorted_observations_tests.rs"]
mod sorted_observations_tests;

/// Folds an ascending segment into the cumulative breakpoints.
fn sv_push_breakpoints(
    values: &[f64],
    breakpoints: &mut Vec<(f64, usize)>,
    cumulative: &mut usize,
) {
    for value in values {
        *cumulative += 1;
        match breakpoints.last_mut() {
            Some((last_value, last_cumulative)) if last_value == value => {
                *last_cumulative = *cumulative;
            }
            _ => breakpoints.push((*value, *cumulative)),
        }
    }
}
