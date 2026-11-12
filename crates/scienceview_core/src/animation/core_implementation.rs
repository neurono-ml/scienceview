//! `SVAnimation`: the model-owned clock and its keyframe window.

use std::collections::VecDeque;

use crate::sv_figure::SVExecutionMode;

use super::animation_span::sv_run_animation_operation;
use super::sampling::sv_sample_keyframes;
use super::{
    SVAnimationDirection, SVAnimationLoop, SVDuration, SVInterpolation, SVKeyframe,
    SVKeyframeDelta, SVKeyframeSource, SVPlaybackState,
};

/// Everything the builder hands over once configuration is validated.
pub(crate) struct SVAnimationSetup {
    pub(crate) source: SVKeyframeSource,
    pub(crate) keyframes: Vec<SVKeyframe>,
    pub(crate) capacity: usize,
    pub(crate) interpolation: SVInterpolation,
    pub(crate) loop_mode: SVAnimationLoop,
    pub(crate) direction: SVAnimationDirection,
    pub(crate) speed: f64,
    pub(crate) execution_mode: SVExecutionMode,
}

/// A playable sequence of figure states.
///
/// The timeline owns the keyframes, the clock and the playback controls, and
/// every backend renders [`SVAnimation::sample`]; time only moves when the
/// caller supplies it through [`SVAnimation::advance`]. A fixed list and a
/// stream share the same structure: the source decides how keyframes arrive,
/// never how they are sampled.
#[derive(Clone, Debug)]
pub struct SVAnimation {
    pub(super) source: SVKeyframeSource,
    pub(super) keyframes: VecDeque<SVKeyframe>,
    pub(super) capacity: usize,
    pub(super) interpolation: SVInterpolation,
    pub(super) loop_mode: SVAnimationLoop,
    pub(super) direction: SVAnimationDirection,
    pub(super) speed: f64,
    pub(super) execution_mode: SVExecutionMode,
    pub(super) state: SVPlaybackState,
    pub(super) position: f64,
    pub(super) pass: u32,
    pub(super) pass_direction: i8,
    pub(super) accepted_keyframes: u64,
    pub(super) duplicate_keyframes: u64,
    pub(super) out_of_order_keyframes: u64,
    pub(super) evicted_keyframes: u64,
}

impl SVAnimation {
    pub(crate) fn sv_from_setup(setup: SVAnimationSetup) -> Self {
        let pass_direction = match setup.direction {
            SVAnimationDirection::Reverse => -1,
            SVAnimationDirection::Forward | SVAnimationDirection::Alternate => 1,
        };
        Self {
            source: setup.source,
            keyframes: setup.keyframes.into(),
            capacity: setup.capacity,
            interpolation: setup.interpolation,
            loop_mode: setup.loop_mode,
            direction: setup.direction,
            speed: setup.speed,
            execution_mode: setup.execution_mode,
            state: SVPlaybackState::Idle,
            position: 0.0,
            pass: 0,
            pass_direction,
            accepted_keyframes: 0,
            duplicate_keyframes: 0,
            out_of_order_keyframes: 0,
            evicted_keyframes: 0,
        }
    }

    /// Returns the state at the current clock position.
    pub fn sample(&self) -> Option<SVKeyframeDelta> {
        sv_run_animation_operation(
            "sample",
            self.source,
            self.keyframes.len(),
            self.position,
            || {
                Ok(sv_sample_keyframes(
                    &self.keyframes,
                    self.position,
                    self.interpolation,
                ))
            },
        )
        .unwrap_or_default()
    }

    /// Returns the state the clock would show at an explicit position.
    pub fn sample_at(&self, position: SVDuration) -> Option<SVKeyframeDelta> {
        sv_run_animation_operation(
            "sample_at",
            self.source,
            self.keyframes.len(),
            position.as_seconds(),
            || {
                Ok(sv_sample_keyframes(
                    &self.keyframes,
                    position.as_seconds(),
                    self.interpolation,
                ))
            },
        )
        .unwrap_or_default()
    }
}
