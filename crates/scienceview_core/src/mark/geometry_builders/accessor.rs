//! Resolving a column's representation once, then reading it uniformly.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};

/// A column whose representation was already resolved.
///
/// [`SVColumnView::dispatch`] runs exactly once, here; every later read is a
/// plain match on the resolved accessor. Sparse storage answers from its stored
/// entries and never allocates a dense twin, and a streaming accessor reads the
/// bounded snapshot it was handed, so neither path can block.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SVColumnAccessor<'a> {
    /// A borrowed contiguous slice.
    Dense(&'a [f64]),
    /// Borrowed sparse storage, never expanded.
    Sparse(SVSparseView<'a>),
    /// A borrowed window snapshot.
    Streaming(SVStreamingView<'a>),
}

impl<'a> SVColumnAccessor<'a> {
    /// Resolves the representation exactly once.
    pub(crate) fn resolve(view: SVColumnView<'a>) -> Self {
        view.dispatch(
            Self::Dense,
            |sparse| Self::Sparse(*sparse),
            |streaming| Self::Streaming(*streaming),
        )
    }

    /// Returns the logical number of values.
    pub(crate) fn length(&self) -> usize {
        match self {
            Self::Dense(values) => values.len(),
            Self::Sparse(sparse) => sparse.length(),
            Self::Streaming(streaming) => streaming.length(),
        }
    }

    /// Returns the value at a logical position; an unstored sparse position is
    /// its implicit zero and a position past the length reads as zero, which
    /// keeps a truncated live window drawing instead of panicking.
    pub(crate) fn value_at(&self, index: usize) -> f64 {
        match self {
            Self::Dense(values) => values.get(index).copied().unwrap_or(0.0),
            Self::Sparse(sparse) => sparse.value_at(index).unwrap_or(0.0),
            Self::Streaming(streaming) => streaming.values().get(index).copied().unwrap_or(0.0),
        }
    }
}
