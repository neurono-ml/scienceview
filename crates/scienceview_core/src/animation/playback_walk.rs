//! Walking the playback clock across passes, directions and loops.

use std::time::Duration;

use super::{
    SVAnimation, SVAnimationDirection, SVAnimationLoop, SVKeyframeSource, SVPlaybackState,
};

impl SVAnimation {
    /// Returns the timestamp of the newest retained keyframe, in seconds.
    pub(super) fn sv_end_seconds(&self) -> f64 {
        self.keyframes
            .back()
            .map(|keyframe| keyframe.timestamp().as_seconds())
            .unwrap_or(0.0)
    }

    /// Returns the direction sign of the current pass.
    pub(super) fn sv_initial_sign(&self) -> i8 {
        match self.direction {
            SVAnimationDirection::Reverse => -1,
            SVAnimationDirection::Forward => 1,
            SVAnimationDirection::Alternate => {
                if self.pass % 2 == 0 {
                    1
                } else {
                    -1
                }
            }
        }
    }

    /// Returns to the first frame of a fresh playback.
    pub(super) fn sv_restart(&mut self) {
        self.pass = 0;
        self.pass_direction = match self.direction {
            SVAnimationDirection::Reverse => -1,
            SVAnimationDirection::Forward | SVAnimationDirection::Alternate => 1,
        };
        self.position = if self.pass_direction > 0 {
            0.0
        } else {
            self.sv_end_seconds()
        };
    }

    pub(super) fn sv_advance_elapsed(&mut self, elapsed: Duration) {
        if self.state != SVPlaybackState::Playing {
            return;
        }
        let distance = elapsed.as_secs_f64() * self.speed;
        if !distance.is_finite() || distance <= 0.0 {
            return;
        }
        let end = self.sv_end_seconds();
        if end <= 0.0 || self.keyframes.len() < 2 {
            if self.source == SVKeyframeSource::Fixed {
                self.state = SVPlaybackState::Finished;
            }
            return;
        }
        self.sv_walk(distance, end);
    }

    fn sv_can_start_next_pass(&self) -> bool {
        self.loop_mode
            .total_passes()
            .is_none_or(|total| self.pass + 1 < total)
    }

    fn sv_remaining_passes(&self) -> u32 {
        match self.loop_mode.total_passes() {
            Some(total) => total.saturating_sub(self.pass + 1),
            None => u32::MAX,
        }
    }

    fn sv_walk(&mut self, mut distance: f64, end: f64) {
        loop {
            let sign = f64::from(self.pass_direction);
            let remaining = if sign > 0.0 {
                end - self.position
            } else {
                self.position
            };
            if distance < remaining {
                self.position = (self.position + sign * distance).clamp(0.0, end);
                return;
            }

            distance -= remaining;
            self.position = if sign > 0.0 { end } else { 0.0 };
            if !self.sv_can_start_next_pass() {
                if self.source == SVKeyframeSource::Fixed {
                    self.state = SVPlaybackState::Finished;
                }
                return;
            }

            self.pass += 1;
            self.pass_direction = self.sv_initial_sign();
            self.position = if self.pass_direction > 0 { 0.0 } else { end };
            if distance <= 0.0 {
                return;
            }
            self.sv_skip_whole_passes(&mut distance, end);
        }
    }

    fn sv_skip_whole_passes(&mut self, distance: &mut f64, end: f64) {
        if *distance < end {
            return;
        }
        let whole = (*distance / end).floor();
        let whole = if whole.is_finite() && whole >= 1.0 {
            whole as u64
        } else {
            return;
        };
        let skipped = match self.loop_mode {
            SVAnimationLoop::Forever => whole,
            SVAnimationLoop::Count(_) => whole.min(u64::from(self.sv_remaining_passes())),
            SVAnimationLoop::None => 0,
        };
        if skipped == 0 {
            return;
        }
        self.pass = self.pass.saturating_add(skipped as u32);
        self.pass_direction = self.sv_initial_sign();
        self.position = if self.pass_direction > 0 { 0.0 } else { end };
        *distance -= skipped as f64 * end;
    }
}
