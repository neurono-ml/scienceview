//! Companion concurrency tests for the animation timeline.

use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;

use crate::sv_animation::{
    SVAnimation, SVAnimationBuilder, SVKeyframe, SVKeyframeDelta, SVKeyframeSource, SVDuration,
};
use crate::sv_mark::SVMarkProperty;

fn sv_position(value: f64) -> SVKeyframeDelta {
    SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, value)]).expect("finite values")
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

#[test]
fn one_timeline_serves_several_threads_without_losing_the_bound() {
    let shared = Arc::new(Mutex::new(sv_streaming(16)));
    let mut handles = Vec::new();

    for producer in 0..4u64 {
        let animation = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for index in 0..250u64 {
                let timestamp = producer * 10_000 + index * 10;
                let mut guard = animation.lock().unwrap_or_else(PoisonError::into_inner);
                guard
                    .push_keyframe(sv_frame(timestamp, index as f64))
                    .expect("the stream accepts an increasing frame");
                guard.advance(Duration::from_millis(1));
            }
        }));
    }

    for handle in handles {
        handle.join().expect("no producer panicked");
    }

    let guard = shared.lock().unwrap_or_else(PoisonError::into_inner);
    assert!(guard.length() <= 16);
    assert_eq!(guard.accepted_keyframes(), 1_000);
}

#[test]
fn playback_is_reproducible_from_the_same_sequence_of_ticks() {
    let sv_play = || {
        let mut animation = SVAnimationBuilder::new()
            .deltas([sv_position(0.0), sv_position(10.0), sv_position(0.0)])
            .loop_mode(crate::sv_animation::SVAnimationLoop::Count(3))
            .build()
            .expect("a fixed timeline builds");
        animation.play();
        let mut samples = Vec::new();
        for _ in 0..40 {
            animation.advance(Duration::from_millis(125));
            samples.push((
                animation.position_seconds(),
                animation
                    .sample()
                    .and_then(|delta| delta.value_of(SVMarkProperty::Position)),
            ));
        }
        samples
    };

    let first = thread::spawn(sv_play);
    let second = thread::spawn(sv_play);

    assert_eq!(
        first.join().expect("no panic"),
        second.join().expect("no panic")
    );
}
