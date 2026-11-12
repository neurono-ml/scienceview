//! Executable specification of the enter and exit rules and the reduced-motion
//! interaction with them.

use crate::sv_mark::SVMarkPoint;
use crate::sv_transition::{
    SVEasing, SVEnterRule, SVExitRule, SVMarkPropertyValue, SVMarkState, SVMatchKeyValue,
    SVTransition, SVTransitionPhase,
};

fn sv_key(code: f64) -> SVMatchKeyValue {
    SVMatchKeyValue::category(code).expect("finite")
}

fn sv_mark(code: f64, x: f64) -> SVMarkState {
    SVMarkState::new(sv_key(code))
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(x, 0.0)))
        .with_property(SVMarkPropertyValue::opacity(1.0))
        .with_property(SVMarkPropertyValue::size(10.0))
}

fn sv_plan(
    enter_rule: SVEnterRule,
    exit_rule: SVExitRule,
) -> crate::sv_transition::SVTransitionPlan {
    let previous = [sv_mark(1.0, 0.0)];
    let next = [sv_mark(2.0, 10.0)];

    SVTransition::new()
        .easing(SVEasing::Linear)
        .enter_rule(enter_rule)
        .exit_rule(exit_rule)
        .plan(&previous, &next)
        .expect("keys are unique")
}

#[test]
fn a_fade_in_starts_transparent_and_ends_opaque() {
    let plan = sv_plan(SVEnterRule::FadeIn, SVExitRule::None);

    let start = plan.evaluate(0.0).expect("valid progress");
    let end = plan.evaluate(1.0).expect("valid progress");
    let entering = start.mark(sv_key(2.0)).expect("the enter is drawn");

    assert_eq!(entering.properties().opacity(), Some(0.0));
    assert_eq!(
        entering.properties().position(),
        Some(SVMarkPoint::new(10.0, 0.0))
    );
    assert_eq!(
        end.mark(sv_key(2.0))
            .and_then(|mark| mark.properties().opacity()),
        Some(1.0)
    );
}

#[test]
fn a_scale_in_starts_at_zero_size() {
    let plan = sv_plan(SVEnterRule::ScaleIn, SVExitRule::None);

    let entering = plan
        .evaluate(0.25)
        .expect("valid progress")
        .mark(sv_key(2.0))
        .expect("the enter is drawn")
        .properties()
        .size();

    assert!((entering.expect("size") - 2.5).abs() < 1e-12);
}

#[test]
fn a_slide_in_starts_below_its_target() {
    let plan = sv_plan(SVEnterRule::SlideIn, SVExitRule::None);

    let start = plan
        .evaluate(0.0)
        .expect("valid progress")
        .mark(sv_key(2.0))
        .expect("the enter is drawn")
        .properties()
        .position()
        .expect("position");

    assert_eq!(start.x(), 10.0);
    assert!(start.y() > 0.0, "the mark slides up from below");
}

#[test]
fn an_enter_with_no_rule_appears_at_its_target_state() {
    let plan = sv_plan(SVEnterRule::None, SVExitRule::None);

    let frame = plan.evaluate(0.0).expect("valid progress");
    let start = frame.mark(sv_key(2.0)).expect("the enter is drawn");

    assert_eq!(start.properties().opacity(), Some(1.0));
    assert_eq!(start.properties().size(), Some(10.0));
}

#[test]
fn a_fade_out_draws_the_exit_until_the_end_and_then_drops_it() {
    let plan = sv_plan(SVEnterRule::None, SVExitRule::FadeOut);

    let start = plan.evaluate(0.0).expect("valid progress");
    let middle = plan.evaluate(0.5).expect("valid progress");
    let end = plan.evaluate(1.0).expect("valid progress");

    assert_eq!(
        start.mark(sv_key(1.0)).map(|mark| mark.phase()),
        Some(SVTransitionPhase::Exit)
    );
    assert!(
        (middle
            .mark(sv_key(1.0))
            .and_then(|mark| mark.properties().opacity())
            .expect("opacity")
            - 0.5)
            .abs()
            < 1e-12
    );
    assert!(
        end.mark(sv_key(1.0)).is_none(),
        "the exit is gone at the end"
    );
}

#[test]
fn a_scale_out_shrinks_to_nothing() {
    let plan = sv_plan(SVEnterRule::None, SVExitRule::ScaleOut);

    let size = plan
        .evaluate(0.5)
        .expect("valid progress")
        .mark(sv_key(1.0))
        .and_then(|mark| mark.properties().size())
        .expect("size");

    assert!((size - 5.0).abs() < 1e-12);
}

#[test]
fn an_exit_with_no_rule_disappears_immediately() {
    let plan = sv_plan(SVEnterRule::None, SVExitRule::None);

    assert!(
        plan.evaluate(0.0)
            .expect("valid progress")
            .mark(sv_key(1.0))
            .is_none()
    );
}

#[test]
fn a_slide_out_ends_below_its_last_position() {
    let plan = sv_plan(SVEnterRule::None, SVExitRule::SlideOut);

    let position = plan
        .evaluate(0.5)
        .expect("valid progress")
        .mark(sv_key(1.0))
        .and_then(|mark| mark.properties().position())
        .expect("position");

    assert_eq!(position.x(), 0.0);
    assert!(position.y() > 0.0);
}
