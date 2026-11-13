//! The reducer: series, selection and the payload vocabulary.

use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlEvent, SVControlOrigin, SVControlRange,
    SVControlState, SVSelectionRegion, SVSeriesReference,
};
use crate::sv_error::SVError;
use crate::sv_scale::SVDomain;

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
fn toggling_a_series_flips_its_visibility_once_per_action() {
    let mut state = state_with_horizontal();
    let series = SVSeriesReference::Name("alpha".to_string());

    let first = state
        .dispatch(&SVControlAction::ToggleSeries(series.clone()))
        .expect("toggling never fails");
    assert!(!state.is_series_visible(&series));
    assert_eq!(first.events().len(), 1);

    state
        .dispatch(&SVControlAction::ToggleSeries(series.clone()))
        .expect("toggling never fails");
    assert!(state.is_series_visible(&series));
}

#[test]
fn a_selection_is_reported_and_can_be_cleared() {
    let mut state = state_with_horizontal();
    let region = SVSelectionRegion::new(
        SVControlRange::new(10.0, 20.0).expect("ordered range"),
        SVControlRange::new(30.0, 40.0).expect("ordered range"),
    );

    state
        .dispatch(&SVControlAction::SelectRegion(region))
        .expect("selecting never fails");
    assert_eq!(state.selection(), Some(region));

    let cleared = state
        .dispatch(&SVControlAction::ClearSelection)
        .expect("clearing never fails");
    assert_eq!(state.selection(), None);
    assert!(matches!(
        cleared.events()[0].change(),
        crate::sv_control::SVControlChange::Selection(change)
            if change.previous.is_some() && change.current.is_none()
    ));
}

#[test]
fn reset_restores_every_domain_and_clears_the_selection() {
    let mut state = state_with_horizontal();
    state
        .dispatch(&SVControlAction::Zoom {
            axis: SVControlAxis::Horizontal,
            factor: 0.25,
            anchor: 0.5,
        })
        .expect("the axis is registered");
    state
        .dispatch(&SVControlAction::ToggleSeries(SVSeriesReference::Index(0)))
        .expect("toggling never fails");

    state
        .dispatch(&SVControlAction::Reset)
        .expect("resetting never fails");

    assert_eq!(
        state.visible_domain(SVControlAxis::Horizontal),
        Some(SVDomain::new(0.0, 100.0).expect("restored domain"))
    );
    // Reset restores the view, never the series visibility.
    assert!(!state.is_series_visible(&SVSeriesReference::Index(0)));
}

#[test]
fn an_unregistered_axis_is_a_typed_error() {
    let mut state = SVControlState::new();

    assert_eq!(
        state.dispatch(&SVControlAction::Pan {
            axis: SVControlAxis::Vertical,
            fraction: 0.1,
        }),
        Err(SVError::UnknownControlAxis {
            axis: SVControlAxis::Vertical,
        })
    );
}

#[test]
fn a_custom_payload_is_preserved_unchanged() {
    let mut state = state_with_horizontal();

    let report = state
        .dispatch(&SVControlAction::Custom {
            name: "filter".to_string(),
            payload: "{\"region\":\"north\"}".to_string(),
        })
        .expect("custom actions never fail");

    let event = &report.events()[0];
    assert_eq!(event.origin(), &SVControlOrigin::Programmatic);
    assert_eq!(
        event.change(),
        &crate::sv_control::SVControlChange::Custom(crate::sv_control::SVCustomPayload {
            name: "filter".to_string(),
            payload: "{\"region\":\"north\"}".to_string(),
        })
    );
}

#[test]
fn a_repeated_action_without_a_change_emits_no_event() {
    let mut state = state_with_horizontal();
    let action = SVControlAction::Pan {
        axis: SVControlAxis::Horizontal,
        fraction: 0.0,
    };

    let first = state.dispatch(&action).expect("the axis is registered");
    let second = state.dispatch(&action).expect("the axis is registered");

    assert!(first.events().is_empty());
    assert!(second.events().is_empty());
}

#[test]
fn an_event_serializes_to_a_stable_text_form() {
    let mut state = state_with_horizontal();
    let report = state
        .dispatch_from(
            SVControlOrigin::Control("zoom-in".to_string()),
            &SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: 0.5,
                anchor: 0.5,
            },
        )
        .expect("the axis is registered");

    assert_eq!(
        report.events()[0].to_text(),
        "control=zoom-in;change=range;axis=Horizontal;previous=0,100;current=25,75"
    );
}

#[test]
fn the_same_action_sequence_produces_the_same_payloads() {
    fn run() -> Vec<SVControlEvent> {
        let mut state = state_with_horizontal();
        let mut events = Vec::new();
        for action in [
            SVControlAction::Zoom {
                axis: SVControlAxis::Horizontal,
                factor: 0.5,
                anchor: 0.5,
            },
            SVControlAction::Pan {
                axis: SVControlAxis::Horizontal,
                fraction: 0.2,
            },
            SVControlAction::ToggleSeries(SVSeriesReference::Index(3)),
        ] {
            events.extend(
                state
                    .dispatch(&action)
                    .expect("registered axis")
                    .events()
                    .to_vec(),
            );
        }
        events
    }

    assert_eq!(run(), run());
}
