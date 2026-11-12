//! Build-time validation of an animation configuration (patterns P7).

use crate::sv_error::SVError;

use super::builder::SVAnimationBuilder;
use super::core_implementation::SVAnimationSetup;
use super::{SVAnimation, SVAnimationLoop, SVDuration, SVKeyframe, SVKeyframeSource};

impl SVAnimationBuilder {
    /// Validates the configuration and builds the timeline.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidPlaybackSpeed`] for a speed that is zero,
    /// negative or not finite, [`SVError::EmptyKeyframeSource`] for a fixed
    /// source without keyframes, [`SVError::OutOfOrderKeyframes`] when fixed
    /// timestamps do not strictly increase,
    /// [`SVError::InvalidAnimationCapacity`] for a streaming window smaller
    /// than one segment, and [`SVError::InvalidConfiguration`] for a preloaded
    /// stream or a counted loop of zero passes.
    pub fn build(self) -> Result<SVAnimation, SVError> {
        if !self.speed.is_finite() || self.speed <= 0.0 {
            return Err(SVError::InvalidPlaybackSpeed);
        }
        if matches!(self.loop_mode, SVAnimationLoop::Count(0)) {
            return Err(SVError::InvalidConfiguration {
                message: "a counted animation loop needs at least one pass".to_owned(),
            });
        }

        match self.source {
            SVKeyframeSource::Fixed => {
                if self.keyframes.is_empty() {
                    return Err(SVError::EmptyKeyframeSource);
                }
                Self::sv_validate_order(&self.keyframes)?;
            }
            SVKeyframeSource::Streaming => {
                if !self.keyframes.is_empty() {
                    return Err(SVError::InvalidConfiguration {
                        message: "a streaming keyframe source starts empty; push keyframes instead"
                            .to_owned(),
                    });
                }
                if self.capacity < 2 {
                    return Err(SVError::InvalidAnimationCapacity {
                        capacity: self.capacity,
                    });
                }
            }
        }

        Ok(SVAnimation::sv_from_setup(SVAnimationSetup {
            source: self.source,
            keyframes: self.keyframes,
            capacity: self.capacity,
            interpolation: self.interpolation,
            loop_mode: self.loop_mode,
            direction: self.direction,
            speed: self.speed,
            execution_mode: self.execution_mode,
        }))
    }

    pub(super) fn sv_next_timestamp(&self) -> SVDuration {
        match self.keyframes.last() {
            Some(last) => last
                .timestamp()
                .as_duration()
                .checked_add(self.duration.as_duration())
                .map(SVDuration::sv_from_duration)
                .unwrap_or_else(|| last.timestamp()),
            None => SVDuration::zero(),
        }
    }

    fn sv_validate_order(keyframes: &[SVKeyframe]) -> Result<(), SVError> {
        for (position, window) in keyframes.windows(2).enumerate() {
            let previous = window[0].timestamp().as_duration();
            let found = window[1].timestamp().as_duration();
            if found <= previous {
                return Err(SVError::OutOfOrderKeyframes {
                    position: position + 1,
                    previous_milliseconds: previous.as_millis() as u64,
                    found_milliseconds: found.as_millis() as u64,
                });
            }
        }
        Ok(())
    }
}
