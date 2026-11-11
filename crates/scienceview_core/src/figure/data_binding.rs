//! The data a figure carries, in the representation it arrived in.
//!
//! This is the placeholder binding of the figure substrate: borrowed values
//! plus the representation and the logical number of marks. The dedicated data
//! layer replaces the constructors behind the same builder method, so a recipe
//! never learns which representation it was given until it dispatches.

use super::representation::SVDataRepresentation;
use crate::sv_error::SVError;

/// Borrowed figure data and the representation it arrived in.
///
/// Values stay borrowed from the caller: the binding copies nothing, so a
/// figure never doubles the memory of the data it plots.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVDataBinding<'values> {
    representation: SVDataRepresentation,
    values: &'values [f64],
    point_count: usize,
}

impl<'values> SVDataBinding<'values> {
    /// A dense binding: the values are contiguous.
    pub fn dense(values: &'values [f64]) -> Self {
        Self {
            representation: SVDataRepresentation::Dense,
            values,
            point_count: values.len(),
        }
    }

    /// A sparse binding: the values are present values, `point_count` is the
    /// logical length of the series.
    ///
    /// The logical length is never smaller than the values actually present: a
    /// smaller count is rejected with [`SVError::InconsistentPointCount`].
    pub fn sparse(values: &'values [f64], point_count: usize) -> Result<Self, SVError> {
        Self::with_point_count(SVDataRepresentation::Sparse, values, point_count)
    }

    /// A streaming binding: the values are a snapshot of the current window.
    ///
    /// The logical length is never smaller than the values actually present: a
    /// smaller count is rejected with [`SVError::InconsistentPointCount`].
    pub fn streaming(values: &'values [f64], point_count: usize) -> Result<Self, SVError> {
        Self::with_point_count(SVDataRepresentation::Streaming, values, point_count)
    }

    fn with_point_count(
        representation: SVDataRepresentation,
        values: &'values [f64],
        point_count: usize,
    ) -> Result<Self, SVError> {
        if point_count < values.len() {
            return Err(SVError::InconsistentPointCount {
                point_count,
                value_count: values.len(),
            });
        }

        Ok(Self {
            representation,
            values,
            point_count,
        })
    }

    /// The representation the values arrived in.
    pub fn representation(self) -> SVDataRepresentation {
        self.representation
    }

    /// The values, borrowed.
    pub fn values(self) -> &'values [f64] {
        self.values
    }

    /// The logical number of marks the series describes.
    pub fn point_count(self) -> usize {
        self.point_count
    }
}
