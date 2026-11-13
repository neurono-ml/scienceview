//! The streaming representation at the Python boundary: an owned bounded
//! rolling window fed by `push`.
//!
//! A producer pushes samples from Python; a read borrows the current window and
//! never waits. The window drops its oldest sample once it is full, so a stalled
//! consumer cannot grow memory without bound.

use scienceview::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVDataSource, SVReactivity, SVSourceSchema,
};
use scienceview::sv_error::SVError;
use scienceview::sv_window::{SVRollingWindow, SVWindowPolicy, SVWindowReadGuard};
use std::sync::{Mutex, MutexGuard};

/// A two-column live source: the value pushed by the producer and its implicit
/// position.
///
/// The value column is named `value` and carries the `Y` role; the position
/// column is named `index`, carries the `X` role and counts every sample the
/// stream ever accepted, so a window that dropped samples still produces a
/// truthful horizontal axis.
#[derive(Debug)]
pub struct SVPythonStream {
    descriptors: [SVColumnDescriptor<'static>; 2],
    values: Mutex<SVRollingWindow>,
    indices: Mutex<SVRollingWindow>,
}

impl SVPythonStream {
    /// Builds a stream that retains the last `capacity` samples.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidWindowCapacity`] when the capacity is zero.
    pub fn new(capacity: usize) -> Result<Self, SVError> {
        Ok(Self {
            descriptors: [
                SVColumnDescriptor::new("index", SVColumnRole::X),
                SVColumnDescriptor::new("value", SVColumnRole::Y),
            ],
            values: Mutex::new(SVRollingWindow::new(SVWindowPolicy::drop_oldest(capacity))?),
            indices: Mutex::new(SVRollingWindow::new(SVWindowPolicy::drop_oldest(capacity))?),
        })
    }

    /// Appends every sample, applying the overflow policy of the window.
    ///
    /// Returns the number of samples the stream accepted.
    pub fn push(&self, samples: impl IntoIterator<Item = f64>) -> usize {
        let mut values = sv_lock(&self.values);
        let mut indices = sv_lock(&self.indices);
        let mut accepted = 0;
        for sample in samples {
            let position = indices.total_samples() as f64;
            indices.push(position);
            values.push(sample);
            accepted += 1;
        }
        accepted
    }

    /// The number of samples currently retained.
    pub fn length(&self) -> usize {
        sv_lock(&self.values).length()
    }

    /// The sequence number of the newest retained sample.
    pub fn sequence(&self) -> u64 {
        sv_lock(&self.values).sequence()
    }

    /// The number of samples the window already dropped.
    pub fn dropped_samples(&self) -> u64 {
        sv_lock(&self.values).dropped_samples()
    }

    /// The maximum number of samples the window retains.
    pub fn capacity(&self) -> usize {
        sv_lock(&self.values).capacity()
    }

    fn read(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "value" => Ok(SVColumnSnapshot::Window(SVWindowReadGuard::new(sv_lock(
                &self.values,
            )))),
            "index" => Ok(SVColumnSnapshot::Window(SVWindowReadGuard::new(sv_lock(
                &self.indices,
            )))),
            other => Err(SVError::ColumnNotFound {
                name: other.to_owned(),
            }),
        }
    }
}

impl SVDataSource for SVPythonStream {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(&self.descriptors)
    }

    fn reactivity(&self) -> SVReactivity {
        SVReactivity::Observed
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.read(name)
    }
}

/// A lock helper kept private to this module so the public surface exposes no
/// interior mutability detail.
fn sv_lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
