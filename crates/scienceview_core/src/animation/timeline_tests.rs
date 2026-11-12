//! Companion tests for the fixed keyframe source, sampling and playback.

use std::time::Duration;

use crate::sv_animation::{
    SVAnimation, SVAnimationBuilder, SVAnimationDirection, SVAnimationLoop, SVInterpolation,
    SVDuration, SVKeyframe, SVKeyframeDelta, SVPlaybackState,
};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkProperty;

fn sv_position(value: f64) -> SVKeyframeDelta {
    SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, value)]).expect("finite values")
}

fn sv_seconds(value: f64) -> SVDuration {
    SVDuration::from_seconds(value).expect("finite, non-negative seconds")
}

fn sv_fixed(values: &[f64]) -> SVAnimation {
    SVAnimationBuilder::new()
        .deltas(values.iter().copied().map(sv_position))
        .build()
        .expect("a fixed timeline builds")
}

fn sv_sample(animation: &SVAnimation) -> f64 {
    animation
        .sample()
        .expect("a timeline with keyframes samples")
        .value_of(SVMarkProperty::Position)
        .expect("the sample carries a position")
}

#[test]
fn a_fixed_source_assigns_one_default_duration_per_keyframe() {
    let animation = sv_fixed(&[0.0, 10.0, 20.0]);

    assert_eq!(animation.length(), 3);
    assert_eq!(animation.position().as_duration().as_millis(), 0);
    let (first, second) = animation.retained_keyframes();
    assert_eq!(first.len(), 3);
    assert_eq!(first[1].timestamp().as_duration().as_millis(), 1000);
    assert_eq!(first[2].timestamp().as_duration().as_millis(), 2000);
    assert!(second.is_empty());
}

#[test]
fn a_fixed_source_samples_every_keyframe_in_order() {
    let animation = sv_fixed(&[0.0, 10.0, 20.0]);

    assert_eq!(sv_sample(&animation), 0.0);
    assert_eq!(
        animation
            .sample_at(sv_seconds(1.0))
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(10.0)
    );
    assert_eq!(
        animation
            .sample_at(sv_seconds(2.0))
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(20.0)
    );
}

#[test]
fn a_fixed_source_interpolates_the_midpoint_of_a_segment() {
    let animation = sv_fixed(&[0.0, 10.0]);

    assert_eq!(
        animation
            .sample_at(sv_seconds(0.5))
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(5.0)
    );
}

#[test]
fn a_sample_before_the_first_keyframe_holds_the_first_state() {
    let animation = SVAnimationBuilder::new()
        .keyframe(SVKeyframe::new(sv_seconds(1.0), sv_position(4.0)))
        .keyframe(SVKeyframe::new(sv_seconds(2.0), sv_position(6.0)))
        .build()
        .expect("a fixed timeline builds");

    assert_eq!(
        animation
            .sample_at(sv_seconds(0.0))
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(4.0)
    );
}

#[test]
fn a_sample_after_the_last_keyframe_holds_the_last_state() {
    let animation = sv_fixed(&[0.0, 10.0]);

    assert_eq!(
        animation
            .sample_at(sv_seconds(9.0))
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(10.0)
    );
}

#[test]
fn a_fixed_source_rejects_out_of_order_timestamps() {
    let error = SVAnimationBuilder::new()
        .keyframe(SVKeyframe::new(sv_seconds(2.0), sv_position(0.0)))
        .keyframe(SVKeyframe::new(sv_seconds(1.0), sv_position(1.0)))
        .build()
        .expect_err("a timeline cannot run backwards");

    assert_eq!(
        error,
        SVError::OutOfOrderKeyframes {
            position: 1,
            previous_milliseconds: 2000,
            found_milliseconds: 1000,
        }
    );
}

#[test]
fn a_fixed_source_rejects_duplicate_timestamps() {
    let error = SVAnimationBuilder::new()
        .keyframe(SVKeyframe::new(sv_seconds(1.0), sv_position(0.0)))
        .keyframe(SVKeyframe::new(sv_seconds(1.0), sv_position(1.0)))
        .build()
        .expect_err("a duplicate timestamp is not strictly increasing");

    assert_eq!(
        error,
        SVError::OutOfOrderKeyframes {
            position: 1,
            previous_milliseconds: 1000,
            found_milliseconds: 1000,
        }
    );
}

#[test]
fn a_fixed_source_needs_at_least_one_keyframe() {
    assert_eq!(
        SVAnimationBuilder::new().build().err(),
        Some(SVError::EmptyKeyframeSource)
    );
}

#[test]
fn the_timeline_starts_idle_and_advances_nothing_before_play() {
    let mut animation = sv_fixed(&[0.0, 10.0]);

    assert_eq!(animation.state(), SVPlaybackState::Idle);
    assert_eq!(
        animation.advance(Duration::from_millis(500)),
        SVPlaybackState::Idle
    );
    assert_eq!(animation.position().as_duration().as_millis(), 0);
}

#[test]
fn play_advances_from_the_current_position_and_pause_freezes_it() {
    let mut animation = sv_fixed(&[0.0, 10.0]);

    animation.play();
    animation.advance(Duration::from_millis(400));
    assert_eq!(animation.state(), SVPlaybackState::Playing);
    assert!((animation.position_seconds() - 0.4).abs() < 1e-12);

    animation.pause();
    animation.advance(Duration::from_millis(400));
    assert_eq!(animation.state(), SVPlaybackState::Paused);
    assert!((animation.position_seconds() - 0.4).abs() < 1e-12);

    animation.play();
    animation.advance(Duration::from_millis(100));
    assert!((animation.position_seconds() - 0.5).abs() < 1e-12);
}

#[test]
fn advancing_by_zero_changes_nothing() {
    let mut animation = sv_fixed(&[0.0, 10.0]);
    animation.play();
    animation.advance(Duration::from_millis(300));

    let state = animation.advance(Duration::ZERO);

    assert_eq!(state, SVPlaybackState::Playing);
    assert!((animation.position_seconds() - 0.3).abs() < 1e-12);
}

#[test]
fn a_finished_timeline_holds_its_last_state() {
    let mut animation = sv_fixed(&[0.0, 10.0]);
    animation.play();
    animation.advance(Duration::from_millis(5000));

    assert_eq!(animation.state(), SVPlaybackState::Finished);
    assert!((animation.position_seconds() - 2.0).abs() < 1e-12);
    assert_eq!(sv_sample(&animation), 10.0);
    assert_eq!(animation.normalised_position(), 1.0);
}

#[test]
fn a_looping_timeline_wraps_forever() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .loop_mode(SVAnimationLoop::Forever)
        .build()
        .expect("a looping timeline builds");
    animation.play();

    animation.advance(Duration::from_millis(2500));

    assert_eq!(animation.state(), SVPlaybackState::Playing);
    assert!((animation.position_seconds() - 0.5).abs() < 1e-12);
    assert_eq!(
        animation
            .sample()
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(5.0)
    );
}

#[test]
fn a_counted_loop_finishes_after_the_last_pass() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .loop_mode(SVAnimationLoop::Count(2))
        .build()
        .expect("a counted timeline builds");
    animation.play();

    animation.advance(Duration::from_millis(2000));
    assert_eq!(
        animation.state(),
        SVPlaybackState::Playing,
        "the second pass starts"
    );
    assert!((animation.position_seconds() - 0.0).abs() < 1e-12);

    animation.advance(Duration::from_millis(2000));
    assert_eq!(animation.state(), SVPlaybackState::Finished);
    assert!((animation.position_seconds() - 2.0).abs() < 1e-12);
}

#[test]
fn a_counted_loop_skips_whole_passes_when_the_elapsed_time_is_large() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .loop_mode(SVAnimationLoop::Count(10))
        .build()
        .expect("a counted timeline builds");
    animation.play();

    animation.advance(Duration::from_millis(20_000));

    assert_eq!(animation.state(), SVPlaybackState::Finished);
    assert!((animation.position_seconds() - 2.0).abs() < 1e-12);
}

#[test]
fn the_reverse_direction_starts_at_the_end_and_runs_backwards() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .direction(SVAnimationDirection::Reverse)
        .build()
        .expect("a reversed timeline builds");

    animation.play();
    assert!((animation.position_seconds() - 2.0).abs() < 1e-12);
    animation.advance(Duration::from_millis(500));
    assert!((animation.position_seconds() - 1.5).abs() < 1e-12);
    assert_eq!(
        animation
            .sample()
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(2.5)
    );
}

#[test]
fn the_alternate_direction_reverses_on_the_next_pass() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .direction(SVAnimationDirection::Alternate)
        .loop_mode(SVAnimationLoop::Forever)
        .build()
        .expect("an alternating timeline builds");
    animation.play();
    animation.advance(Duration::from_millis(2500));

    assert!((animation.position_seconds() - 1.5).abs() < 1e-12);
    assert_eq!(
        animation
            .sample()
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(2.5)
    );
}

#[test]
fn the_speed_multiplies_the_elapsed_time() {
    let mut animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .speed(2.0)
        .build()
        .expect("a sped-up timeline builds");
    animation.play();

    animation.advance(Duration::from_millis(500));

    assert!((animation.position_seconds() - 1.0).abs() < 1e-12);
}

#[test]
fn the_speed_must_be_finite_and_positive() {
    assert_eq!(
        SVAnimationBuilder::new()
            .deltas([sv_position(0.0)])
            .speed(0.0)
            .build()
            .err(),
        Some(SVError::InvalidPlaybackSpeed)
    );
    assert_eq!(
        SVAnimationBuilder::new()
            .deltas([sv_position(0.0)])
            .speed(f64::NAN)
            .build()
            .err(),
        Some(SVError::InvalidPlaybackSpeed)
    );
}

#[test]
fn the_interpolation_kind_shapes_the_midpoint() {
    let sv_with = |interpolation: SVInterpolation| {
        SVAnimationBuilder::new()
            .deltas([sv_position(0.0), sv_position(10.0)])
            .interpolation(interpolation)
            .build()
            .expect("a timeline builds")
    };
    let sv_at_half = |animation: &SVAnimation| {
        animation
            .sample_at(sv_seconds(0.5))
            .expect("a sample")
            .value_of(SVMarkProperty::Position)
            .expect("a position")
    };

    assert_eq!(sv_at_half(&sv_with(SVInterpolation::Linear)), 5.0);
    assert!((sv_at_half(&sv_with(SVInterpolation::EaseIn)) - 2.5).abs() < 1e-12);
    assert!((sv_at_half(&sv_with(SVInterpolation::EaseOut)) - 7.5).abs() < 1e-12);
    assert!((sv_at_half(&sv_with(SVInterpolation::EaseInOut)) - 5.0).abs() < 1e-12);
    assert_eq!(sv_at_half(&sv_with(SVInterpolation::Step)), 0.0);
}

#[test]
fn a_step_interpolation_swaps_at_the_end_of_the_segment() {
    let animation = SVAnimationBuilder::new()
        .deltas([sv_position(0.0), sv_position(10.0)])
        .interpolation(SVInterpolation::Step)
        .build()
        .expect("a timeline builds");

    assert_eq!(
        animation
            .sample_at(sv_seconds(1.0))
            .expect("a sample")
            .value_of(SVMarkProperty::Position),
        Some(10.0)
    );
}

#[test]
fn seeking_clamps_to_the_timeline_and_leaves_a_finished_timeline_paused() {
    let mut animation = sv_fixed(&[0.0, 10.0]);
    animation.play();
    animation.advance(Duration::from_millis(5000));
    assert_eq!(animation.state(), SVPlaybackState::Finished);

    animation.seek(sv_seconds(0.5));
    assert_eq!(animation.state(), SVPlaybackState::Paused);
    assert!((animation.position_seconds() - 0.5).abs() < 1e-12);

    animation.seek(sv_seconds(99.0));
    assert!((animation.position_seconds() - 2.0).abs() < 1e-12);
}

#[test]
fn stepping_moves_to_the_neighbouring_keyframe() {
    let mut animation = sv_fixed(&[0.0, 10.0, 20.0]);

    animation.step(true).expect("a step forward");
    assert!((animation.position_seconds() - 1.0).abs() < 1e-12);
    animation.step(true).expect("a step forward");
    assert!((animation.position_seconds() - 2.0).abs() < 1e-12);
    animation.step(false).expect("a step backward");
    assert!((animation.position_seconds() - 1.0).abs() < 1e-12);
}

#[test]
fn a_restart_after_finishing_begins_at_the_start() {
    let mut animation = sv_fixed(&[0.0, 10.0]);
    animation.play();
    animation.advance(Duration::from_millis(5000));
    assert_eq!(animation.state(), SVPlaybackState::Finished);

    animation.play();
    assert_eq!(animation.state(), SVPlaybackState::Playing);
    assert!(animation.position_seconds() < 1e-12);
}
