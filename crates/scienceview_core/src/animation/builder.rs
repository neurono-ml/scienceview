//! The builder: the only public way to assemble an animation timeline.

use crate::sv_figure::SVExecutionMode;

use super::{
    SVAnimationDirection, SVAnimationLoop, SVDuration, SVInterpolation, SVKeyframe,
    SVKeyframeDelta, SVKeyframeSource,
};

/// Builds an [`SVAnimation`](super::SVAnimation).
///
/// The source is a deliberate choice: [`SVKeyframeSource::Fixed`] takes its
/// keyframes up front and validates their order once, at
/// [`build`](SVAnimationBuilder::build), while [`SVKeyframeSource::Streaming`]
/// starts empty and accepts arrivals through
/// [`SVAnimation::push_keyframe`].
///
/// ```
/// use scienceview_core::sv_animation::{SVAnimationBuilder, SVKeyframeDelta};
/// use scienceview_core::sv_mark::SVMarkProperty;
///
/// let delta = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Opacity, 1.0)])
///     .expect("finite values");
/// let animation = SVAnimationBuilder::new()
///     .deltas([delta])
///     .build()
///     .expect("a fixed timeline builds");
///
/// assert_eq!(animation.length(), 1);
/// ```
#[derive(Clone, Debug)]
pub struct SVAnimationBuilder {
    pub(super) source: SVKeyframeSource,
    pub(super) keyframes: Vec<SVKeyframe>,
    pub(super) duration: SVDuration,
    pub(super) capacity: usize,
    pub(super) interpolation: SVInterpolation,
    pub(super) loop_mode: SVAnimationLoop,
    pub(super) direction: SVAnimationDirection,
    pub(super) speed: f64,
    pub(super) execution_mode: SVExecutionMode,
}

impl SVAnimationBuilder {
    /// Starts a fixed timeline with the documented defaults.
    pub fn new() -> Self {
        Self {
            source: SVKeyframeSource::Fixed,
            keyframes: Vec::new(),
            duration: SVDuration::default(),
            capacity: 1024,
            interpolation: SVInterpolation::Linear,
            loop_mode: SVAnimationLoop::None,
            direction: SVAnimationDirection::Forward,
            speed: 1.0,
            execution_mode: SVExecutionMode::Automatic,
        }
    }

    /// Chooses how keyframes arrive.
    #[must_use]
    pub fn source(mut self, source: SVKeyframeSource) -> Self {
        self.source = source;
        self
    }

    /// Appends deltas, one default duration apart, starting at the current end.
    #[must_use]
    pub fn deltas(mut self, deltas: impl IntoIterator<Item = SVKeyframeDelta>) -> Self {
        for delta in deltas {
            let timestamp = self.sv_next_timestamp();
            self.keyframes.push(SVKeyframe::new(timestamp, delta));
        }
        self
    }

    /// Appends a keyframe with an explicit timestamp.
    #[must_use]
    pub fn keyframe(mut self, keyframe: SVKeyframe) -> Self {
        self.keyframes.push(keyframe);
        self
    }

    /// Configures the default duration used by subsequent [`deltas`](SVAnimationBuilder::deltas) calls.
    #[must_use]
    pub fn duration(mut self, duration: SVDuration) -> Self {
        self.duration = duration;
        self
    }

    /// Configures how many keyframes a streaming source retains.
    #[must_use]
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    /// Configures the curve applied inside every segment.
    #[must_use]
    pub fn interpolation(mut self, interpolation: SVInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Configures how often the timeline replays.
    #[must_use]
    pub fn loop_mode(mut self, loop_mode: SVAnimationLoop) -> Self {
        self.loop_mode = loop_mode;
        self
    }

    /// Configures the direction of travel.
    #[must_use]
    pub fn direction(mut self, direction: SVAnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Configures the playback multiplier.
    #[must_use]
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Configures the execution mode, which defaults to
    /// [`SVExecutionMode::Automatic`].
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }
}

impl Default for SVAnimationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
