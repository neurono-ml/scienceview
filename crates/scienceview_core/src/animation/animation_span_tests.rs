//! Verifies the animation span fields through a real subscriber (patterns P8).
//!
//! Every test holds the shared span-test guard, so no parallel test can change
//! the process-wide callsite interest cache while a recorder is installed. The
//! cache is rebuilt once after the guard is held, which is enough because the
//! guard serializes every subscriber install.

use std::time::Duration;

use crate::sv_animation::{
    SVAnimation, SVAnimationAction, SVAnimationBuilder, SVAnimationDirection, SVDuration,
    SVKeyframe, SVKeyframeDelta, SVKeyframeSource,
};
use crate::sv_mark::SVMarkProperty;
use crate::sv_observability::span_test_support::{
    SVSpanFields, SVSpanRecorder, sv_span_test_guard,
};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

fn sv_frame(milliseconds: u64, value: f64) -> SVKeyframe {
    let delta =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, value)]).expect("finite values");
    SVKeyframe::new(SVDuration::milliseconds(milliseconds), delta)
}

fn sv_streaming() -> SVAnimation {
    let mut animation = SVAnimationBuilder::new()
        .source(SVKeyframeSource::Streaming)
        .capacity(4)
        .build()
        .expect("a streaming timeline builds");
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    animation
}

fn sv_capture_span(recorder: &SVSpanRecorder, operation: impl FnOnce()) -> Vec<SVSpanFields> {
    operation();
    recorder.captured()
}

/// Returns every playback action with a fresh value for the payload variants.
fn sv_actions() -> Vec<SVAnimationAction> {
    vec![
        SVAnimationAction::Play,
        SVAnimationAction::Pause,
        SVAnimationAction::Toggle,
        SVAnimationAction::Stop,
        SVAnimationAction::Step { forward: true },
        SVAnimationAction::Seek(SVDuration::milliseconds(500)),
        SVAnimationAction::SetSpeed(2.0),
        SVAnimationAction::SetDirection(SVAnimationDirection::Reverse),
    ]
}

#[test]
fn an_advance_emits_one_span_with_the_source_and_the_position() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let mut animation = sv_streaming();
    animation.play();

    let spans = tracing::subscriber::with_default(subscriber, || {
        sv_capture_span(&recorder, || {
            animation.advance(Duration::ZERO);
        })
    });

    assert_eq!(spans.len(), 1, "one span per advance: {spans:?}");
    let advance = &spans[0];
    assert_eq!(advance["operation"], "advance");
    assert_eq!(advance["backend.kind"], "backend-independent");
    assert_eq!(advance["keyframe.source"], "Streaming");
    assert_eq!(advance["keyframes"], "2");
    assert_eq!(advance["position"], "0");
    assert_eq!(advance["outcome"], "success");
    assert!(advance.contains_key("duration"), "{advance:?}");
}

#[test]
fn a_push_keyframe_emits_one_span() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let mut animation = sv_streaming();

    let spans = tracing::subscriber::with_default(subscriber, || {
        sv_capture_span(&recorder, || {
            animation
                .push_keyframe(sv_frame(1000, 10.0))
                .expect("a duplicate is dropped, not an error");
        })
    });

    assert_eq!(spans.len(), 1, "one span per arrival: {spans:?}");
    assert_eq!(spans[0]["operation"], "push_keyframe");
    assert_eq!(spans[0]["outcome"], "success");
}

#[test]
fn a_sample_emits_one_span() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let animation = sv_streaming();

    let spans = tracing::subscriber::with_default(subscriber, || {
        sv_capture_span(&recorder, || {
            let _ = animation.sample();
        })
    });

    assert_eq!(spans.len(), 1, "one span per sample: {spans:?}");
    assert_eq!(spans[0]["operation"], "sample");
    assert_eq!(spans[0]["backend.kind"], "backend-independent");
}

#[test]
fn every_dispatch_action_emits_exactly_one_span() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let mut animation = sv_streaming();

    tracing::subscriber::with_default(subscriber, || {
        for action in sv_actions() {
            let before = recorder.captured().len();
            animation
                .dispatch(action)
                .expect("every action is valid on a streaming timeline");
            let spans = recorder.captured();
            assert_eq!(
                spans.len(),
                before + 1,
                "one span per dispatch of {action:?}, not a nested duplicate: {spans:?}"
            );
            assert_eq!(spans[before]["operation"], "dispatch", "{action:?}");
            assert_eq!(spans[before]["outcome"], "success", "{action:?}");
        }
    });
}
