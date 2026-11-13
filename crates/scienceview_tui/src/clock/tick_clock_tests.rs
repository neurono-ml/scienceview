//! The clock rate, elapsed-time advancement and the slow-terminal rule.

use super::SVTickClock;
use super::SVTickOutcome;
use std::time::Duration;

#[test]
fn a_tick_shorter_than_the_interval_is_idle() {
    let mut clock = SVTickClock::new(Duration::from_millis(16));

    assert_eq!(clock.tick(Duration::from_millis(8)), SVTickOutcome::Idle);
    assert_eq!(clock.position(), Duration::from_millis(8));
}

#[test]
fn accumulated_short_ticks_do_not_lose_time() {
    let mut clock = SVTickClock::new(Duration::from_millis(16));

    assert_eq!(clock.tick(Duration::from_millis(10)), SVTickOutcome::Idle);
    assert_eq!(
        clock.tick(Duration::from_millis(10)),
        SVTickOutcome::Frame(Duration::from_millis(20))
    );
}

#[test]
fn a_slow_tick_advances_by_the_elapsed_time_not_one_interval() {
    let mut clock = SVTickClock::new(Duration::from_millis(16));

    assert_eq!(
        clock.tick(Duration::from_millis(250)),
        SVTickOutcome::Frame(Duration::from_millis(250))
    );
    assert_eq!(clock.position(), Duration::from_millis(250));
}

#[test]
fn a_zero_interval_never_yields_more_than_one_frame_per_tick() {
    let mut clock = SVTickClock::new(Duration::ZERO);

    assert_eq!(clock.frame_interval(), Duration::from_millis(1));
    assert_eq!(
        clock.advance(Duration::from_secs(10)),
        Some(Duration::from_secs(10))
    );
    assert_eq!(clock.advance(Duration::ZERO), None);
}

#[test]
fn a_frame_rate_becomes_its_interval() {
    let clock = SVTickClock::from_frames_per_second(50);

    assert_eq!(clock.frame_interval(), Duration::from_millis(20));
    assert_eq!(
        SVTickClock::from_frames_per_second(0).frame_interval(),
        Duration::from_secs(1)
    );
}

#[test]
fn reset_returns_the_clock_to_the_start() {
    let mut clock = SVTickClock::new(Duration::from_millis(10));
    clock.tick(Duration::from_millis(10));

    clock.reset();

    assert_eq!(clock.position(), Duration::ZERO);
    assert_eq!(clock.advance(Duration::from_millis(5)), None);
}
