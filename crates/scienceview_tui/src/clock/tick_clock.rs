//! A bounded, elapsed-time clock for a caller-supplied frame source.
//!
//! The clock owns no timer and no thread. The caller reads the terminal (or
//! sleeps) and passes the elapsed time; the clock decides whether a frame is
//! due. This keeps the loop deterministic in tests and honest on a slow
//! terminal: when a tick takes longer than the frame interval the clock
//! advances by the elapsed time, not by one fixed tick.

use std::time::Duration;

/// What one turn of the tick clock produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVTickOutcome {
    /// A frame is due at this animation position.
    Frame(Duration),
    /// No frame is due yet; the clock is still accumulating time.
    Idle,
}

/// Advances an animation position at a bounded frame rate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVTickClock {
    frame_interval: Duration,
    position: Duration,
    pending: Duration,
}

impl SVTickClock {
    /// Builds a clock that yields a frame at most once per `frame_interval`.
    ///
    /// A zero or non-finite interval becomes one millisecond, so the loop can
    /// never busy-spin without a bound.
    pub fn new(frame_interval: Duration) -> Self {
        Self {
            frame_interval: if frame_interval.is_zero() {
                Duration::from_millis(1)
            } else {
                frame_interval
            },
            position: Duration::ZERO,
            pending: Duration::ZERO,
        }
    }

    /// Builds a clock from a target frame rate.
    ///
    /// A rate of zero falls back to one frame per second.
    pub fn from_frames_per_second(frames_per_second: u32) -> Self {
        let frames_per_second = frames_per_second.max(1);
        Self::new(Duration::from_micros(
            1_000_000 / u64::from(frames_per_second),
        ))
    }

    /// Returns the configured frame interval.
    pub const fn frame_interval(&self) -> Duration {
        self.frame_interval
    }

    /// Returns the animation position reached so far.
    pub const fn position(&self) -> Duration {
        self.position
    }

    /// Advances the clock by `elapsed` and reports whether a frame is due.
    ///
    /// The position accumulates the whole elapsed time, including the part
    /// below the frame interval, so no time is lost between frames. At most one
    /// frame is reported per call, which is what bounds the tick rate.
    pub fn tick(&mut self, elapsed: Duration) -> SVTickOutcome {
        self.position = self.position.saturating_add(elapsed);
        self.pending = self.pending.saturating_add(elapsed);

        if self.pending < self.frame_interval {
            return SVTickOutcome::Idle;
        }

        self.pending = Duration::ZERO;
        SVTickOutcome::Frame(self.position)
    }

    /// Advances the clock and returns the position when a frame is due.
    pub fn advance(&mut self, elapsed: Duration) -> Option<Duration> {
        match self.tick(elapsed) {
            SVTickOutcome::Frame(position) => Some(position),
            SVTickOutcome::Idle => None,
        }
    }

    /// Returns the clock to the start of the timeline.
    pub fn reset(&mut self) {
        self.position = Duration::ZERO;
        self.pending = Duration::ZERO;
    }
}
