//! Seeking and stepping the playback clock.

use crate::sv_error::SVError;

use super::animation_span::sv_run_animation_operation;
use super::{SVAnimation, SVDuration, SVPlaybackState};

impl SVAnimation {
    /// Moves the clock to an explicit position, clamped to the timeline.
    ///
    /// Seeking a finished timeline leaves it paused at the new position, so the
    /// next play resumes there instead of restarting.
    pub fn seek(&mut self, position: SVDuration) {
        let _ = sv_run_animation_operation(
            "seek",
            self.source,
            self.keyframes.len(),
            position.as_seconds(),
            || {
                self.sv_seek_without_span(position);
                Ok(())
            },
        );
    }

    /// Pauses and moves to the neighbouring keyframe.
    ///
    /// # Errors
    ///
    /// This operation is infallible today; the result shape keeps room for a
    /// future source that cannot be stepped.
    pub fn step(&mut self, forward: bool) -> Result<(), SVError> {
        sv_run_animation_operation(
            "step",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                if self.state == SVPlaybackState::Playing {
                    self.state = SVPlaybackState::Paused;
                }
                self.sv_step_to_neighbour(forward);
                Ok(())
            },
        )
    }

    pub(super) fn sv_seek_without_span(&mut self, position: SVDuration) {
        self.position = position.as_seconds().clamp(0.0, self.sv_end_seconds());
        if matches!(
            self.state,
            SVPlaybackState::Idle | SVPlaybackState::Finished
        ) {
            self.state = SVPlaybackState::Paused;
        }
    }

    pub(super) fn sv_step_to_neighbour(&mut self, forward: bool) {
        let step_tolerance = 1e-9;
        let current = self.position;
        let target = if forward {
            self.keyframes
                .iter()
                .find(|keyframe| keyframe.timestamp().as_seconds() > current + step_tolerance)
                .map(|keyframe| keyframe.timestamp().as_seconds())
        } else {
            self.keyframes
                .iter()
                .rev()
                .find(|keyframe| keyframe.timestamp().as_seconds() < current - step_tolerance)
                .map(|keyframe| keyframe.timestamp().as_seconds())
        };
        if let Some(target) = target {
            self.position = target;
        }
    }

    pub(super) fn sv_validated_speed(speed: f64) -> Result<f64, SVError> {
        if !speed.is_finite() || speed <= 0.0 {
            return Err(SVError::InvalidPlaybackSpeed);
        }
        Ok(speed)
    }
}
