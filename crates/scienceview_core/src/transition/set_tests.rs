//! Executable specification of the enter, update and exit sets.

use crate::sv_mark::SVMarkPoint;
use crate::sv_transition::{
    SVMarkPropertyValue, SVMarkState, SVMatchKeyValue, SVTransition, SVTransitionPhase,
};

fn sv_state(key: SVMatchKeyValue, x: f64) -> SVMarkState {
    SVMarkState::new(key).with_property(SVMarkPropertyValue::position(SVMarkPoint::new(x, 0.0)))
}

fn sv_key(code: f64) -> SVMatchKeyValue {
    SVMatchKeyValue::category(code).expect("finite")
}

#[test]
fn a_persisting_key_is_an_update() {
    let previous = [sv_state(sv_key(1.0), 0.0)];
    let next = [sv_state(sv_key(1.0), 10.0)];

    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    assert!(plan.enter().is_empty());
    assert!(plan.exit().is_empty());
    assert_eq!(plan.update().len(), 1);
    assert_eq!(plan.update()[0].key(), sv_key(1.0));
}

#[test]
fn a_key_only_in_the_next_state_is_an_enter() {
    let previous = [sv_state(sv_key(1.0), 0.0)];
    let next = [sv_state(sv_key(1.0), 0.0), sv_state(sv_key(2.0), 10.0)];

    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    assert_eq!(plan.enter().len(), 1);
    assert_eq!(plan.enter()[0].key(), sv_key(2.0));
    assert_eq!(plan.update().len(), 1);
    assert!(plan.exit().is_empty());
}

#[test]
fn a_key_only_in_the_previous_state_is_an_exit() {
    let previous = [sv_state(sv_key(1.0), 0.0), sv_state(sv_key(2.0), 10.0)];
    let next = [sv_state(sv_key(1.0), 0.0)];

    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    assert_eq!(plan.exit().len(), 1);
    assert_eq!(plan.exit()[0].key(), sv_key(2.0));
    assert!(plan.enter().is_empty());
}

#[test]
fn a_changed_key_is_an_exit_plus_an_enter() {
    let previous = [sv_state(sv_key(1.0), 0.0)];
    let next = [sv_state(sv_key(2.0), 10.0)];

    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    assert!(plan.update().is_empty());
    assert_eq!(plan.enter().len(), 1);
    assert_eq!(plan.exit().len(), 1);
}

#[test]
fn matching_never_pairs_marks_by_position() {
    let previous = [sv_state(sv_key(4.0), 0.0), sv_state(sv_key(5.0), 10.0)];
    let next = [sv_state(sv_key(5.0), 20.0), sv_state(sv_key(6.0), 30.0)];

    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    assert_eq!(plan.update().len(), 1);
    assert_eq!(plan.update()[0].key(), sv_key(5.0));
    assert_eq!(
        plan.update()[0]
            .from()
            .properties()
            .position()
            .expect("the source carries a position")
            .x(),
        10.0
    );
    assert_eq!(plan.enter()[0].key(), sv_key(6.0));
    assert_eq!(plan.exit()[0].key(), sv_key(4.0));
}

#[test]
fn the_frame_reports_the_phase_of_every_mark() {
    let previous = [sv_state(sv_key(1.0), 0.0)];
    let next = [sv_state(sv_key(2.0), 10.0)];

    let frame = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique")
        .evaluate(0.0)
        .expect("zero is a valid progress");

    assert_eq!(
        frame.mark(sv_key(1.0)).map(|mark| mark.phase()),
        Some(SVTransitionPhase::Exit)
    );
    assert_eq!(
        frame.mark(sv_key(2.0)).map(|mark| mark.phase()),
        Some(SVTransitionPhase::Enter)
    );
}

#[test]
fn duplicate_keys_in_one_state_are_rejected_before_interpolation() {
    let previous = [sv_state(sv_key(1.0), 0.0), sv_state(sv_key(1.0), 10.0)];
    let next = [sv_state(sv_key(1.0), 20.0)];

    let error = SVTransition::new()
        .plan(&previous, &next)
        .expect_err("a repeated key is ambiguous");

    assert!(matches!(
        error,
        crate::sv_error::SVError::InvalidConfiguration { .. }
    ));
}

#[test]
fn the_frame_order_is_deterministic_across_calls() {
    let previous = [sv_state(sv_key(1.0), 0.0), sv_state(sv_key(2.0), 1.0)];
    let next = [sv_state(sv_key(2.0), 2.0), sv_state(sv_key(3.0), 3.0)];
    let plan = SVTransition::new()
        .plan(&previous, &next)
        .expect("keys are unique");

    let first = plan.evaluate(0.4).expect("valid progress");
    let second = plan.evaluate(0.4).expect("valid progress");

    assert_eq!(first, second);
    let keys: Vec<SVMatchKeyValue> = first.entries().iter().map(|mark| mark.key()).collect();
    assert_eq!(
        keys,
        vec![sv_key(2.0), sv_key(3.0), sv_key(1.0)],
        "enters and updates follow the next state, exits follow the previous one"
    );
}
