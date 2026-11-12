//! Concurrency: a plan is immutable, so any thread may evaluate any progress.

use std::sync::Arc;

use crate::sv_mark::SVMarkPoint;
use crate::sv_transition::{
    SVEasing, SVMarkPropertyValue, SVMarkState, SVMatchKeyValue, SVTransition,
};

fn sv_plan() -> crate::sv_transition::SVTransitionPlan {
    let previous: Vec<SVMarkState> = (0..64)
        .map(|position| {
            SVMarkState::new(SVMatchKeyValue::index(position)).with_property(
                SVMarkPropertyValue::position(SVMarkPoint::new(position as f64, 0.0)),
            )
        })
        .collect();
    let next: Vec<SVMarkState> = (0..64)
        .map(|position| {
            SVMarkState::new(SVMatchKeyValue::index(position)).with_property(
                SVMarkPropertyValue::position(SVMarkPoint::new(0.0, position as f64)),
            )
        })
        .collect();

    SVTransition::new()
        .easing(SVEasing::Linear)
        .plan(&previous, &next)
        .expect("keys are unique")
}

#[test]
fn a_plan_evaluates_the_same_frame_on_every_thread() {
    let plan = Arc::new(sv_plan());
    let expected = plan.evaluate(0.5).expect("valid progress");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let plan = Arc::clone(&plan);
                scope.spawn(move || plan.evaluate(0.5).expect("valid progress"))
            })
            .collect();

        for handle in handles {
            let frame = handle.join().expect("the thread does not panic");
            assert_eq!(frame, expected);
        }
    });
}

#[test]
fn a_plan_is_send_and_sync() {
    fn sv_assert_send_and_sync<T: Send + Sync>() {}
    sv_assert_send_and_sync::<crate::sv_transition::SVTransitionPlan>();
    sv_assert_send_and_sync::<crate::sv_transition::SVTransitionFrame>();
    sv_assert_send_and_sync::<crate::sv_transition::SVMarkState>();
}
