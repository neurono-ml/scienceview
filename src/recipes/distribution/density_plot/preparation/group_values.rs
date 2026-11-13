//! One group of observations and its aligned weights, keeping the
//! representation.

use scienceview_core::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use scienceview_core::sv_error::SVError;

/// The storage of one density column, values or weights.
///
/// A group with no grouping column, no weighting and no missing value borrows
/// its column, so a sparse input reaches the kernel as a sparse view and is
/// never densified. A grouped column, or a column with a dropped missing value,
/// is owned: the values were already read once and no representation is
/// expanded.
pub(crate) enum SVDensityStorage<'values> {
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

impl SVDensityStorage<'_> {
    /// Runs `evaluate` on the stored view.
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

/// The observations of one density curve, with their aligned weights.
pub(crate) struct SVDensityGroupValues<'values> {
    pub(crate) storage: SVDensityStorage<'values>,
    pub(crate) weights: Option<SVDensityStorage<'values>>,
}

impl SVDensityGroupValues<'_> {
    /// Applies the kernel to the group's values and its weights together.
    ///
    /// # Errors
    ///
    /// Returns the evaluation error when either column cannot be read.
    pub(crate) fn evaluate<Output>(
        &self,
        evaluate: impl FnOnce(SVColumnView<'_>, Option<SVColumnView<'_>>) -> Result<Output, SVError>,
    ) -> Result<Output, SVError> {
        self.storage.evaluate(|values| match &self.weights {
            None => evaluate(values, None),
            Some(weights) => weights.evaluate(|weights| evaluate(values, Some(weights))),
        })
    }

    /// Returns the logical number of observations, implicit zeros included.
    pub(crate) fn observation_count(&self) -> usize {
        self.storage.observation_count()
    }

    /// Returns the total weight the group carries, or its observation count
    /// when it is unweighted.
    pub(crate) fn total_weight(&self) -> f64 {
        let Some(weights) = &self.weights else {
            return self.observation_count() as f64;
        };
        match weights {
            SVDensityStorage::OwnedDense(values) => values.iter().sum(),
            SVDensityStorage::OwnedSparse { values, .. } => values.iter().sum(),
            SVDensityStorage::Borrowed(SVColumnView::Dense(values)) => values.iter().sum(),
            SVDensityStorage::Borrowed(SVColumnView::Streaming(streaming)) => {
                streaming.values().iter().sum()
            }
            SVDensityStorage::Borrowed(SVColumnView::Sparse(sparse)) => {
                sparse.values().iter().sum()
            }
            SVDensityStorage::OwnedStreaming { values, .. } => values.iter().sum(),
        }
    }
}
