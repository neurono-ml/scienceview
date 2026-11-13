//! The frame loop: the browser time reaches the model, and a paused model idles.

use crate::SVPausedWasmClock;
use crate::SVWasmFrameLoop;
use crate::SVWasmFrameOutcome;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

#[test]
fn a_paused_clock_performs_no_animation_work() {
    let mut frame_loop = SVWasmFrameLoop::new(SVPausedWasmClock);
    let mut draws = 0;

    let outcome = frame_loop.run_frame(1.0, |_position| {
        draws += 1;
        true
    });

    assert_eq!(outcome, SVWasmFrameOutcome::Idle);
    assert_eq!(draws, 0);
    assert_eq!(frame_loop.presented_frames(), 0);
    assert_eq!(frame_loop.idle_frames(), 1);
}

#[test]
fn the_browser_timestamp_reaches_the_model_clock() {
    let seen_times = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&seen_times);
    let mut frame_loop = SVWasmFrameLoop::new(move |frame_time: f64| {
        recorded
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(frame_time);
        Some(frame_time / 2.0)
    });
    let mut positions = Vec::new();

    frame_loop.run_frame(0.5, |position| {
        positions.push(position);
        true
    });
    frame_loop.run_frame(1.5, |position| {
        positions.push(position);
        true
    });

    assert_eq!(positions, vec![0.25, 0.75]);
    assert_eq!(
        *seen_times
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()),
        vec![0.5, 1.5]
    );
    assert_eq!(frame_loop.presented_frames(), 2);
}

#[test]
fn a_callback_that_draws_nothing_asks_for_no_repaint() {
    let mut frame_loop = SVWasmFrameLoop::new(|_frame_time: f64| Some(1.0));

    let outcome = frame_loop.run_frame(0.1, |_position| false);

    assert_eq!(outcome, SVWasmFrameOutcome::Idle);
    assert_eq!(frame_loop.presented_frames(), 0);
    assert_eq!(frame_loop.idle_frames(), 1);
}

#[test]
fn pausing_after_a_frame_stops_the_next_one() {
    let paused = Arc::new(AtomicBool::new(false));
    let model = Arc::clone(&paused);
    let mut frame_loop = SVWasmFrameLoop::new(move |frame_time: f64| {
        if model.load(Ordering::Relaxed) {
            None
        } else {
            Some(frame_time)
        }
    });
    let mut draws = 0;

    let first = frame_loop.run_frame(0.1, |_position| {
        draws += 1;
        true
    });
    paused.store(true, Ordering::Relaxed);
    let second = frame_loop.run_frame(0.2, |_position| {
        draws += 1;
        true
    });

    assert_eq!(first, SVWasmFrameOutcome::Presented);
    assert_eq!(second, SVWasmFrameOutcome::Idle);
    assert_eq!(draws, 1);
    assert_eq!(frame_loop.presented_frames(), 1);
}

#[test]
fn the_host_target_drives_the_loop_without_an_animation_frame() {
    let mut frame_loop = SVWasmFrameLoop::new(|frame_time: f64| Some(frame_time));

    let outcome = frame_loop.run_frame(2.0, |position| position > 0.0);

    assert_eq!(outcome, SVWasmFrameOutcome::Presented);
    assert_eq!(frame_loop.presented_frames(), 1);
}
