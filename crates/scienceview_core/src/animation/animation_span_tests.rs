//! Verifies the animation span fields through a real subscriber (patterns P8).

use std::time::Duration;

use crate::sv_animation::{
    SVAnimation, SVAnimationBuilder, SVDuration, SVKeyframe, SVKeyframeDelta, SVKeyframeSource,
};
use crate::sv_mark::SVMarkProperty;
use crate::sv_observability::span_test_support::SVSpanRecorder;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

fn sv_frame(milliseconds: u64, value: f64) -> SVKeyframe {
    let delta =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, value)]).expect("finite values");
    SVKeyframe::new(SVDuration::milliseconds(milliseconds), delta)
}

fn sv_streaming() -> SVAnimation {
    SVAnimationBuilder::new()
        .source(SVKeyframeSource::Streaming)
        .capacity(4)
        .build()
        .expect("a streaming timeline builds")
}

#[test]
fn an_advance_emits_one_span_with_the_source_and_the_position() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let mut animation = sv_streaming();
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");
    animation
        .push_keyframe(sv_frame(1000, 10.0))
        .expect("the second frame");
    animation.play();

    tracing::subscriber::with_default(subscriber, || {
        animation.advance(Duration::from_millis(500));
    });

    let spans = recorder.captured();
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
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let mut animation = sv_streaming();

    tracing::subscriber::with_default(subscriber, || {
        animation
            .push_keyframe(sv_frame(0, 1.0))
            .expect("the first frame");
    });

    let spans = recorder.captured();
    assert_eq!(spans.len(), 1, "one span per arrival: {spans:?}");
    assert_eq!(spans[0]["operation"], "push_keyframe");
    assert_eq!(spans[0]["outcome"], "success");
}

#[test]
fn a_sample_emits_one_span() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let mut animation = sv_streaming();
    animation
        .push_keyframe(sv_frame(0, 0.0))
        .expect("the first frame");

    tracing::subscriber::with_default(subscriber, || {
        let _ = animation.sample();
    });

    let spans = recorder.captured();
    assert_eq!(spans.len(), 1, "one span per sample: {spans:?}");
    assert_eq!(spans[0]["operation"], "sample");
}
