//! Admitting keyframes into a streaming timeline's bounded window.

use crate::sv_error::SVError;

use super::animation_span::sv_run_animation_operation;
use super::{SVAnimation, SVKeyframe, SVKeyframeAdmission, SVKeyframeSource, SVPlaybackState};

impl SVAnimation {
    /// Admits one keyframe into a streaming timeline.
    ///
    /// A duplicate or out-of-order timestamp is dropped and counted rather
    /// than failing the live path; the timeline then retains at most
    /// [`capacity`](SVAnimation::capacity) keyframes, evicting the oldest.
    ///
    /// A keyframe delivered while the timeline is playing strictly inside a
    /// transition pre-empts the pending target: the interpolated state at the
    /// current position becomes a keyframe, the future targets are dropped and
    /// the new keyframe becomes the destination, so the marks continue from
    /// where they are.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::AnimationSourceMismatch`] when the timeline was
    /// built on a fixed source.
    pub fn push_keyframe(&mut self, keyframe: SVKeyframe) -> Result<SVKeyframeAdmission, SVError> {
        sv_run_animation_operation(
            "push_keyframe",
            self.source,
            self.keyframes.len(),
            self.position,
            || self.sv_admit_keyframe(keyframe),
        )
    }

    fn sv_admit_keyframe(&mut self, keyframe: SVKeyframe) -> Result<SVKeyframeAdmission, SVError> {
        if self.source != SVKeyframeSource::Streaming {
            return Err(SVError::AnimationSourceMismatch {
                operation: "push_keyframe",
                keyframe_source: self.source,
            });
        }

        let arrived = keyframe.timestamp().as_seconds();
        let pre_empts = self.state == SVPlaybackState::Playing
            && self.sv_is_mid_transition()
            && arrived > self.position;
        if pre_empts {
            self.sv_retarget_from_current_position();
        } else if let Some(newest) = self.keyframes.back() {
            let newest = newest.timestamp().as_seconds();
            if arrived < newest {
                self.out_of_order_keyframes += 1;
                return Ok(SVKeyframeAdmission::DroppedOutOfOrder);
            }
            if arrived == newest {
                self.duplicate_keyframes += 1;
                return Ok(SVKeyframeAdmission::DroppedDuplicate);
            }
        }

        self.keyframes.push_back(keyframe);
        self.accepted_keyframes += 1;
        self.sv_retain_bounded();
        Ok(SVKeyframeAdmission::Accepted)
    }

    fn sv_retain_bounded(&mut self) {
        while self.keyframes.len() > self.capacity {
            self.keyframes.pop_front();
            self.evicted_keyframes += 1;
        }
    }
}
