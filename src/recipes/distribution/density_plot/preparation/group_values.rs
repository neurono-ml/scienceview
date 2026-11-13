//! One group of observations, keeping its representation.

use scienceview_core::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use scienceview_core::sv_error::SVError;

/// The observations of one density curve.
///
/// A group with no grouping column and no missing value borrows its column, so
/// a sparse input reaches the kernel as a sparse view and is never densified.
/// A grouped column, or a column with a dropped missing value, is owned: the
/// values were already read once and no representation is expanded.
pub(crate) enum SVDensityGroupValues<'values> {
    Borrowed(SVColumnView<'values>),
    OwnedDense(Vec<f64>),
    OwnedSparse {
        indices: Vec<usize>,
        values: Vec<f64>,
        length: usize,
    },
    OwnedStreaming {
        values: Vec<f64>,
        sequence: u64,
        dropped_samples: u64,
    },
}

impl SVDensityGroupValues<'_> {
    /// Runs `evaluate` on the group's view.
    ///
    /// # Errors
    ///
    /// Returns the evaluation error, or the sparse-view error when the
    /// prepared support is malformed (it never is by construction).
    pub(crate) fn evaluate<Output>(
        &self,
        evaluate: impl FnOnce(SVColumnView<'_>) -> Result<Output, SVError>,
    ) -> Result<Output, SVError> {
        match self {
            Self::Borrowed(view) => evaluate(*view),
            Self::OwnedDense(values) => evaluate(SVColumnView::Dense(values)),
            Self::OwnedSparse {
                indices,
                values,
                length,
            } => {
                let view = SVSparseView::new(indices, values, *length)?;
                evaluate(SVColumnView::Sparse(view))
            }
            Self::OwnedStreaming {
                values,
                sequence,
                dropped_samples,
            } => evaluate(SVColumnView::Streaming(SVStreamingView::new(
                values,
                *sequence,
                *dropped_samples,
            ))),
        }
    }

    /// Returns the logical number of observations, implicit zeros included.
    pub(crate) fn observation_count(&self) -> usize {
        match self {
            Self::Borrowed(view) => view.length(),
            Self::OwnedDense(values) => values.len(),
            Self::OwnedSparse { length, .. } => *length,
            Self::OwnedStreaming { values, .. } => values.len(),
        }
    }
}
