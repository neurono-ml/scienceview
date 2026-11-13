//! The reducer: actions change domains, visibility and selection.

use crate::sv_control::{SVControlAction, SVControlAxis, SVControlRange, SVControlState};
use crate::sv_scale::{SVDomain, SVScaleTransform};

fn state_with_horizontal() -> SVControlState {
    let mut state = SVControlState::new();
    state
        .register_axis(
            SVControlAxis::Horizontal,
            SVDomain::new(0.0, 100.0).expect("ordered domain"),
        )
        .expect("the axis is new");
    state
}

#[test]
fn zooming_in_narrows_the_visible_domain_and_emits_an_event() {
    let mut state = state_with_horizontal();

    let report = state
        .dispatch(&SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor: 0.5,
            anchor: 0.5,
        })
        .expect("the axis is registered");

    assert_eq!(
        state.visible_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(25.0, 75.0).expect("ordered domain"))
    );
    assert_eq!(report.events().len(), 1);
    assert!(matches!(
        report.events()[0].change(),
        crate::sv_control::SVControlChange::Range(_)
    ));
}

#[test]
fn a_zoom_never_leaves_the_registered_bounds() {
    let mut state = state_with_horizontal();

    state
        .dispatch(&SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor: 100.0,
            anchor: 0.5,
        })
        .expect("the axis is registered");

    assert_eq!(
        state.visible_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(0.0, 100.0).expect("clamped domain"))
    );
}

#[test]
fn panning_shifts_the_visible_domain_and_stops_at_the_bounds() {
    let mut state = state_with_horizontal();
    state
        .dispatch(&SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor: 0.5,
            anchor: 0.5,
        })
        .expect("the axis is registered");

    state
        .dispatch(&SVControlAction::Pan {
            axis: SVControlAxis::Horizontal,
            fraction: 1.0,
        })
        .expect("the axis is registered");
    assert_eq!(
        state.visible_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(50.0, 100.0).expect("clamped domain"))
    );

    state
        .dispatch(&SVControlAction::Pan {
            axis: SVControlAxis::Horizontal,
            fraction: -10.0,
        })
        .expect("the axis is registered");
    assert_eq!(
        state.visible_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(0.0, 50.0).expect("clamped domain"))
    );
}

#[test]
fn a_logarithmic_axis_zooms_in_its_transformed_space() {
    let mut state = SVControlState::new();
    state
        .register_axis_with_transform(
            SVControlAxis::Horizontal,
            SVDomain::new(1.0, 1000.0).expect("ordered domain"),
            SVScaleTransform::Logarithmic,
        )
        .expect("the axis is new");

    state
        .dispatch(&SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor: 0.5,
            anchor: 0.5,
        })
        .expect("the axis is registered");

    let visible = state
        .visible_domain(SVControlAxis::Horizontal)
        .expect("the axis is registered");
    assert!((visible.lower() - 5.623_413_251_9).abs() < 1e-9);
    assert!((visible.upper() - 177.827_941_003_892).abs() < 1e-9);
}

#[test]
fn setting_a_range_clamps_it_to_the_axis_bounds() {
    let mut state = state_with_horizontal();

    state
        .dispatch(&SVControlAction::SetRange {
            axis: SVControlAxis::Horizontal,
            range: SVControlRange::new(-20.0, 40.0).expect("ordered range"),
        })
        .expect("the axis is registered");

    assert_eq!(
        state.visible_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(0.0, 40.0).expect("clamped domain"))
    );
}

#[test]
fn bounds_must_be_inside_the_transform_space() {
    let mut state = SVControlState::new();
    state
        .register_axis_with_transform(
            SVControlAxis::Horizontal,
            SVDomain::new(1.0, 1000.0).expect("ordered domain"),
            SVScaleTransform::Logarithmic,
        )
        .expect("the axis is new");

    assert!(matches!(
        state.set_axis_bounds(
            SVControlAxis::Horizontal,
            SVDomain::new(-1.0, 100.0).expect("ordered domain"),
        ),
        Err(crate::sv_error::SVError::InvalidConfiguration { .. })
    ));
}
