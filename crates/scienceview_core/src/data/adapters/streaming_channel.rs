//! A live column fed by a standard-library channel.

use std::borrow::Cow;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;

use crate::data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVDataError, SVDataSource, SVSourceSchema,
};
use crate::window::{SVRollingWindow, SVWindowPolicy, SVWindowReadGuard, sv_lock};

/// A streaming, single-column source fed by a standard-library channel.
///
/// The producer owns the sending half and pushes samples; a read drains
/// whatever already arrived — never waiting — into a bounded rolling window and
/// hands out a borrowed snapshot of it. Neither the drain nor the read can
/// grow without bound: an overflowing window drops its oldest sample.
#[derive(Debug)]
pub struct SVChannelColumn<'a> {
    descriptor: SVColumnDescriptor<'a>,
    receiver: Mutex<Receiver<f64>>,
    window: Mutex<SVRollingWindow>,
}

impl<'a> SVChannelColumn<'a> {
    /// Wraps a channel receiver with an explicit window policy.
    ///
    /// # Errors
    ///
    /// Returns [`SVDataError::InvalidWindowCapacity`] when the policy allows no
    /// sample.
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        receiver: Receiver<f64>,
        policy: SVWindowPolicy,
    ) -> Result<Self, SVDataError> {
        Ok(Self {
            descriptor: SVColumnDescriptor::new(name, SVColumnRole::Y),
            receiver: Mutex::new(receiver),
            window: Mutex::new(SVRollingWindow::new(policy)?),
        })
    }

    /// Wraps a channel receiver that keeps the last `capacity` samples.
    ///
    /// # Errors
    ///
    /// Returns [`SVDataError::InvalidWindowCapacity`] when the capacity is
    /// zero.
    pub fn with_capacity(
        name: impl Into<Cow<'a, str>>,
        receiver: Receiver<f64>,
        capacity: usize,
    ) -> Result<Self, SVDataError> {
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

    /// Drains every sample already available into the window.
    ///
    /// The drain is non-blocking: a channel that stays empty leaves the window
    /// untouched and the read returns the current window immediately.
    fn drain(&self) {
        let receiver = sv_lock(&self.receiver);
        let mut window = sv_lock(&self.window);
        window.extend(receiver.try_iter());
    }
}

impl SVDataSource for SVChannelColumn<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::from_columns(std::slice::from_ref(&self.descriptor))
    }

    fn reactivity(&self) -> crate::data::SVReactivity {
        crate::data::SVReactivity::EventDriven
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVDataError> {
        if name != self.descriptor.name() {
            return Err(SVDataError::ColumnNotFound {
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
#[path = "streaming_channel_tests.rs"]
mod streaming_channel_tests;
