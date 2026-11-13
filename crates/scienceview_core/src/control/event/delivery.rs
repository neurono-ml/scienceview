//! One event delivered to a subscriber.

use core::fmt;

use super::change::SVControlChange;
use super::origin::SVControlOrigin;

/// One event delivered to a subscriber.
///
/// The origin identifies the control or widget that requested the change, and
/// the change carries the previous and the current value, so a callback or a
/// script never has to read the figure to know what happened.
#[derive(Clone, Debug, PartialEq)]
pub struct SVControlEvent {
    origin: SVControlOrigin,
    change: SVControlChange,
}

impl SVControlEvent {
    /// Pairs an origin with a change.
    pub fn new(origin: SVControlOrigin, change: SVControlChange) -> Self {
        Self { origin, change }
    }

    /// Returns where the event came from.
    pub fn origin(&self) -> &SVControlOrigin {
        &self.origin
    }

    /// Returns what changed.
    pub fn change(&self) -> &SVControlChange {
        &self.change
    }

    /// Returns the canonical textual encoding of the event.
    pub fn to_text(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for SVControlEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.origin {
            SVControlOrigin::Programmatic => formatter.write_str("origin=programmatic;")?,
            _ => write!(formatter, "{};", self.origin)?,
        }
        write!(formatter, "{}", self.change)
    }
}
