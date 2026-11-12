//! Companion tests for the keyframe vocabulary: durations, deltas, frames.

use crate::sv_animation::{
    SVDuration, SVInterpolation, SVKeyframe, SVKeyframeChange, SVKeyframeDelta,
};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkProperty;

#[test]
fn a_duration_rejects_a_negative_number_of_milliseconds() {
    assert_eq!(
        SVDuration::from_milliseconds(-1).err(),
        Some(SVError::InvalidAnimationDuration)
    );
}

#[test]
fn a_duration_rejects_a_non_finite_number_of_seconds() {
    assert_eq!(
        SVDuration::from_seconds(f64::NAN).err(),
        Some(SVError::InvalidAnimationDuration)
    );
    assert_eq!(
        SVDuration::from_seconds(f64::INFINITY).err(),
        Some(SVError::InvalidAnimationDuration)
    );
}

#[test]
fn a_zero_duration_is_accepted_and_reported_as_zero() {
    let duration = SVDuration::from_milliseconds(0).expect("zero is a valid duration");

    assert!(duration.is_zero());
    assert_eq!(duration.as_duration().as_millis(), 0);
    assert_eq!(duration.as_seconds(), 0.0);
}

#[test]
fn a_duration_converts_between_milliseconds_and_seconds() {
    let duration = SVDuration::milliseconds(1500);

    assert_eq!(duration.as_duration().as_millis(), 1500);
    assert!((duration.as_seconds() - 1.5).abs() < 1e-12);
    assert!(!duration.is_zero());
}

#[test]
fn a_delta_rejects_a_non_finite_value() {
    let error = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, f64::NAN)])
        .expect_err("NaN cannot be drawn");

    assert_eq!(
        error,
        SVError::NonFiniteAnimationValue {
            property: SVMarkProperty::Position,
            position: 0,
        }
    );
}

#[test]
fn a_delta_rejects_an_infinite_value() {
    let error = SVKeyframeDelta::from_pairs(&[
        (SVMarkProperty::Size, 1.0),
        (SVMarkProperty::Opacity, f64::INFINITY),
    ])
    .expect_err("infinity cannot be drawn");

    assert_eq!(
        error,
        SVError::NonFiniteAnimationValue {
            property: SVMarkProperty::Opacity,
            position: 1,
        }
    );
}

#[test]
fn a_delta_rejects_the_same_property_twice() {
    let changes = vec![
        SVKeyframeChange::new(SVMarkProperty::Position, 1.0).expect("finite"),
        SVKeyframeChange::new(SVMarkProperty::Position, 2.0).expect("finite"),
    ];

    assert_eq!(
        SVKeyframeDelta::new(changes).err(),
        Some(SVError::DuplicateAnimationProperty {
            property: SVMarkProperty::Position,
        })
    );
}

#[test]
fn a_delta_reads_back_the_values_it_was_given() {
    let delta = SVKeyframeDelta::from_pairs(&[
        (SVMarkProperty::Position, 2.0),
        (SVMarkProperty::Opacity, 0.5),
    ])
    .expect("finite values");

    assert_eq!(delta.length(), 2);
    assert!(!delta.is_empty());
    assert_eq!(delta.value_of(SVMarkProperty::Position), Some(2.0));
    assert_eq!(delta.value_of(SVMarkProperty::Opacity), Some(0.5));
    assert_eq!(delta.value_of(SVMarkProperty::Size), None);
    assert_eq!(delta.changes()[0].property(), SVMarkProperty::Position);
}

#[test]
fn an_empty_delta_is_valid_and_empty() {
    let delta = SVKeyframeDelta::new(Vec::new()).expect("no change is a valid change");

    assert!(delta.is_empty());
    assert_eq!(delta.length(), 0);
}

#[test]
fn interpolation_hits_both_endpoints_exactly() {
    let start = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 0.0)]).expect("finite");
    let target = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 10.0)]).expect("finite");

    assert_eq!(
        start
            .interpolate(&target, 0.0)
            .value_of(SVMarkProperty::Position),
        Some(0.0)
    );
    assert_eq!(
        start
            .interpolate(&target, 1.0)
            .value_of(SVMarkProperty::Position),
        Some(10.0)
    );
}

#[test]
fn interpolation_reaches_the_midpoint_of_an_interpolable_property() {
    let start = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 2.0)]).expect("finite");
    let target = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 6.0)]).expect("finite");

    assert_eq!(
        start
            .interpolate(&target, 0.5)
            .value_of(SVMarkProperty::Position),
        Some(4.0)
    );
}

#[test]
fn interpolation_clamps_a_progress_outside_the_unit_interval() {
    let start = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 0.0)]).expect("finite");
    let target = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 10.0)]).expect("finite");

    assert_eq!(
        start
            .interpolate(&target, -3.0)
            .value_of(SVMarkProperty::Position),
        Some(0.0)
    );
    assert_eq!(
        start
            .interpolate(&target, 4.0)
            .value_of(SVMarkProperty::Position),
        Some(10.0)
    );
}

#[test]
fn interpolation_keeps_a_property_that_only_the_start_declares() {
    let start = SVKeyframeDelta::from_pairs(&[
        (SVMarkProperty::Position, 0.0),
        (SVMarkProperty::Opacity, 0.25),
    ])
    .expect("finite");
    let target = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 10.0)]).expect("finite");
    let sampled = start.interpolate(&target, 0.5);

    assert_eq!(sampled.value_of(SVMarkProperty::Opacity), Some(0.25));
}

#[test]
fn interpolation_adopts_a_property_that_only_the_target_declares() {
    let start = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 0.0)]).expect("finite");
    let target = SVKeyframeDelta::from_pairs(&[
        (SVMarkProperty::Position, 10.0),
        (SVMarkProperty::Opacity, 0.75),
    ])
    .expect("finite");
    let sampled = start.interpolate(&target, 0.5);

    assert_eq!(sampled.value_of(SVMarkProperty::Opacity), Some(0.75));
}

#[test]
fn interpolation_swaps_a_discrete_property_only_at_the_end_of_the_segment() {
    let start =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Shape, 0.0)]).expect("finite values");
    let target =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Shape, 1.0)]).expect("finite values");

    assert_eq!(
        start
            .interpolate(&target, 0.5)
            .value_of(SVMarkProperty::Shape),
        Some(0.0)
    );
    assert_eq!(
        start
            .interpolate(&target, 1.0)
            .value_of(SVMarkProperty::Shape),
        Some(1.0)
    );
}

#[test]
fn a_keyframe_carries_its_timestamp_and_delta() {
    let delta = SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 3.0)]).expect("finite");
    let keyframe = SVKeyframe::new(SVDuration::milliseconds(250), delta.clone());

    assert_eq!(keyframe.timestamp().as_duration().as_millis(), 250);
    assert_eq!(
        keyframe.delta().value_of(SVMarkProperty::Position),
        Some(3.0)
    );
}

#[test]
fn the_interpolation_kinds_share_one_easing_curve_definition() {
    assert_eq!(SVInterpolation::Linear.ease(0.25), 0.25);
    assert!((SVInterpolation::EaseIn.ease(0.5) - 0.25).abs() < 1e-12);
    assert!((SVInterpolation::EaseOut.ease(0.5) - 0.75).abs() < 1e-12);
    assert!((SVInterpolation::EaseInOut.ease(0.5) - 0.5).abs() < 1e-12);
    assert_eq!(SVInterpolation::Step.ease(0.99), 0.0);
    assert_eq!(SVInterpolation::Step.ease(1.0), 1.0);
}
