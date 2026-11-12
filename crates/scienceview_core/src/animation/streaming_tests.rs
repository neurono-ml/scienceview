//! Companion tests for the streaming keyframe source, its window and re-targeting.

use std::time::Duration;

use crate::sv_animation::{
    SVAnimation, SVAnimationBuilder, SVDuration, SVKeyframe, SVKeyframeAdmission, SVKeyframeDelta,
    SVKeyframeSource, SVPlaybackState,
};
use crate::sv_error::SVError;
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
fn a_streaming_timeline_starts_empty_and_playing_it_changes_nothing() {
    let mut animation = sv_streaming(4);
    animation.play();
    animation.advance(Duration::from_millis(100));

    assert_eq!(animation.source(), SVKeyframeSource::Streaming);
    assert_eq!(animation.length(), 0);
    assert!(animation.sample().is_none());
    assert_eq!(animation.state(), SVPlaybackState::Playing);
}

#[test]
fn a_streaming_source_accepts_increasing_keyframes() {
    let mut animation = sv_streaming(8);

    assert_eq!(
        animation.push_keyframe(sv_frame(0, 0.0)),
        Ok(SVKeyframeAdmission::Accepted)
    );
    assert_eq!(
        animation.push_keyframe(sv_frame(1000, 10.0)),
        Ok(SVKeyframeAdmission::Accepted)
    );
    assert_eq!(animation.length(), 2);
    assert_eq!(animation.accepted_keyframes(), 2);
    assert_eq!(sv_sample_at(&animation, 0.5), Some(5.0));
}

#[test]
fn a_streaming_source_drops_a_duplicate_timestamp() {
    let mut animation = sv_streaming(8);
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");

    assert_eq!(
        animation.push_keyframe(sv_frame(1000, 99.0)),
        Ok(SVKeyframeAdmission::DroppedDuplicate)
    );
    assert_eq!(animation.length(), 2);
    assert_eq!(animation.duplicate_keyframes(), 1);
    assert_eq!(sv_sample_at(&animation, 1.0), Some(10.0));
}

#[test]
fn a_streaming_source_drops_an_out_of_order_timestamp() {
    let mut animation = sv_streaming(8);
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");

    assert_eq!(
        animation.push_keyframe(sv_frame(500, 99.0)),
        Ok(SVKeyframeAdmission::DroppedOutOfOrder)
    );
    assert_eq!(animation.out_of_order_keyframes(), 1);
    assert_eq!(animation.length(), 2);
}

#[test]
fn the_keyframe_window_stays_bounded_under_a_long_stream() {
    let mut animation = sv_streaming(8);

    for index in 0..10_000u64 {
        assert_eq!(
            animation.push_keyframe(sv_frame(index * 10, index as f64)),
            Ok(SVKeyframeAdmission::Accepted)
        );
    }

    assert_eq!(animation.length(), 8);
    assert_eq!(animation.evicted_keyframes(), 9_992);
    assert_eq!(animation.accepted_keyframes(), 10_000);
    assert_eq!(
        animation
            .retained_keyframes()
            .0
            .last()
            .expect("the newest frame is retained")
            .timestamp()
            .as_duration()
            .as_millis(),
        99_990
    );
}

#[test]
fn a_paused_streaming_timeline_ignores_an_arriving_keyframe() {
    let mut animation = sv_streaming(8);
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    animation.play();
    animation.advance(Duration::from_millis(500));
    animation.pause();
    let position = animation.position_seconds();
    let sample = sv_sample_at(&animation, 0.5);

    assert_eq!(
        animation.push_keyframe(sv_frame(2000, 100.0)),
        Ok(SVKeyframeAdmission::Accepted)
    );

    assert_eq!(animation.position_seconds(), position);
    assert_eq!(sv_sample_at(&animation, 0.5), sample);
    assert_eq!(animation.length(), 3);
}

#[test]
fn the_streaming_timeline_never_finishes() {
    let mut animation = sv_streaming(8);
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    animation.play();

    animation.advance(Duration::from_secs(100));

    assert_eq!(animation.state(), SVPlaybackState::Playing);
    assert!((animation.position_seconds() - 1.0).abs() < 1e-12);
}

#[test]
fn a_fixed_source_rejects_a_streamed_keyframe() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0)])
        .build()
        .expect("a fixed timeline builds");

    assert_eq!(
        animation.push_keyframe(sv_frame(1000, 1.0)),
        Err(SVError::AnimationSourceMismatch {
            operation: "push_keyframe",
            keyframe_source: SVKeyframeSource::Fixed,
        })
    );
}

#[test]
fn a_streaming_source_rejects_a_preloaded_keyframe() {
    assert_eq!(
        SVAnimationBuilder::new()
            .source(SVKeyframeSource::Streaming)
            .deltas([sv_position(0.0)])
            .build()
            .err(),
        Some(SVError::InvalidConfiguration {
            message: "a streaming keyframe source starts empty; push keyframes instead".to_owned(),
        })
    );
}

#[test]
fn the_window_needs_room_for_a_segment() {
    assert_eq!(
        SVAnimationBuilder::new()
            .source(SVKeyframeSource::Streaming)
            .capacity(1)
            .build()
            .err(),
        Some(SVError::InvalidAnimationCapacity { capacity: 1 })
    );
}

#[test]
fn fixed_and_streaming_sources_sample_identically() {
    let mut fixed = SVAnimationBuilder::new()
        .keyframe(sv_frame(0, 0.0))
        .keyframe(sv_frame(1000, 10.0))
        .keyframe(sv_frame(2000, 5.0))
        .build()
        .expect("a fixed timeline builds");
    let mut streaming = sv_streaming(8);
    streaming
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    streaming
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    streaming
        .push_keyframe(sv_frame(2000, 5.0))
        .expect("the third frame");

    for seconds in [0.0, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0] {
        assert_eq!(
            sv_sample_at(&fixed, seconds),
            sv_sample_at(&streaming, seconds),
            "the sources must agree at {seconds} s"
        );
    }

    fixed.play();
    streaming.play();
    for _ in 0..20 {
        fixed.advance(Duration::from_millis(125));
        streaming.advance(Duration::from_millis(125));
        assert_eq!(
            fixed.position_seconds(),
            streaming.position_seconds(),
            "the clocks must agree"
        );
        assert_eq!(
            fixed
                .sample()
                .and_then(|delta| delta.value_of(SVMarkProperty::Position)),
            streaming
                .sample()
                .and_then(|delta| delta.value_of(SVMarkProperty::Position))
        );
    }

    assert_eq!(fixed.state(), SVPlaybackState::Finished);
    assert_eq!(
        streaming.state(),
        SVPlaybackState::Playing,
        "a stream waits for the next arrival instead of finishing"
    );
}
