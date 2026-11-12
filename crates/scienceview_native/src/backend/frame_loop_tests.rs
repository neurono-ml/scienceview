//! The frame loop seam: a model clock drives presented frames.

use crate::backend::SVFrameLoop;
use crate::backend::SVFrameOutcome;
use crate::backend::SVPausedClock;

#[test]
fn a_paused_clock_performs_no_animation_work() {
    let mut frame_loop = SVFrameLoop::new(SVPausedClock);
    let mut callback_calls = 0;

    let outcome = frame_loop.run_frame(|_position| {
        callback_calls += 1;
        true
    });

    assert_eq!(outcome, SVFrameOutcome::Idle);
    assert_eq!(callback_calls, 0);
    assert_eq!(frame_loop.presented_frames(), 0);
    assert_eq!(frame_loop.idle_frames(), 1);
}

#[test]
fn an_advancing_clock_draws_the_frame_at_its_position() {
    let mut positions = vec![0.25, 0.5].into_iter();
    let clock = move || positions.next();
    let mut frame_loop = SVFrameLoop::new(clock);
    let mut drawn = Vec::new();

    let first = frame_loop.run_frame(|position| {
        drawn.push(position);
        true
    });
    let second = frame_loop.run_frame(|position| {
        drawn.push(position);
        true
    });
    let exhausted = frame_loop.run_frame(|position| {
        drawn.push(position);
        true
    });

    assert_eq!(first, SVFrameOutcome::Presented);
    assert_eq!(second, SVFrameOutcome::Presented);
    assert_eq!(exhausted, SVFrameOutcome::Idle);
    assert_eq!(drawn, vec![0.25, 0.5]);
    assert_eq!(frame_loop.presented_frames(), 2);
    assert_eq!(frame_loop.idle_frames(), 1);
}

#[test]
fn a_callback_that_draws_nothing_asks_for_no_repaint() {
    let mut positions = vec![1.0].into_iter();
    let mut frame_loop = SVFrameLoop::new(move || positions.next());

    let outcome = frame_loop.run_frame(|_position| false);

    assert_eq!(outcome, SVFrameOutcome::Idle);
    assert_eq!(frame_loop.presented_frames(), 0);
    assert_eq!(frame_loop.idle_frames(), 1);
}

#[test]
fn the_loop_never_advances_past_the_clock() {
    let mut frame_loop = SVFrameLoop::new(|| -> Option<f64> { None });
    let mut calls = 0;

    for _ in 0..5 {
        frame_loop.run_frame(|_position| {
            calls += 1;
            true
        });
    }

    assert_eq!(calls, 0);
    assert_eq!(frame_loop.idle_frames(), 5);
}
