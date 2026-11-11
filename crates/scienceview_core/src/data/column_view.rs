//! The computation boundary: a column as one of three representations.

use crate::data::{SVDataError, SVRepresentation, SVSparseView};

/// A borrowed snapshot of the current window of a live source.
///
/// The snapshot is produced by a bounded rolling window, so its length never
/// exceeds the window capacity and reading it never waits for new data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVStreamingView<'a> {
    values: &'a [f64],
    sequence: u64,
    dropped_samples: u64,
}

impl<'a> SVStreamingView<'a> {
    /// Builds a streaming view over a borrowed window snapshot.
    pub fn new(values: &'a [f64], sequence: u64, dropped_samples: u64) -> Self {
        Self {
            values,
            sequence,
            dropped_samples,
        }
    }

    /// Returns the borrowed window snapshot, oldest sample first.
    pub fn values(&self) -> &'a [f64] {
        self.values
    }

    /// Returns the number of samples the window has accepted so far.
    ///
    /// The sequence number grows monotonically and stays flat when no new data
    /// arrives, which is how a stalled producer becomes observable.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the number of samples the window already dropped.
    pub fn dropped_samples(&self) -> u64 {
        self.dropped_samples
    }

    /// Returns the number of samples in the snapshot.
    pub fn length(&self) -> usize {
        self.values.len()
    }
}

/// A column presented to computation.
///
/// Exactly one variant reaches a kernel, and the kernel dispatches on it
/// exactly once — [`SVColumnView::dispatch`] is the sanctioned way to do so.
/// Dense and streaming views borrow contiguously; the sparse view borrows its
/// indices and values and is never densified.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVColumnView<'a> {
    /// A contiguous, borrowed slice of values.
    Dense(&'a [f64]),
    /// Borrowed indices and values, never expanded.
    Sparse(SVSparseView<'a>),
    /// A borrowed snapshot of the current window of a live source.
    Streaming(SVStreamingView<'a>),
}

impl<'a> SVColumnView<'a> {
    /// Resolves the representation once, for dispatch and observability.
    pub fn representation(&self) -> SVRepresentation {
        match self {
            SVColumnView::Dense(_) => SVRepresentation::Dense,
            SVColumnView::Sparse(_) => SVRepresentation::Sparse,
            SVColumnView::Streaming(_) => SVRepresentation::Streaming,
        }
    }

    /// Returns the logical number of values in the column.
    pub fn length(&self) -> usize {
        match self {
            SVColumnView::Dense(values) => values.len(),
            SVColumnView::Sparse(sparse) => sparse.length(),
            SVColumnView::Streaming(streaming) => streaming.length(),
        }
    }

    /// Returns `true` when the column holds no value.
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Borrows the values as a contiguous slice.
    ///
    /// # Errors
    ///
    /// A sparse view is never densified, so it returns
    /// [`SVDataError::UnsupportedRepresentation`] naming the alternative:
    /// iterate over [`SVSparseView::stored_entries`] instead.
    pub fn dense_values(&self) -> Result<&'a [f64], SVDataError> {
        match self {
            SVColumnView::Dense(values) => Ok(values),
            SVColumnView::Streaming(streaming) => Ok(streaming.values()),
            SVColumnView::Sparse(_) => Err(SVDataError::UnsupportedRepresentation {
                operation: "dense_values",
                representation: SVRepresentation::Sparse,
                alternative: "iterate over the sparse stored entries with `stored_entries`",
            }),
        }
    }

    /// Returns the sparse view, when this column is sparse.
    pub fn sparse_view(&self) -> Option<&SVSparseView<'a>> {
        match self {
            SVColumnView::Sparse(sparse) => Some(sparse),
            SVColumnView::Dense(_) | SVColumnView::Streaming(_) => None,
        }
    }

    /// Returns the streaming view, when this column came from a live source.
    pub fn streaming_view(&self) -> Option<&SVStreamingView<'a>> {
        match self {
            SVColumnView::Streaming(streaming) => Some(streaming),
            SVColumnView::Dense(_) | SVColumnView::Sparse(_) => None,
        }
    }

    /// Returns the value at a logical position.
    ///
    /// Sparse columns answer from their stored entries without being
    /// materialised; a position past the logical length returns `None`.
    pub fn value_at(&self, index: usize) -> Option<f64> {
        match self {
            SVColumnView::Dense(values) => values.get(index).copied(),
            SVColumnView::Sparse(sparse) => sparse.value_at(index),
            SVColumnView::Streaming(streaming) => streaming.values().get(index).copied(),
        }
    }

    /// Dispatches on the representation exactly once.
    ///
    /// Exactly one closure runs, so a kernel cannot accidentally read two
    /// representations or dispatch again further down.
    pub fn dispatch<Result>(
        &self,
        on_dense: impl FnOnce(&'a [f64]) -> Result,
        on_sparse: impl FnOnce(&SVSparseView<'a>) -> Result,
        on_streaming: impl FnOnce(&SVStreamingView<'a>) -> Result,
    ) -> Result {
        match self {
            SVColumnView::Dense(values) => on_dense(values),
            SVColumnView::Sparse(sparse) => on_sparse(sparse),
            SVColumnView::Streaming(streaming) => on_streaming(streaming),
        }
    }
}

#[cfg(test)]
#[path = "column_view_tests.rs"]
mod column_view_tests;
