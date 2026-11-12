//! The closed playback vocabulary: state, loop, direction and easing.

use core::fmt;

/// Where the playback clock currently stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVPlaybackState {
    /// The timeline has never been played, or it was stopped.
    #[default]
    Idle,
    /// The clock advances on every [`advance`](super::SVAnimation::advance).
    Playing,
    /// The clock holds its position; a stream does not re-target it.
    Paused,
    /// A fixed timeline reached its end and has no pass left.
    Finished,
}

impl SVPlaybackState {
    /// Returns the lowercase name used in spans and actions.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Playing => "playing",
            Self::Paused => "paused",
            Self::Finished => "finished",
        }
    }
}

impl fmt::Display for SVPlaybackState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// How often the timeline replays itself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVAnimationLoop {
    /// Play once and finish.
    #[default]
    None,
    /// Play the timeline `n` times in total.
    Count(u32),
    /// Never finish.
    Forever,
}

impl SVAnimationLoop {
    /// Returns the number of passes the loop allows, if it is bounded.
    pub fn total_passes(self) -> Option<u32> {
        match self {
            Self::None => Some(1),
            Self::Count(count) => Some(count.max(1)),
            Self::Forever => None,
        }
    }
}

/// In which order the timeline visits its keyframes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVAnimationDirection {
    /// From the first keyframe to the last.
    #[default]
    Forward,
    /// From the last keyframe to the first.
    Reverse,
    /// Forward on one pass, backward on the next.
    Alternate,
}

/// How the state between two neighbouring keyframes is computed.
///
/// The curve is applied to the segment progress before the two deltas are
/// interpolated, so every backend reading [`sample`](super::SVAnimation::sample)
/// sees the same easing without owning a copy of it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVInterpolation {
    /// Constant progress.
    #[default]
    Linear,
    /// Slow start, fast end.
    EaseIn,
    /// Fast start, slow end.
    EaseOut,
    /// Slow at both ends.
    EaseInOut,
    /// Hold the start state and swap at the end of the segment.
    Step,
}

impl SVInterpolation {
    /// Applies the curve to a progress in the unit interval.
    pub fn ease(self, progress: f64) -> f64 {
        let progress = if progress.is_finite() {
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        };
        match self {
            Self::Linear => progress,
            Self::EaseIn => progress * progress,
            Self::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Self::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            Self::Step => {
                if progress >= 1.0 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}
