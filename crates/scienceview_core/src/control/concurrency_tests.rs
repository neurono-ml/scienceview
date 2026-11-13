//! Controls, gestures and the reducer are safe to share across threads.

use std::sync::{Arc, Mutex};
use std::thread;

use crate::sv_animation::{SVAnimationAction, SVAnimationBuilder, SVKeyframeDelta};
use crate::sv_control::{
    SVControlAction, SVControlAxis, SVControlEvent, SVControlSession, SVControlState,
    sv_default_gesture_map,
};
use crate::sv_mark::SVMarkProperty;
use crate::sv_scale::SVDomain;

fn sv_assert_send_sync<T: Send + Sync>() {}

#[test]
fn the_public_values_are_send_and_sync() {
    sv_assert_send_sync::<SVControlAction>();
    sv_assert_send_sync::<SVControlAxis>();
    sv_assert_send_sync::<SVControlEvent>();
    sv_assert_send_sync::<crate::sv_control::SVControlState>();
    sv_assert_send_sync::<crate::sv_control::SVGestureMap>();
}

#[test]
fn concurrent_dispatches_leave_the_state_deterministic() {
    let mut state = SVControlState::new();
    state
        .register_axis(
            SVControlAxis::Horizontal,
            SVDomain::new(0.0, 100.0).expect("ordered domain"),
        )
        .expect("the axis is new");
    let shared = Arc::new(Mutex::new(state));

    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                for _ in 0..25 {
                    let mut state = shared.lock().expect("the state is not poisoned");
                    state
                        .dispatch(&SVControlAction::ToggleSeries(
                            crate::sv_control::SVSeriesReference::Index(worker),
                        ))
                        .expect("toggling never fails");
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().expect("the worker does not panic");
    }

    let state = shared.lock().expect("the state is not poisoned");
    let visible: Vec<usize> = (0..8)
        .filter(|index| {
            state.is_series_visible(&crate::sv_control::SVSeriesReference::Index(*index))
        })
        .collect();
    // Every worker toggles its own series an odd number of times, so every
    // series ends hidden, whatever the interleaving was.
    assert!(visible.is_empty());
}

#[test]
fn a_shared_session_delivers_events_to_a_single_subscriber_under_contention() {
    let first =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 0.0)]).expect("finite values");
    let second =
        SVKeyframeDelta::from_pairs(&[(SVMarkProperty::Position, 10.0)]).expect("finite values");
    let animation = SVAnimationBuilder::new()
        .deltas([first, second])
        .build()
        .expect("a fixed timeline builds");
    let mut session = SVControlSession::new(SVControlState::new()).with_animation(animation);
    let seen: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let sink = Arc::clone(&seen);
    session.subscribe(move |_event: &SVControlEvent| {
        *sink.lock().expect("the sink is not poisoned") += 1;
    });
    let shared = Arc::new(Mutex::new(session));

    let workers: Vec<_> = (0..4)
        .map(|worker| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                for step in 0..20 {
                    let action = if (worker + step) % 2 == 0 {
                        SVAnimationAction::Play
                    } else {
                        SVAnimationAction::Pause
                    };
                    let mut session = shared.lock().expect("the session is not poisoned");
                    session
                        .dispatch(&SVControlAction::SetPlayback(action))
                        .expect("play and pause are always valid");
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().expect("the worker does not panic");
    }

    assert_eq!(*seen.lock().expect("the sink is not poisoned"), 80);
}

#[test]
fn a_gesture_map_resolves_identically_from_several_threads() {
    let map = Arc::new(sv_default_gesture_map());
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let map = Arc::clone(&map);
            thread::spawn(move || {
                map.resolve(&crate::sv_control::SVGesture::Key(
                    crate::sv_control::SVKeyboardKey::Home,
                ))
            })
        })
        .collect();

    for worker in workers {
        assert_eq!(
            worker.join().expect("the worker does not panic"),
            Some(SVControlAction::Reset)
        );
    }
}

#[test]
fn an_empty_state_reads_under_locks_without_a_panic() {
    let shared = Arc::new(Mutex::new(SVControlState::new()));
    let state = Arc::clone(&shared);
    let worker = thread::spawn(move || {
        let state = state.lock().expect("the state is not poisoned");
        state.axis_domain(SVControlAxis::Horizontal)
    });

    assert_eq!(worker.join().expect("the worker does not panic"), None);
}
