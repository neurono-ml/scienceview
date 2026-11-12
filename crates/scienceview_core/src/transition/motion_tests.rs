//! Executable specification of motion tokens, reduced motion and progress.

use std::time::Duration;

use crate::sv_mark::SVMarkPoint;
use crate::sv_transition::{
    SVEasing, SVMarkPropertyValue, SVMarkState, SVMatchKeyValue, SVMotionStyle, SVTransition,
};

fn sv_key() -> SVMatchKeyValue {
    SVMatchKeyValue::index(0)
}

fn sv_transition_with(style: SVMotionStyle) -> SVTransition {
    SVTransition::new().motion(style)
}

#[test]
fn the_built_in_motion_tokens_are_the_documented_defaults() {
    let motion = SVMotionStyle::built_in();

    assert_eq!(motion.duration(), Duration::from_millis(240));
    assert_eq!(motion.delay(), Duration::ZERO);
    assert!(!motion.is_reduced_motion());
    assert_eq!(
        motion.easing(),
        SVEasing::CubicBezier {
            x1: 0.2,
            y1: 0.0,
            x2: 0.0,
            y2: 1.0,
        }
    );
}

#[test]
fn an_instant_transition_defaults_to_a_zero_duration() {
    let transition = SVTransition::new();

    assert_eq!(transition.motion_style().duration(), Duration::ZERO);
    assert_eq!(transition.motion_style().progress_at(Duration::ZERO), 1.0);
}

#[test]
fn progress_follows_the_duration_and_the_delay() {
    let motion = SVMotionStyle::built_in().with_delay(Duration::from_millis(40));

    assert_eq!(motion.progress_at(Duration::ZERO), 0.0);
    assert_eq!(motion.progress_at(Duration::from_millis(40)), 0.0);
    assert!((motion.progress_at(Duration::from_millis(160)) - 0.5).abs() < 1e-12);
    assert_eq!(motion.progress_at(Duration::from_millis(280)), 1.0);
    assert_eq!(motion.progress_at(Duration::from_secs(10)), 1.0);
}

#[test]
fn reduced_motion_collapses_every_duration_to_zero() {
    let motion = SVMotionStyle::built_in().reduced();

    assert!(motion.is_reduced_motion());
    assert_eq!(motion.duration(), Duration::ZERO);
    assert_eq!(motion.progress_at(Duration::ZERO), 1.0);
    assert_eq!(motion.progress_at(Duration::from_millis(120)), 1.0);
}

#[test]
fn a_reduced_motion_plan_draws_the_final_state_at_any_progress() {
    let previous = [SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)))];
    let next = [SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(10.0, 0.0)))];
    let plan = sv_transition_with(SVMotionStyle::built_in().reduced())
        .plan(&previous, &next)
        .expect("keys are unique");

    let final_frame = plan.evaluate(1.0).expect("valid progress");

    for progress in [0.0, 0.25, 0.5, 0.99] {
        assert_eq!(
            plan.evaluate(progress).expect("valid progress"),
            final_frame,
            "reduced motion must not interpolate at progress {progress}"
        );
    }
}

#[test]
fn a_non_finite_progress_is_rejected() {
    let previous = [SVMarkState::new(sv_key())];
    let next = [SVMarkState::new(sv_key())];
    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    for progress in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            plan.evaluate(progress).is_err(),
            "progress {progress} must be rejected"
        );
    }
}

#[test]
fn a_progress_outside_the_unit_interval_is_clamped() {
    let previous = [SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)))];
    let next = [SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(10.0, 0.0)))];
    let plan = SVTransition::new()
        .easing(SVEasing::Linear)
        .plan(&previous, &next)
        .expect("keys are unique");

    assert_eq!(
        plan.evaluate(-2.0).expect("clamped"),
        plan.evaluate(0.0).expect("valid")
    );
    assert_eq!(
        plan.evaluate(7.0).expect("clamped"),
        plan.evaluate(1.0).expect("valid")
    );
}

#[test]
fn a_plan_evaluates_its_timeline_progress() {
    let plan = sv_transition_with(SVMotionStyle::built_in())
        .plan(&[SVMarkState::new(sv_key())], &[SVMarkState::new(sv_key())])
        .expect("keys are unique");

    assert_eq!(plan.progress_at(Duration::ZERO), 0.0);
    assert_eq!(plan.progress_at(Duration::from_millis(240)), 1.0);
}

#[test]
fn the_configured_easing_is_applied_to_the_interpolation() {
    let previous = [SVMarkState::new(sv_key()).with_property(SVMarkPropertyValue::size(0.0))];
    let next = [SVMarkState::new(sv_key()).with_property(SVMarkPropertyValue::size(100.0))];
    let plan = SVTransition::new()
        .easing(SVEasing::EaseIn)
        .plan(&previous, &next)
        .expect("keys are unique");

    let size = plan
        .evaluate(0.5)
        .expect("valid progress")
        .mark(sv_key())
        .expect("the update is present")
        .properties()
        .size()
        .expect("the mark carries a size");

    assert!(
        size < 50.0,
        "an ease-in curve must lag linear at the midpoint, got {size}"
    );
    assert!(size > 0.0);
}

#[cfg(feature = "theme")]
mod themed {
    use super::*;
    use scienceview_theme::{SVMotionDuration, SVMotionEasing, SVTheme};

    #[test]
    fn a_themed_transition_reads_its_duration_and_easing_from_the_tokens() {
        let theme = SVTheme::builder().build().expect("the theme builds");
        let motion = theme.tokens().motion().to_owned();

        let style = SVMotionStyle::from_motion_tokens(&motion);

        assert_eq!(style.duration(), motion.duration(SVMotionDuration::Normal));
        assert_eq!(
            style.easing(),
            SVEasing::from(motion.easing(SVMotionEasing::Standard))
        );
        assert!(!style.is_reduced_motion());
    }

    #[test]
    fn a_reduced_theme_collapses_the_themed_transition_to_an_instant_swap() {
        let theme = SVTheme::builder()
            .reduced_motion(true)
            .build()
            .expect("the theme builds");
        let motion = theme.tokens().motion().to_owned();

        let style = SVMotionStyle::from_motion_tokens(&motion);

        assert!(style.is_reduced_motion());
        assert_eq!(style.duration(), Duration::ZERO);
    }
}
