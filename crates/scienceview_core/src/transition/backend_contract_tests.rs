//! Executable specification of the backend transition contract: a backend
//! declares support, and one without it resolves the transition to its final
//! state.

use crate::sv_backend::SVBackendKind;
use crate::sv_mark::SVMarkPoint;
use crate::sv_transition::{
    SVEasing, SVMarkPropertyValue, SVMarkState, SVMatchKeyValue, SVTransition, SVTransitionSupport,
    sv_transition_support,
};

fn sv_key() -> SVMatchKeyValue {
    SVMatchKeyValue::index(0)
}

fn sv_plan() -> crate::sv_transition::SVTransitionPlan {
    let previous = [SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)))];
    let next = [SVMarkState::new(sv_key())
        .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(10.0, 0.0)))];

    SVTransition::new()
        .easing(SVEasing::Linear)
        .plan(&previous, &next)
        .expect("keys are unique")
}

#[test]
fn a_static_export_backend_declares_transition_unsupported() {
    assert_eq!(
        sv_transition_support(SVBackendKind::Svg),
        SVTransitionSupport::FinalStateOnly
    );
}

#[test]
fn every_in_place_backend_declares_transition_support() {
    for backend_kind in [
        SVBackendKind::Html,
        SVBackendKind::Tui,
        SVBackendKind::Native,
        SVBackendKind::Wasm,
        SVBackendKind::React,
        SVBackendKind::Vue,
        SVBackendKind::Angular,
        SVBackendKind::Tauri,
        SVBackendKind::Dioxus,
    ] {
        assert_eq!(
            sv_transition_support(backend_kind),
            SVTransitionSupport::Animated,
            "{backend_kind} updates in place, so it animates"
        );
    }
}

#[test]
fn a_static_backend_draws_the_final_state_at_any_progress() {
    let plan = sv_plan();
    let final_frame = plan.evaluate(1.0).expect("valid progress");

    for progress in [0.0, 0.25, 0.5, 0.75] {
        assert_eq!(
            plan.evaluate_for_backend(SVBackendKind::Svg, progress)
                .expect("valid progress"),
            final_frame,
            "a static export has no clock"
        );
    }
}

#[test]
fn an_animating_backend_interpolates_at_the_same_progress() {
    let plan = sv_plan();

    let animated = plan
        .evaluate_for_backend(SVBackendKind::Native, 0.5)
        .expect("valid progress");
    let direct = plan.evaluate(0.5).expect("valid progress");

    assert_eq!(animated, direct);
    assert_ne!(
        animated,
        plan.evaluate(1.0).expect("valid progress"),
        "an animating backend must not jump to the end"
    );
}

#[test]
fn transition_support_agrees_with_the_capability_matrix() {
    for backend_kind in [
        SVBackendKind::Svg,
        SVBackendKind::Html,
        SVBackendKind::Tui,
        SVBackendKind::Native,
        SVBackendKind::Wasm,
        SVBackendKind::React,
        SVBackendKind::Vue,
        SVBackendKind::Angular,
        SVBackendKind::Tauri,
        SVBackendKind::Dioxus,
    ] {
        let capabilities = crate::sv_backend::sv_backend_capabilities(backend_kind);
        let expected =
            if capabilities.supports(crate::sv_backend::SVBackendCapability::InPlaceUpdate) {
                SVTransitionSupport::Animated
            } else {
                SVTransitionSupport::FinalStateOnly
            };

        assert_eq!(sv_transition_support(backend_kind), expected);
    }
}
