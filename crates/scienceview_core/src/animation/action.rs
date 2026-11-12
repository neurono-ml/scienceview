//! The animation slice of the semantic action vocabulary.

use super::{SVAnimationDirection, SVDuration};

/// One playback command, independent of the control that produced it.
///
/// The UI kit resolves a gesture — a button, a key, a scrub slider — into one
/// of these actions and dispatches it through
/// [`SVAnimation::dispatch`](super::SVAnimation::dispatch), the same way it
/// dispatches every other action of the model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVAnimationAction {
    /// Start or resume playback from the current position.
    Play,
    /// Hold the current position; an arriving stream does not re-target it.
    Pause,
    /// Pause a playing timeline and play a paused one.
    Toggle,
    /// Return to the start and become idle.
    Stop,
    /// Pause and move to the neighbouring keyframe.
    Step {
        /// Whether to step towards the end of the timeline.
        forward: bool,
    },
    /// Move the clock to an explicit position, clamped to the timeline.
    Seek(SVDuration),
    /// Change the playback multiplier.
    SetSpeed(f64),
    /// Change the direction of travel.
    SetDirection(SVAnimationDirection),
}
