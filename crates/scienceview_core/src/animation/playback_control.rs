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
                self.sv_play_without_span();
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
                self.sv_pause_without_span();
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
                self.sv_stop_without_span();
                Ok(())
            },
        );
    }

    /// Starts playback unless it is already playing.
    pub(super) fn sv_play_without_span(&mut self) {
        if matches!(
            self.state,
            SVPlaybackState::Idle | SVPlaybackState::Finished
        ) {
            self.sv_restart();
        }
        self.state = SVPlaybackState::Playing;
    }

    /// Holds the position unless it is already held.
    pub(super) fn sv_pause_without_span(&mut self) {
        if self.state == SVPlaybackState::Playing {
            self.state = SVPlaybackState::Paused;
        }
    }

    /// Returns to the start and becomes idle.
    pub(super) fn sv_stop_without_span(&mut self) {
        self.sv_restart();
        self.position = 0.0;
        self.state = SVPlaybackState::Idle;
    }

    /// Pauses a playing timeline and plays a paused one.
    pub(super) fn sv_toggle_without_span(&mut self) {
        if self.state == SVPlaybackState::Playing {
            self.sv_pause_without_span();
        } else {
            self.sv_play_without_span();
        }
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
                    SVAnimationAction::Play => self.sv_play_without_span(),
                    SVAnimationAction::Pause => self.sv_pause_without_span(),
                    SVAnimationAction::Toggle => self.sv_toggle_without_span(),
                    SVAnimationAction::Stop => self.sv_stop_without_span(),
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
