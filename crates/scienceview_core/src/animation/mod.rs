//! Animation: one timeline, two keyframe sources, one model-owned clock
//! (`FDR-006`).
//!
//! [`SVAnimation`] turns a sequence of figure states into a playable timeline.
//! The states are [`SVKeyframeDelta`] values expressed with the property
//! vocabulary [`sv_mark`](crate::sv_mark) exports, so animation decides *when*
//! a property changes while the transition kernel decides *whether* and *how*
//! it tweens ([`SVKeyframeDelta::interpolate`] is the seam: it takes the two
//! neighbouring deltas and the eased progress a transition produces).
//!
//! ## One clock, in the model
//!
//! Playback state is model state (design D1): [`SVAnimation`] owns the position,
//! the speed, the loop mode and the direction, and every backend renders the
//! same [`SVAnimation::sample`]. Time is supplied by the caller through
//! [`SVAnimation::advance`] — there is no wall clock, no timer and no async
//! runtime, so the same sequence of ticks always produces the same frames.
//!
//! ```
//! use std::time::Duration;
//! use scienceview_core::sv_animation::{
//!     SVAnimationBuilder, SVAnimationLoop, SVKeyframeDelta, SVPlaybackState,
//! };
//! use scienceview_core::sv_mark::SVMarkProperty;
//!
//! let first = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 0.0)])
//!     .expect("finite values");
//! let second = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 10.0)])
//!     .expect("finite values");
//! let mut animation = SVAnimationBuilder::new()
//!     .deltas([first, second])
//!     .loop_mode(SVAnimationLoop::Forever)
//!     .build()
//!     .expect("a fixed timeline builds");
//!
//! animation.play();
//! animation.advance(Duration::from_millis(500));
//!
//! assert_eq!(animation.state(), SVPlaybackState::Playing);
//! assert_eq!(
//!     animation
//!         .sample()
//!         .and_then(|delta| delta.value_of(SVMarkProperty::Position)),
//!     Some(5.0)
//! );
//! ```
//!
//! ## Fixed and streaming sources are one structure
//!
//! [`SVKeyframeSource`] is `Fixed` or `Streaming` (design D3). A fixed list is
//! validated at build time — timestamps must be strictly increasing — while a
//! streaming timeline accepts keyframes through
//! [`SVAnimation::push_keyframe`] as they arrive, never blocking and never
//! growing without bound: the retained window holds at most
//! [`SVAnimation::capacity`] keyframes and drops the oldest deterministically.
//! The counters [`SVAnimation::accepted_keyframes`],
//! [`SVAnimation::duplicate_keyframes`], [`SVAnimation::out_of_order_keyframes`]
//! and [`SVAnimation::evicted_keyframes`] make every drop observable.
//!
//! ```text
//! Fixed:     A -- B -- C                     (validated once, at build)
//! Streaming: ... -- X -- Y -- Z -- (drop oldest, keep capacity)
//! ```
//!
//! ## Interruptible, never jumping
//!
//! A keyframe delivered while the timeline is playing strictly inside a
//! transition pre-empts the pending target (design D4): the state interpolated
//! at the current position is recorded as a keyframe, the future targets are
//! dropped, and the arriving keyframe becomes the new destination. Sampling
//! immediately before and after the delivery therefore agrees, because the new
//! segment starts exactly at the current state.
//!
//! ## Playback is drivable as actions
//!
//! [`SVAnimationAction`] carries the animation slice of the `model-events`
//! vocabulary, so the UI kit dispatches play, pause, toggle, stop, step, seek,
//! speed and direction exactly as it dispatches every other control.
//!
//! ## No backend selection
//!
//! An animation is backend-independent model state, not an artist: it carries
//! [`SVExecutionMode`](crate::sv_figure::SVExecutionMode) but no
//! `SVBackendKind`. The single span every operation emits records
//! `backend.kind = "backend-independent"`, the source kind, the retained
//! keyframe count, the position, the duration and the outcome.

mod action;
mod admission;
mod builder;
mod core_implementation;
mod duration;
mod keyframe;
mod playback;
mod sampling;
mod source;
mod animation_span;

pub use action::SVAnimationAction;
pub use admission::SVKeyframeAdmission;
pub use builder::SVAnimationBuilder;
pub use core_implementation::SVAnimation;
pub use duration::SVDuration;
pub use keyframe::{SVKeyframe, SVKeyframeChange, SVKeyframeDelta};
pub use playback::{
    SVAnimationDirection, SVAnimationLoop, SVInterpolation, SVPlaybackState,
};
pub use source::SVKeyframeSource;

#[cfg(test)]
#[path = "animation_concurrency_tests.rs"]
mod animation_concurrency_tests;
#[cfg(test)]
#[path = "animation_span_tests.rs"]
mod animation_span_tests;
#[cfg(test)]
#[path = "keyframe_tests.rs"]
mod keyframe_tests;
#[cfg(test)]
#[path = "streaming_tests.rs"]
mod streaming_tests;
#[cfg(test)]
#[path = "timeline_tests.rs"]
mod timeline_tests;
