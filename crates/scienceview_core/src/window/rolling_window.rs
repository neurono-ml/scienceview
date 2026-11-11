//! A bounded ring buffer fed by a producer and borrowed by the render thread.

use std::collections::VecDeque;
use std::sync::MutexGuard;

use crate::data::SVDataError;
use crate::window::{SVWindowOverflow, SVWindowPolicy};

/// A bounded rolling window of samples.
///
/// The producer owns the window and pushes samples into it; readers only ever
/// observe a snapshot, so a read never waits for new data and the buffer can
/// never grow past its capacity. This is the single bounded copy in the
/// library: asynchronous sources are drained into the window and the render
/// thread borrows the result.
#[derive(Debug)]
pub struct SVRollingWindow {
    policy: SVWindowPolicy,
    values: VecDeque<f64>,
    dropped_samples: u64,
    total_samples: u64,
}

impl SVRollingWindow {
    /// Builds a window with an explicit policy.
    ///
    /// # Errors
    ///
    /// Returns [`SVDataError::InvalidWindowCapacity`] when the capacity is
    /// zero.
    pub fn new(policy: SVWindowPolicy) -> Result<Self, SVDataError> {
        if policy.maximum_number_of_points() == 0 {
            return Err(SVDataError::InvalidWindowCapacity { capacity: 0 });
        }

        Ok(Self {
            values: VecDeque::with_capacity(policy.maximum_number_of_points()),
            policy,
            dropped_samples: 0,
            total_samples: 0,
        })
    }

    /// Builds a window that keeps the last `capacity` samples.
    ///
    /// # Errors
    ///
    /// Returns [`SVDataError::InvalidWindowCapacity`] when the capacity is
    /// zero.
    pub fn with_capacity(capacity: usize) -> Result<Self, SVDataError> {
        Self::new(SVWindowPolicy::drop_oldest(capacity))
    }

    /// Returns the policy of this window.
    pub fn policy(&self) -> SVWindowPolicy {
        self.policy
    }

    /// Returns the maximum number of samples the window holds.
    pub fn capacity(&self) -> usize {
        self.policy.maximum_number_of_points()
    }

    /// Returns the number of samples currently retained.
    pub fn length(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` when no sample is retained.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the number of samples the window accepted so far.
    pub fn total_samples(&self) -> u64 {
        self.total_samples
    }

    /// Returns the number of samples the window already dropped.
    pub fn dropped_samples(&self) -> u64 {
        self.dropped_samples
    }

    /// Returns the sequence number of the newest retained sample.
    ///
    /// It equals [`SVRollingWindow::total_samples`], so a reader can tell
    /// whether new data arrived since the previous read.
    pub fn sequence(&self) -> u64 {
        self.total_samples
    }

    /// Appends one sample, applying the overflow policy when the window is full.
    pub fn push(&mut self, value: f64) {
        if self.values.len() >= self.capacity() {
            match self.policy.overflow() {
                SVWindowOverflow::DropOldest => {
                    self.values.pop_front();
                    self.dropped_samples += 1;
                }
                SVWindowOverflow::DropNewest => {
                    self.dropped_samples += 1;
                    return;
                }
            }
        }

        self.values.push_back(value);
        self.total_samples += 1;
    }

    /// Appends every sample of an iterator, applying the overflow policy.
    pub fn extend<I>(&mut self, values: I)
    where
        I: IntoIterator<Item = f64>,
    {
        for value in values {
            self.push(value);
        }
    }

    /// Returns the retained samples as two borrowed chunks.
    pub fn values(&self) -> (&[f64], &[f64]) {
        self.values.as_slices()
    }

    /// Returns the retained samples as one contiguous borrowed slice.
    ///
    /// The ring is straightened in place when it wraps, which needs exclusive
    /// access; readers holding a [`SVWindowReadGuard`] have it.
    pub fn snapshot(&mut self) -> &[f64] {
        self.values.make_contiguous();
        self.values.as_slices().0
    }

    /// Removes every retained sample without touching the counters.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

/// An exclusive, non-blocking read handle over a shared rolling window.
///
/// A live source keeps its window behind a mutex; the read handle carries the
/// snapshot sequence so that metrics and tests can prove that no data was
/// waited for and that samples were dropped rather than buffered without
/// bound.
#[derive(Debug)]
pub struct SVWindowReadGuard<'a> {
    window: MutexGuard<'a, SVRollingWindow>,
    sequence: u64,
    dropped_samples: u64,
}

impl<'a> SVWindowReadGuard<'a> {
    /// Wraps a mutex guard, capturing the sequence at the moment of the read.
    pub fn new(window: MutexGuard<'a, SVRollingWindow>) -> Self {
        let sequence = window.sequence();
        let dropped_samples = window.dropped_samples();
        Self {
            window,
            sequence,
            dropped_samples,
        }
    }

    /// Returns the sequence number of the newest retained sample.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the number of samples the window had dropped at read time.
    pub fn dropped_samples(&self) -> u64 {
        self.dropped_samples
    }

    /// Returns the retained samples as one contiguous borrowed slice.
    pub fn values(&mut self) -> &[f64] {
        self.window.snapshot()
    }

    /// Returns the retained samples as two borrowed chunks.
    pub fn chunks(&self) -> (&[f64], &[f64]) {
        self.window.values()
    }
}

/// Locks a mutex, recovering the data when a producer panicked while holding it.
#[cfg(any(feature = "streaming", feature = "streaming-tokio"))]
pub(crate) fn sv_lock<Value>(mutex: &std::sync::Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
#[path = "rolling_window_tests.rs"]
mod rolling_window_tests;
