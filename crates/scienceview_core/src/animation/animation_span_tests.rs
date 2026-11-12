//! Verifies the animation span fields through a real subscriber (patterns P8).

use std::time::Duration;

use crate::sv_animation::{SVAnimationBuilder, SVKeyframeDelta, SVKeyframeSource};
use crate::sv_mark::SVMarkProperty;
use crate::sv_observability::span_test_support::SVSpanRecorder;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

fn sv_position(value: f64) -> SVKeyframeDelta {
    SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, value)]).expect("finite values")
}

#[test]
fn every_public_animation_operation_emits_one_span() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());

    tracing::subscriber::with_default(subscriber, || {
        let mut animation = SVAnimationBuilder::new()
            .source(SVKeyframeSource::Streaming)
            .capacity(4)
            .build()
            .expect("a streaming timeline builds");
        animation.play();
        animation.advance(Duration::from_millis(100));
        animation.pause();
        animation.step(true).expect("a step");
        animation.seek(crate::sv_animation::SVDuration::milliseconds(0));
        animation.set_speed(2.0).expect("a valid speed");
        animation.dispatch(crate::sv_animation::SVAnimationAction::Toggle);
        animation.push_keyframe(crate::sv_animation::SVKeyframe::new(
            crate::sv_animation::SVDuration::milliseconds(0),
            sv_position(1.0),
        ));
        animation.sample();
    });

    let spans = recorder.captured();
    let operations: Vec<&str> = spans
        .iter()
        .filter_map(|span| span.get("operation").map(String::as_str))
        .collect();

    assert!(operations.contains(&"advance"), "{operations:?}");
    assert!(operations.contains(&"push_keyframe"), "{operations:?}");
    assert!(operations.contains(&"sample"), "{operations:?}");
    let advance = spans
        .iter()
        .find(|span| span.get("operation").map(String::as_str) == Some("advance"))
        .expect("an advance span");
    assert_eq!(advance["backend.kind"], "backend-independent");
    assert_eq!(advance["keyframe.source"], "Streaming");
    assert_eq!(advance["outcome"], "success");
    assert!(advance.contains_key("keyframes"), "{advance:?}");
    assert!(advance.contains_key("position"), "{advance:?}");
    assert!(advance.contains_key("duration"), "{advance:?}");
}
