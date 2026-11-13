//! Playback controls bind to the animation timeline through the same actions.

use std::sync::{Arc, Mutex};

use crate::sv_animation::{
    SVAnimationAction, SVAnimationBuilder, SVDuration, SVKeyframeDelta, SVPlaybackState,
};
use crate::sv_control::{
    SVControlAction, SVControlBuilder, SVControlEvent, SVControlState, SVPlaybackCommand,
    SVWidgetAdapter,
};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkProperty;

fn animation() -> crate::sv_animation::SVAnimation {
    let first =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 0.0)]).expect("finite values");
    let second =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 10.0)]).expect("finite values");
    SVAnimationBuilder::new()
        .deltas([first, second])
        .build()
        .expect("a fixed timeline builds")
}

#[test]
fn a_play_control_starts_the_animation_and_emits_a_playback_event() {
    let mut session =
        crate::sv_control::SVControlSession::new(SVControlState::new()).with_animation(animation());
    let control = SVControlBuilder::playback(SVPlaybackCommand::Play)
        .build()
        .expect("the configuration is complete");

    let events = session
        .dispatch_surface(&control)
        .expect("the animation accepts play")
        .to_vec();

    assert_eq!(
        session
            .animation()
            .expect("an animation is attached")
            .state(),
        SVPlaybackState::Playing
    );
    assert_eq!(events.len(), 1);
    assert!(matches!(
        events[0].change(),
        crate::sv_control::SVControlChange::Playback(_)
    ));
}

#[test]
fn pause_step_seek_and_speed_reach_the_timeline() {
    let mut session =
        crate::sv_control::SVControlSession::new(SVControlState::new()).with_animation(animation());

    session
        .dispatch(&SVControlAction::SetPlayback(SVAnimationAction::Pause))
        .expect("pause is valid");
    session
        .dispatch(&SVControlAction::SetPlayback(SVAnimationAction::Step {
            forward: true,
        }))
        .expect("step is valid");
    session
        .dispatch(&SVControlAction::SetPlayback(SVAnimationAction::Seek(
            SVDuration::milliseconds(500),
        )))
        .expect("seek is valid");
    session
        .dispatch(&SVControlAction::SetPlayback(SVAnimationAction::SetSpeed(
            2.0,
        )))
        .expect("speed is valid");

    let animation = session.animation().expect("an animation is attached");
    assert_eq!(animation.state(), SVPlaybackState::Paused);
    assert_eq!(animation.position_seconds(), 0.5);
    assert_eq!(animation.speed(), 2.0);
}

#[test]
fn an_invalid_speed_leaves_the_timeline_untouched() {
    let mut session =
        crate::sv_control::SVControlSession::new(SVControlState::new()).with_animation(animation());
    let before = session.animation().expect("attached").speed();

    assert_eq!(
        session.dispatch(&SVControlAction::SetPlayback(SVAnimationAction::SetSpeed(
            0.0
        ))),
        Err(SVError::InvalidPlaybackSpeed)
    );
    assert_eq!(session.animation().expect("attached").speed(), before);
}

#[test]
fn a_playback_action_without_an_animation_is_a_typed_error() {
    let mut session = crate::sv_control::SVControlSession::new(SVControlState::new());

    assert_eq!(
        session.dispatch(&SVControlAction::SetPlayback(SVAnimationAction::Play)),
        Err(SVError::MissingAnimationTimeline)
    );
}

#[test]
fn subscribers_receive_every_event_and_can_unsubscribe() {
    let mut session =
        crate::sv_control::SVControlSession::new(SVControlState::new()).with_animation(animation());
    let seen: Arc<Mutex<Vec<SVControlEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let subscription = session.subscribe(move |event: &SVControlEvent| {
        sink.lock()
            .expect("the sink is not poisoned")
            .push(event.clone());
    });

    session
        .dispatch(&SVControlAction::SetPlayback(SVAnimationAction::Play))
        .expect("play is valid");
    assert_eq!(seen.lock().expect("not poisoned").len(), 1);

    session.unsubscribe(subscription);
    session
        .dispatch(&SVControlAction::SetPlayback(SVAnimationAction::Pause))
        .expect("pause is valid");
    assert_eq!(seen.lock().expect("not poisoned").len(), 1);
}

#[test]
fn an_external_widget_drives_the_figure_and_observes_the_same_events() {
    let mut session =
        crate::sv_control::SVControlSession::new(SVControlState::new()).with_animation(animation());
    let seen: Arc<Mutex<Vec<SVControlEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    session.subscribe(move |event: &SVControlEvent| {
        sink.lock()
            .expect("the sink is not poisoned")
            .push(event.clone());
    });
    let widget = SVWidgetAdapter::new(
        "external-play",
        SVControlAction::SetPlayback(SVAnimationAction::Play),
    );

    let events = session
        .dispatch_surface(&widget)
        .expect("the widget dispatches play");

    assert_eq!(
        events[0].origin(),
        &crate::sv_control::SVControlOrigin::ExternalWidget("external-play".to_string())
    );
    assert_eq!(seen.lock().expect("not poisoned").len(), 1);
    assert_eq!(
        session.animation().expect("attached").state(),
        SVPlaybackState::Playing
    );
}
