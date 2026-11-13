//! The integration contract: a third-party widget behaves exactly like the
//! built-in control, and both observe the same events.

use std::sync::{Arc, Mutex};

use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlBuilder, SVControlEvent, SVControlPlacement,
    SVControlSession, SVControlState, SVControlStyle, SVControlSurface, SVSeriesReference,
    SVWidgetAdapter,
};
use crate::sv_error::SVError;
use crate::sv_scale::SVDomain;

fn state() -> SVControlState {
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
fn an_external_widget_mutates_the_state_like_a_built_in_control() {
    let built_in = SVControlBuilder::zoom_in()
        .build()
        .expect("the configuration is complete");
    let external = SVWidgetAdapter::new("framework-zoom", built_in.action());

    let mut first = state();
    let mut second = state();
    built_in
        .dispatch_into(&mut first)
        .expect("the built-in control dispatches");
    external
        .dispatch_into(&mut second)
        .expect("the widget dispatches");

    assert_eq!(
        first.visible_domain(SVControlAxis::Horizontal),
        second.visible_domain(SVControlAxis::Horizontal)
    );
}

#[test]
fn a_disabled_widget_is_rejected_with_a_typed_error() {
    let mut widget = SVWidgetAdapter::new("framework-zoom", SVControlAction::Reset);
    widget.set_enabled(false);

    assert_eq!(
        widget.activate(),
        Err(SVError::ControlDisabled {
            name: "framework-zoom".to_string(),
        })
    );
}

#[test]
fn a_host_placed_control_dispatches_the_same_action_as_an_in_frame_one() {
    let in_frame = SVControlBuilder::pan()
        .placement(SVControlPlacement::InFrame)
        .target_axis(SVControlAxis::Horizontal)
        .step(0.25)
        .build()
        .expect("the configuration is complete");
    let in_host = SVControlBuilder::pan()
        .placement(SVControlPlacement::Host)
        .target_axis(SVControlAxis::Horizontal)
        .step(0.25)
        .build()
        .expect("the configuration is complete");

    assert_eq!(in_frame.action(), in_host.action());
    assert_ne!(in_frame.placement(), in_host.placement());
}

#[test]
fn a_control_carries_a_script_that_receives_the_event_payload() {
    let mut session = SVControlSession::new(state());
    let seen: Arc<Mutex<Vec<SVControlEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let sink_script = move |event: &SVControlEvent| {
        sink.lock()
            .expect("the sink is not poisoned")
            .push(event.clone());
    };
    session.subscribe(sink_script);
    let mut control = SVControlBuilder::zoom_in()
        .build()
        .expect("the configuration is complete");
    control.attach_script("function on(event) { log(event.to_text()); }");

    session
        .dispatch_surface(&control)
        .expect("the control dispatches");

    assert!(
        control
            .script()
            .is_some_and(|source| source.contains("on(event)"))
    );
    let seen = seen.lock().expect("the sink is not poisoned");
    assert_eq!(seen.len(), 1);
    assert!(
        seen[0]
            .to_text()
            .starts_with("control=zoom-in;change=range;")
    );
}

#[test]
fn a_series_toggle_and_a_range_slider_emit_their_own_payloads() {
    let mut session = SVControlSession::new(state());
    let toggle = SVControlBuilder::series_toggle(SVSeriesReference::Name("alpha".to_string()))
        .build()
        .expect("the configuration is complete");
    let slider = SVControlBuilder::range_slider(SVControlAxis::Horizontal)
        .range(crate::sv_control::SVControlRange::new(10.0, 40.0).expect("ordered range"))
        .build()
        .expect("the configuration is complete");

    let toggled = session
        .dispatch_surface(&toggle)
        .expect("toggling never fails")
        .to_vec();
    let ranged = session
        .dispatch_surface(&slider)
        .expect("the slider dispatches")
        .to_vec();

    assert!(matches!(
        toggled[0].change(),
        crate::sv_control::SVControlChange::SeriesVisibility(_)
    ));
    assert!(matches!(
        ranged[0].change(),
        crate::sv_control::SVControlChange::Range(change)
            if change.current.lower() == 10.0 && change.current.upper() == 40.0
    ));
    assert_eq!(
        session.state().visible_range(SVControlAxis::Horizontal),
        Some(crate::sv_control::SVControlRange::new(10.0, 40.0).expect("ordered range"))
    );
}

#[test]
fn the_style_can_be_overridden_explicitly() {
    let style = SVControlStyle::built_in().with_accent(crate::sv_scale::SVColour::new(1, 2, 3, 4));
    let control = SVControlBuilder::reset()
        .style(style.clone())
        .build()
        .expect("the configuration is complete");

    assert_eq!(
        control.style().accent(),
        crate::sv_scale::SVColour::new(1, 2, 3, 4)
    );
    assert_eq!(control.style().spacing(), style.spacing());
}

#[cfg(feature = "theme")]
#[test]
fn a_compiled_theme_supplies_the_control_style() {
    let theme = scienceview_theme::SVTheme::builder()
        .build()
        .expect("the default theme compiles");

    let style = SVControlStyle::from_theme(&theme);

    assert_eq!(style.text_size(), 12.0);
    assert!(style.spacing() > 0.0);
    assert!(style.radius() > 0.0);
}
