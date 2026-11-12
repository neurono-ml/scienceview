//! A finite, non-negative span of playback time.

use std::time::Duration;

use crate::sv_error::SVError;

/// The per-keyframe duration of an animation, and any explicit time span the
/// playback API accepts.
///
/// A duration is never negative and never `NaN`: the fallible constructors
/// return [`SVError::InvalidAnimationDuration`] instead, so a timeline cannot
/// be built on a broken clock. Zero is valid and means an instant swap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SVDuration {
    duration: Duration,
}

impl SVDuration {
    /// The duration the animation surface uses when none is configured.
    pub const DEFAULT_MILLISECONDS: u64 = 1000;

    /// Builds a duration from a whole number of milliseconds.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidAnimationDuration`] when `milliseconds` is
    /// negative.
    pub fn from_milliseconds(milliseconds: i64) -> Result<Self, SVError> {
        let milliseconds =
            u64::try_from(milliseconds).map_err(|_| SVError::InvalidAnimationDuration)?;
        Ok(Self::milliseconds(milliseconds))
    }

    /// Builds a duration from a number of seconds.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidAnimationDuration`] when `seconds` is
    /// negative, not finite or too large to represent.
    pub fn from_seconds(seconds: f64) -> Result<Self, SVError> {
        let duration =
            Duration::try_from_secs_f64(seconds).map_err(|_| SVError::InvalidAnimationDuration)?;
        Ok(Self { duration })
    }

    /// Builds a duration from a whole number of milliseconds, infallibly.
    pub fn milliseconds(milliseconds: u64) -> Self {
        Self {
            duration: Duration::from_millis(milliseconds),
        }
    }

    /// Returns the zero duration, an instant swap.
    pub fn zero() -> Self {
        Self {
            duration: Duration::ZERO,
        }
    }

    /// Borrows the underlying standard-library duration.
    pub fn as_duration(&self) -> Duration {
        self.duration
    }

    /// Returns the duration in seconds.
    pub fn as_seconds(&self) -> f64 {
        self.duration.as_secs_f64()
    }

    /// Returns `true` when the duration is zero.
    pub fn is_zero(&self) -> bool {
        self.duration.is_zero()
    }

    pub(crate) fn sv_from_duration(duration: Duration) -> Self {
        Self { duration }
    }
}

impl Default for SVDuration {
    fn default() -> Self {
        Self::milliseconds(Self::DEFAULT_MILLISECONDS)
    }
}
