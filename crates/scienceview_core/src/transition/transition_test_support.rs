//! Shared fixtures for the transition tests: one-key states and the frame that
//! resolves one property at a given progress.

use crate::sv_transition::{
    SVEasing, SVMarkPropertyValue, SVMarkState, SVMatchKeyValue, SVTransition,
};

pub(crate) fn sv_key() -> SVMatchKeyValue {
    SVMatchKeyValue::index(0)
}

fn sv_pair(from: SVMarkPropertyValue, to: SVMarkPropertyValue) -> (SVMarkState, SVMarkState) {
    (
        SVMarkState::new(sv_key()).with_property(from),
        SVMarkState::new(sv_key()).with_property(to),
    )
}

pub(crate) fn sv_properties_at(
    from: SVMarkPropertyValue,
    to: SVMarkPropertyValue,
    progress: f64,
) -> crate::sv_transition::SVMarkProperties {
    let (previous, next) = sv_pair(from, to);
    let frame = SVTransition::new()
        .easing(SVEasing::Linear)
        .plan(&[previous], &[next])
        .expect("keys are unique")
        .evaluate(progress)
        .expect("the progress is valid");

    frame
        .mark(sv_key())
        .expect("the update is part of the frame")
        .properties()
        .clone()
}
