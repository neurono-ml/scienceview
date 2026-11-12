//! Resolution of motion tokens into the transition timeline.
//!
//! # The built-in defaults
//!
//! With the `theme` feature a caller resolves the style from the compiled
//! theme's motion tokens ([`SVMotionStyle::from_motion_tokens`]). Without it,
//! [`SVMotionStyle::built_in`] supplies the same documented defaults the default
//! theme carries, so the same figure looks the same either way:
//!
//! | Name | Value |
//! |---|---|
//! | `motion.duration.fast` | 120 ms |
//! | `motion.duration.normal` | 240 ms |
//! | `motion.duration.slow` | 400 ms |
//! | `motion.easing.standard` | cubic-bezier(0.2, 0, 0, 1) |
//!
//! A style may declare reduced motion, which collapses its duration to zero and
//! makes every frame its final state. There is no per-call override: the theme
//! owns the accessibility decision (design D4).

use std::time::Duration;

use super::easing::SVEasing;

/// The resolved timing of one transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVMotionStyle {
    duration: Duration,
    easing: SVEasing,
    delay: Duration,
    reduced_motion: bool,
}

impl SVMotionStyle {
    /// The built-in defaults: the normal duration and the standard easing.
    pub fn built_in() -> Self {
        Self {
            duration: Duration::from_millis(240),
            easing: SVEasing::CubicBezier {
                x1: 0.2,
                y1: 0.0,
                x2: 0.0,
                y2: 1.0,
            },
            delay: Duration::ZERO,
            reduced_motion: false,
        }
    }

    /// An instant swap: a zero duration and the default easing.
    pub fn instant() -> Self {
        Self {
            duration: Duration::ZERO,
            easing: SVEasing::EaseInOut,
            delay: Duration::ZERO,
            reduced_motion: false,
        }
    }

    /// Resolves the style from a compiled theme's motion tokens.
    ///
    /// The duration is the theme's normal token and the easing is its standard
    /// token; both already collapse under the theme's reduced-motion flag, which
    /// the style records as well.
    #[cfg(feature = "theme")]
    pub fn from_motion_tokens(tokens: &scienceview_theme::SVMotionTokens) -> Self {
        Self {
            duration: tokens.duration(scienceview_theme::SVMotionDuration::Normal),
            easing: SVEasing::from(tokens.easing(scienceview_theme::SVMotionEasing::Standard)),
            delay: Duration::ZERO,
            reduced_motion: tokens.reduced_motion(),
        }
    }

    /// Returns the authored duration, already collapsed under reduced motion.
    pub fn duration(&self) -> Duration {
        if self.reduced_motion {
            Duration::ZERO
        } else {
            self.duration
        }
    }

    /// Returns the easing curve.
    pub fn easing(&self) -> SVEasing {
        self.easing
    }

    /// Returns the per-series delay.
    pub fn delay(&self) -> Duration {
        self.delay
    }

    /// Returns `true` when the style declares reduced motion.
    pub fn is_reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    /// Sets the duration.
    #[must_use]
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the easing curve.
    #[must_use]
    pub fn with_easing(mut self, easing: SVEasing) -> Self {
        self.easing = easing;
        self
    }

    /// Sets the per-series delay.
    #[must_use]
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Declares reduced motion, collapsing the duration to zero.
    ///
    /// The declaration is one-way on purpose (design D4): a style is either
    /// subject to reduced motion or not, and a caller cannot switch the
    /// declaration off once it is made. A theme that reduces motion therefore
    /// keeps every frame at its final state.
    #[must_use]
    pub fn reduced(mut self) -> Self {
        self.reduced_motion = true;
        self
    }

    /// Maps a caller-supplied elapsed time onto the progress of the timeline.
    ///
    /// Reduced motion and a zero duration are an instant swap: the progress is
    /// `1.0`. Otherwise the delay is subtracted first, so a mark scheduled later
    /// holds its previous state until its turn.
    pub fn progress_at(&self, elapsed: Duration) -> f64 {
        let duration = self.duration();
        if self.reduced_motion || duration.is_zero() {
            return 1.0;
        }

        let elapsed_after_delay = elapsed.saturating_sub(self.delay).as_secs_f64();
        (elapsed_after_delay / duration.as_secs_f64()).clamp(0.0, 1.0)
    }
}

impl Default for SVMotionStyle {
    fn default() -> Self {
        Self::instant()
    }
}
