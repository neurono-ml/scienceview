//! Executable specification of the motion tokens and reduced-motion collapse.

use std::time::Duration;

use crate::{
    SVMotionDuration, SVMotionEasing, SVTheme, SVTimingFunction, SVTokenValue, sv_default_theme,
};

fn sv_reduced_theme() -> SVTheme {
    SVTheme::builder()
        .reduced_motion(true)
        .build()
        .expect("the build-time theme cannot fail to build")
}

#[test]
fn durations_are_authored_in_milliseconds_and_resolve_to_durations() {
    let motion = sv_default_theme().tokens().motion().to_owned();
    assert_eq!(
        motion.duration(SVMotionDuration::Fast),
        Duration::from_millis(120)
    );
    assert_eq!(
        motion.duration(SVMotionDuration::Normal),
        Duration::from_millis(240)
    );
    assert_eq!(
        motion.duration(SVMotionDuration::Slow),
        Duration::from_millis(400)
    );
    assert_eq!(motion.durations().normal_milliseconds(), 240.0);
}

#[test]
fn easings_are_tokens_and_evaluate_to_the_authored_curve() {
    let motion = sv_default_theme().tokens().motion().to_owned();
    assert_eq!(
        motion.easing(SVMotionEasing::Standard),
        SVTimingFunction::CubicBezier {
            x1: 0.2,
            y1: 0.0,
            x2: 0.0,
            y2: 1.0,
        }
    );
    let easing = motion.easing(SVMotionEasing::Decelerate);
    assert!((easing.evaluate(0.0) - 0.0).abs() < 0.001);
    assert!((easing.evaluate(1.0) - 1.0).abs() < 0.001);
    assert!(easing.evaluate(0.5) > 0.0 && easing.evaluate(0.5) < 1.0);
}

#[test]
fn the_default_theme_does_not_reduce_motion() {
    let theme = sv_default_theme();
    assert!(!theme.reduced_motion());
    assert_eq!(
        theme.resolve("motion.reduced"),
        Ok(SVTokenValue::Boolean(false))
    );
}

#[test]
fn reduced_motion_collapses_every_duration_to_zero() {
    let theme = sv_reduced_theme();
    let motion = theme.tokens().motion().to_owned();
    assert!(motion.reduced_motion());
    assert_eq!(motion.duration(SVMotionDuration::Fast), Duration::ZERO);
    assert_eq!(motion.duration(SVMotionDuration::Normal), Duration::ZERO);
    assert_eq!(motion.duration(SVMotionDuration::Slow), Duration::ZERO);
}

#[test]
fn reduced_motion_collapses_durations_at_the_token_boundary() {
    let theme = sv_reduced_theme();
    assert_eq!(
        theme.resolve("motion.duration.fast"),
        Ok(SVTokenValue::Duration(Duration::ZERO))
    );
    assert_eq!(
        theme.resolve("motion.duration.normal"),
        Ok(SVTokenValue::Duration(Duration::ZERO))
    );
    assert_eq!(
        theme.resolve("motion.duration.slow"),
        Ok(SVTokenValue::Duration(Duration::ZERO))
    );
    assert_eq!(
        theme.resolve("motion.reduced"),
        Ok(SVTokenValue::Boolean(true))
    );
}

#[test]
fn reduced_motion_keeps_easings_readable() {
    let theme = sv_reduced_theme();
    assert_eq!(
        theme.resolve("motion.easing.standard"),
        Ok(SVTokenValue::Easing(SVTimingFunction::CubicBezier {
            x1: 0.2,
            y1: 0.0,
            x2: 0.0,
            y2: 1.0,
        }))
    );
}
