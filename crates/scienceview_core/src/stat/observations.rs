//! Reading a column without densifying it.
//!
//! A sparse column is presented as its stored entries plus the multiplicity of
//! the implicit zeros, so a kernel can count, weigh or binned the zeros without
//! expanding them into a dense buffer (spec `data-view-boundary`).

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_stat::numeric::sv_validate_finite;

/// The stored observations of a column, with its implicit zeros counted.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SVObservationValues<'a> {
    /// A contiguous column: every value is stored.
    Contiguous(&'a [f64]),
    /// A sparse column: stored entries plus implicit zeros.
    Stored {
        /// The sorted logical positions of the stored values.
        indices: &'a [usize],
        /// The values stored at those positions.
        values: &'a [f64],
        /// The logical length, implicit zeros included.
        length: usize,
    },
}

impl<'a> SVObservationValues<'a> {
    /// Dispatches on the representation exactly once.
    pub(crate) fn from_view(view: SVColumnView<'a>) -> Self {
        view.dispatch(
            SVObservationValues::Contiguous,
            |sparse: &SVSparseView<'a>| SVObservationValues::Stored {
                indices: sparse.indices(),
                values: sparse.values(),
                length: sparse.length(),
            },
            |streaming| SVObservationValues::Contiguous(streaming.values()),
        )
    }

    /// Returns the logical number of observations.
    pub(crate) fn length(&self) -> usize {
        match self {
            SVObservationValues::Contiguous(values) => values.len(),
            SVObservationValues::Stored { length, .. } => *length,
        }
    }

    /// Returns the contiguous slice when every value is stored.
    pub(crate) fn contiguous(&self) -> Option<&'a [f64]> {
        match self {
            SVObservationValues::Contiguous(values) => Some(values),
            SVObservationValues::Stored { .. } => None,
        }
    }

    /// Runs `visit` over every stored value with its logical position.
    pub(crate) fn for_each_stored_entry(&self, mut visit: impl FnMut(usize, f64)) {
        match self {
            SVObservationValues::Contiguous(values) => values
                .iter()
                .copied()
                .enumerate()
                .for_each(|(position, value)| visit(position, value)),
            SVObservationValues::Stored {
                indices, values, ..
            } => indices
                .iter()
                .copied()
                .zip(values.iter().copied())
                .for_each(|(position, value)| visit(position, value)),
        }
    }

    /// Returns the value at a logical position by reading stored entries only.
    pub(crate) fn value_at_position(&self, position: usize) -> Option<f64> {
        match self {
            SVObservationValues::Contiguous(values) => values.get(position).copied(),
            SVObservationValues::Stored {
                indices, values, ..
            } => indices
                .binary_search(&position)
                .ok()
                .map(|index| values[index]),
        }
    }

    /// Visits every maximal run of implicit zeros as `(start, length)`.
    ///
    /// The runs are read from the gaps between stored positions, so a sparse
    /// column with a billion implicit zeros still reports a handful of runs.
    pub(crate) fn for_each_implicit_zero_run(&self, mut visit: impl FnMut(usize, usize)) {
        let SVObservationValues::Stored {
            indices, length, ..
        } = self
        else {
            return;
        };

        let mut expected = 0usize;
        for index in indices.iter().copied() {
            if index > expected {
                visit(expected, index - expected);
            }
            expected = index + 1;
        }
        if expected < *length {
            visit(expected, length - expected);
        }
    }

    /// Returns the number of implicit zeros a sparse column contributes.
    pub(crate) fn implicit_zero_count(&self) -> usize {
        match self {
            SVObservationValues::Contiguous(_) => 0,
            SVObservationValues::Stored {
                indices, length, ..
            } => length.saturating_sub(indices.len()),
        }
    }

    /// Runs `visit` over every stored value, once per stored entry.
    pub(crate) fn for_each_stored(&self, mut visit: impl FnMut(f64)) {
        match self {
            SVObservationValues::Contiguous(values) => {
                values.iter().copied().for_each(&mut visit);
            }
            SVObservationValues::Stored { values, .. } => {
                values.iter().copied().for_each(visit);
            }
        }
    }

    /// Returns the smallest and largest logical value.
    ///
    /// The implicit zeros of a sparse column take part in the comparison.
    pub(crate) fn minimum_and_maximum(&self) -> Option<(f64, f64)> {
        let mut minimum = f64::INFINITY;
        let mut maximum = f64::NEG_INFINITY;
        let mut seen = false;

        self.for_each_stored(|value| {
            minimum = minimum.min(value);
            maximum = maximum.max(value);
            seen = true;
        });

        if self.implicit_zero_count() > 0 {
            minimum = minimum.min(0.0);
            maximum = maximum.max(0.0);
            seen = true;
        }

        seen.then_some((minimum, maximum))
    }

    /// Returns the first logical position of an implicit zero, when any.
    pub(crate) fn first_implicit_zero_position(&self) -> Option<usize> {
        let SVObservationValues::Stored {
            indices, length, ..
        } = self
        else {
            return None;
        };

        let mut expected = 0usize;
        for index in indices.iter().copied() {
            if index > expected {
                return Some(expected);
            }
            expected = index + 1;
        }

        (expected < *length).then_some(expected)
    }

    /// Visits every category in data order, with its multiplicity.
    ///
    /// A sparse column reports its implicit zero once, with the implicit-zero
    /// count as its multiplicity, exactly where it first appears; the visit
    /// therefore preserves first-appearance order without a materialised
    /// expansion.
    pub(crate) fn for_each_first_appearance(&self, mut visit: impl FnMut(f64, u64)) {
        match self {
            SVObservationValues::Contiguous(values) => {
                values.iter().copied().for_each(|value| visit(value, 1));
            }
            SVObservationValues::Stored {
                indices, values, ..
            } => {
                let zero_position = self.first_implicit_zero_position();
                let implicit_zeros = self.implicit_zero_count() as u64;
                let mut zero_visited = zero_position.is_none();

                for (index, value) in indices.iter().copied().zip(values.iter().copied()) {
                    if let Some(position) = zero_position {
                        if !zero_visited && index > position {
                            visit(0.0, implicit_zeros);
                            zero_visited = true;
                        }
                    }
                    visit(value, 1);
                }

                if !zero_visited {
                    visit(0.0, implicit_zeros);
                }
            }
        }
    }

    /// Validates that every logical value is finite.
    pub(crate) fn validate_finite(&self, operation: &'static str) -> Result<(), SVError> {
        match self {
            SVObservationValues::Contiguous(values) => {
                sv_validate_finite(operation, values.iter().copied().enumerate())
            }
            SVObservationValues::Stored {
                indices, values, ..
            } => sv_validate_finite(
                operation,
                indices.iter().copied().zip(values.iter().copied()),
            ),
        }
    }
}
