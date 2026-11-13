//! Drives a figure through the control kit without any user-interface toolkit.
//!
//! The example registers a horizontal axis, builds the standard controls, maps a
//! keyboard gesture to an action, dispatches through a session and prints the
//! event payloads a backend or a script would observe.

use scienceview::sv_animation::{SVAnimationBuilder, SVKeyframeDelta};
use scienceview::sv_control::{
    SVControlAxis, SVControlSession, SVControlState, SVGesture, SVKeyboardKey, SVSeriesReference,
    sv_default_gesture_map, sv_standard_controls,
};
use scienceview::sv_mark::SVMarkProperty;
use scienceview::sv_scale::SVDomain;

fn main() {
    let mut state = SVControlState::new();
    state
        .register_axis(
            SVControlAxis::Horizontal,
            SVDomain::new(0.0, 100.0).expect("ordered domain"),
        )
        .expect("the axis is new");
    state
        .register_axis(
            SVControlAxis::Vertical,
            SVDomain::new(-1.0, 1.0).expect("ordered domain"),
        )
        .expect("the axis is new");

    let first =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Opacity, 0.0)]).expect("finite values");
    let second =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Opacity, 1.0)]).expect("finite values");
    let animation = SVAnimationBuilder::new()
        .deltas([first, second])
        .build()
        .expect("a fixed timeline builds");
    let mut session = SVControlSession::new(state).with_animation(animation);
    session.subscribe(|event| println!("{event}"));

    let controls = sv_standard_controls([SVSeriesReference::Name("alpha".to_string())]);
    for control in &controls {
        session
            .dispatch_surface(control)
            .expect("a standard control dispatches");
    }

    let gestures = sv_default_gesture_map();
    for gesture in [
        SVGesture::Key(SVKeyboardKey::Plus),
        SVGesture::Key(SVKeyboardKey::ArrowRight),
        SVGesture::Wheel {
            axis: SVControlAxis::Horizontal,
            delta: 1.0,
        },
    ] {
        if let Some(action) = gestures.resolve(&gesture) {
            session
                .dispatch(&action)
                .expect("the gesture action is valid");
        }
    }

    println!(
        "visible horizontal range: {:?}",
        session.state().visible_domain(SVControlAxis::Horizontal)
    );
    println!(
        "playback state: {:?}",
        session.playback_state().expect("a timeline is attached")
    );
}
