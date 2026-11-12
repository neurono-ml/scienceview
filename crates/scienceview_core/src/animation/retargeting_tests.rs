//! Companion tests for interruptible re-targeting.

use std::time::Duration;

use crate::sv_animation::{
    SVAnimation, SVAnimationBuilder, SVDuration, SVKeyframe, SVKeyframeAdmission, SVKeyframeDelta,
    SVKeyframeSource,
};
use crate::sv_mark::SVMarkProperty;

fn sv_position(value: f64) -> SVKeyframeDelta {
    SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, value)]).expect("finite values")
}

fn sv_seconds(value: f64) -> SVDuration {
    SVDuration::from_seconds(value).expect("finite, non-negative seconds")
}

fn sv_frame(milliseconds: u64, value: f64) -> SVKeyframe {
    SVKeyframe::new(SVDuration::milliseconds(milliseconds), sv_position(value))
}

fn sv_streaming(capacity: usize) -> SVAnimation {
    SVAnimationBuilder::new()
        .source(SVKeyframeSource::Streaming)
        .capacity(capacity)
        .build()
        .expect("a streaming timeline builds")
}

fn sv_sample_at(animation: &SVAnimation, seconds: f64) -> Option<f64> {
    animation
        .sample_at(sv_seconds(seconds))
        .and_then(|delta| delta.value_of(SVMarkProperty::Position))
}

#[test]
fn a_keyframe_arriving_mid_transition_retargets_without_a_jump() {
    let mut animation = sv_streaming(8);
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    animation.play();
    animation.advance(Duration::from_millis(500));
    let before = sv_sample_at(&animation, 0.5).expect("the interpolated position");

    assert_eq!(
        animation.push_keyframe(sv_frame(750, 100.0)),
        Ok(SVKeyframeAdmission::Accepted)
    );

    assert_eq!(
        sv_sample_at(&animation, 0.5),
        Some(before),
        "the marks continue from where they are"
    );
    assert_eq!(animation.length(), 3, "the pending target was pre-empted");
    assert_eq!(
        animation
            .retained_keyframes()
            .0
            .last()
            .expect("the new target is retained")
            .timestamp()
            .as_duration()
            .as_millis(),
        750
    );

    animation.advance(Duration::from_millis(250));
    assert_eq!(sv_sample_at(&animation, 0.75), Some(100.0));
}

#[test]
fn an_interrupting_keyframe_that_is_not_newer_is_dropped() {
    let mut animation = sv_streaming(8);
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    animation.play();
    animation.advance(Duration::from_millis(500));

    assert_eq!(
        animation.push_keyframe(sv_frame(400, 99.0)),
        Ok(SVKeyframeAdmission::DroppedOutOfOrder)
    );
    assert_eq!(animation.length(), 2);
}
