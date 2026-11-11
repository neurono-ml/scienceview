//! Resolving a column's representation once, then reading it uniformly.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_mark::SVMarkRequest;

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
        #[cfg(test)]
        sv_resolution_counter::sv_record_resolution();
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

/// The two display-space channels of one operation, each resolved once.
///
/// The operation resolves both columns here and hands the accessors to every
/// builder, so a variant that reads the secondary channel for a second purpose
/// (the text mark formats its value) reads the snapshot the first resolution
/// produced instead of dispatching the representation again.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SVResolvedChannels<'a> {
    horizontal: SVColumnAccessor<'a>,
    vertical: SVColumnAccessor<'a>,
}

impl<'a> SVResolvedChannels<'a> {
    /// Resolves the request's two channels exactly once each.
    pub(crate) fn resolve(request: &SVMarkRequest<'a, '_>) -> Self {
        Self {
            horizontal: SVColumnAccessor::resolve(request.primary()),
            vertical: SVColumnAccessor::resolve(request.secondary()),
        }
    }

    /// Returns the resolved horizontal channel.
    pub(crate) fn horizontal(&self) -> SVColumnAccessor<'a> {
        self.horizontal
    }

    /// Returns the resolved vertical channel.
    pub(crate) fn vertical(&self) -> SVColumnAccessor<'a> {
        self.vertical
    }
}

#[cfg(test)]
pub(crate) use sv_resolution_counter::{sv_reset_resolution_count, sv_resolution_count};

/// Test-only count of representation resolutions on the current thread.
///
/// A thread-local keeps the counter isolated when the test binary runs cases
/// in parallel, and lets a test prove that an operation dispatches each column
/// exactly once.
#[cfg(test)]
mod sv_resolution_counter {
    use std::cell::Cell;

    thread_local! {
        static RESOLUTIONS: Cell<usize> = const { Cell::new(0) };
    }

    /// Resets the counter of the current thread.
    pub(crate) fn sv_reset_resolution_count() {
        RESOLUTIONS.with(|resolutions| resolutions.set(0));
    }

    /// Returns the number of resolutions the current thread performed.
    pub(crate) fn sv_resolution_count() -> usize {
        RESOLUTIONS.with(Cell::get)
    }

    /// Records one resolution on the current thread.
    pub(crate) fn sv_record_resolution() {
        RESOLUTIONS.with(|resolutions| resolutions.set(resolutions.get() + 1));
    }
}
