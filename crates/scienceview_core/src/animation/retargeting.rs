//! Re-targeting a running transition from its current interpolated position.

use std::time::Duration;

use super::sampling::sv_sample_keyframes;
use super::{SVAnimation, SVDuration, SVKeyframe};

impl SVAnimation {
    pub(super) fn sv_is_mid_transition(&self) -> bool {
        let (Some(first), Some(last)) = (self.keyframes.front(), self.keyframes.back()) else {
            return false;
        };
        if self.keyframes.len() < 2 {
            return false;
        }
        let first = first.timestamp().as_seconds();
        let last = last.timestamp().as_seconds();
        self.position > first && self.position < last
    }

    pub(super) fn sv_retarget_from_current_position(&mut self) {
        let Some(origin) = sv_sample_keyframes(&self.keyframes, self.position, self.interpolation)
        else {
            return;
        };
        while let Some(last) = self.keyframes.back() {
            if last.timestamp().as_seconds() > self.position {
                self.keyframes.pop_back();
            } else {
                break;
            }
        }
        let needs_origin = self
            .keyframes
            .back()
            .map(|last| last.timestamp().as_seconds() < self.position)
            .unwrap_or(true);
        if needs_origin {
            let timestamp = SVDuration::sv_from_duration(Duration::from_secs_f64(self.position));
            self.keyframes.push_back(SVKeyframe::new(timestamp, origin));
        }
    }
}
