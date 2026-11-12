//! Read-only access to the timeline's configuration and state.

use crate::sv_figure::SVExecutionMode;

use super::{
    SVAnimation, SVAnimationDirection, SVAnimationLoop, SVDuration, SVInterpolation, SVKeyframe,
    SVKeyframeSource, SVPlaybackState,
};

impl SVAnimation {
    /// Returns the kind of source that feeds this timeline.
    pub fn source(&self) -> SVKeyframeSource {
        self.source
    }

    /// Returns the current playback state.
    pub fn state(&self) -> SVPlaybackState {
        self.state
    }

    /// Returns the playback position on the model clock.
    pub fn position(&self) -> SVDuration {
        SVDuration::sv_from_duration(std::time::Duration::from_secs_f64(self.position))
    }

    /// Returns the playback position in seconds.
    pub fn position_seconds(&self) -> f64 {
        self.position
    }

    /// Returns the position as a fraction of the timeline, in `0.0..=1.0`.
    pub fn normalised_position(&self) -> f64 {
        let end = self.sv_end_seconds();
        if end <= 0.0 {
            0.0
        } else {
            (self.position / end).clamp(0.0, 1.0)
        }
    }

    /// Returns the playback multiplier.
    pub fn speed(&self) -> f64 {
        self.speed
    }

    /// Returns the configured loop mode.
    pub fn loop_mode(&self) -> SVAnimationLoop {
        self.loop_mode
    }

    /// Returns the configured direction.
    pub fn direction(&self) -> SVAnimationDirection {
        self.direction
    }

    /// Returns the configured interpolation curve.
    pub fn interpolation(&self) -> SVInterpolation {
        self.interpolation
    }

    /// Returns the execution mode carried from the builder.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.execution_mode
    }

    /// Returns the number of retained keyframes.
    pub fn length(&self) -> usize {
        self.keyframes.len()
    }

    /// Returns the maximum number of keyframes a streaming source retains.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns `true` when no keyframe is retained.
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Returns the retained keyframes as two borrowed chunks, oldest first.
    pub fn retained_keyframes(&self) -> (&[SVKeyframe], &[SVKeyframe]) {
        self.keyframes.as_slices()
    }

    /// Returns how many keyframes the timeline accepted.
    pub fn accepted_keyframes(&self) -> u64 {
        self.accepted_keyframes
    }

    /// Returns how many keyframes were dropped as duplicates.
    pub fn duplicate_keyframes(&self) -> u64 {
        self.duplicate_keyframes
    }

    /// Returns how many keyframes were dropped as out of order.
    pub fn out_of_order_keyframes(&self) -> u64 {
        self.out_of_order_keyframes
    }

    /// Returns how many keyframes left the bounded window.
    pub fn evicted_keyframes(&self) -> u64 {
        self.evicted_keyframes
    }
}
