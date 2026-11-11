//! A live column fed by a Tokio channel, without an async runtime in the core.

use crate::sv_error::SVError;
use std::borrow::Cow;
use std::sync::Mutex;

use tokio::sync::mpsc::Receiver;

use crate::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVDataSource, SVReactivity, SVSourceSchema,
};
use crate::sv_window::{SVRollingWindow, SVWindowPolicy, SVWindowReadGuard, sv_lock};

/// A streaming, single-column source fed by a Tokio channel.
///
/// An asynchronous producer owns its runtime and pushes samples; this adapter
/// presents them synchronously by draining whatever is already available with
/// `try_recv`, so the core never depends on Tokio's runtime, only on its
/// channel type, and the render thread never waits.
#[derive(Debug)]
pub struct SVTokioChannelColumn<'a> {
    descriptor: SVColumnDescriptor<'a>,
    receiver: Mutex<Receiver<f64>>,
    window: Mutex<SVRollingWindow>,
}

impl<'a> SVTokioChannelColumn<'a> {
    /// Wraps a Tokio receiver with an explicit window policy.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidWindowCapacity`] when the policy allows no
    /// sample.
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        receiver: Receiver<f64>,
        policy: SVWindowPolicy,
    ) -> Result<Self, SVError> {
        Ok(Self {
            descriptor: SVColumnDescriptor::new(name, SVColumnRole::Y),
            receiver: Mutex::new(receiver),
            window: Mutex::new(SVRollingWindow::new(policy)?),
        })
    }

    /// Wraps a Tokio receiver that keeps the last `capacity` samples.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidWindowCapacity`] when the capacity is
    /// zero.
    pub fn with_capacity(
        name: impl Into<Cow<'a, str>>,
        receiver: Receiver<f64>,
        capacity: usize,
    ) -> Result<Self, SVError> {
        Self::new(name, receiver, SVWindowPolicy::drop_oldest(capacity))
    }

    /// Changes the role the column declares.
    #[must_use]
    pub fn with_role(mut self, role: SVColumnRole) -> Self {
        self.descriptor = SVColumnDescriptor::new(self.descriptor.name().to_owned(), role);
        self
    }

    /// Returns the column name.
    pub fn name(&self) -> &str {
        self.descriptor.name()
    }

    /// Drains every sample already available, without awaiting.
    fn drain(&self) {
        let mut receiver = sv_lock(&self.receiver);
        let mut window = sv_lock(&self.window);
        while let Ok(value) = receiver.try_recv() {
            window.push(value);
        }
    }
}

impl SVDataSource for SVTokioChannelColumn<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(std::slice::from_ref(&self.descriptor))
    }

    fn reactivity(&self) -> SVReactivity {
        SVReactivity::EventDriven
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        if name != self.descriptor.name() {
            return Err(SVError::ColumnNotFound {
                name: name.to_owned(),
            });
        }

        self.drain();
        Ok(SVColumnSnapshot::Window(SVWindowReadGuard::new(sv_lock(
            &self.window,
        ))))
    }
}

#[cfg(test)]
#[path = "tokio_channel_tests.rs"]
mod tokio_channel_tests;
