//! The playback controls: play, pause, stop, seek, step, speed and dispatch.

use crate::sv_error::SVError;

use super::animation_span::sv_run_animation_operation;
use super::{SVAnimation, SVAnimationAction, SVPlaybackState};

impl SVAnimation {
    /// Advances the clock by the elapsed time the caller measured.
    ///
    /// Nothing moves unless the timeline is playing. The elapsed time is
    /// multiplied by the speed and by the current direction; a finished fixed
    /// timeline stays at its last frame, and a streaming timeline waits at its
    /// newest keyframe for the next arrival.
    pub fn advance(&mut self, elapsed: std::time::Duration) -> SVPlaybackState {
        let advanced = sv_run_animation_operation(
            "advance",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                self.sv_advance_elapsed(elapsed);
                Ok(())
            },
        );
        debug_assert!(advanced.is_ok(), "advancing never fails");
        self.state
    }

    /// Starts or resumes playback.
    pub fn play(&mut self) {
        let _ = sv_run_animation_operation(
            "play",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                if matches!(
                    self.state,
                    SVPlaybackState::Idle | SVPlaybackState::Finished
                ) {
                    self.sv_restart();
                }
                self.state = SVPlaybackState::Playing;
                Ok(())
            },
        );
    }

    /// Holds the current position; an arriving stream does not re-target it.
    pub fn pause(&mut self) {
        let _ = sv_run_animation_operation(
            "pause",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                if self.state == SVPlaybackState::Playing {
                    self.state = SVPlaybackState::Paused;
                }
                Ok(())
            },
        );
    }

    /// Returns to the start and becomes idle.
    pub fn stop(&mut self) {
        let _ = sv_run_animation_operation(
            "stop",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                self.sv_restart();
                self.position = 0.0;
                self.state = SVPlaybackState::Idle;
                Ok(())
            },
        );
    }

    /// Changes the playback multiplier.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidPlaybackSpeed`] when the speed is zero,
    /// negative or not finite.
    pub fn set_speed(&mut self, speed: f64) -> Result<(), SVError> {
        sv_run_animation_operation(
            "set_speed",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                self.speed = Self::sv_validated_speed(speed)?;
                Ok(())
            },
        )
    }

    /// Applies one playback action.
    ///
    /// # Errors
    ///
    /// Propagates the configuration errors of the action it carries, such as
    /// [`SVError::InvalidPlaybackSpeed`].
    pub fn dispatch(&mut self, action: SVAnimationAction) -> Result<(), SVError> {
        sv_run_animation_operation(
            "dispatch",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                match action {
                    SVAnimationAction::Play => self.play(),
                    SVAnimationAction::Pause => self.pause(),
                    SVAnimationAction::Toggle => {
                        if self.state == SVPlaybackState::Playing {
                            self.pause();
                        } else {
                            self.play();
                        }
                    }
                    SVAnimationAction::Stop => self.stop(),
                    SVAnimationAction::Step { forward } => self.sv_step_to_neighbour(forward),
                    SVAnimationAction::Seek(position) => self.sv_seek_without_span(position),
                    SVAnimationAction::SetSpeed(speed) => {
                        self.speed = Self::sv_validated_speed(speed)?;
                    }
                    SVAnimationAction::SetDirection(direction) => {
                        self.direction = direction;
                        self.pass = 0;
                        self.pass_direction = self.sv_initial_sign();
                    }
                }
                Ok(())
            },
        )
    }
}
