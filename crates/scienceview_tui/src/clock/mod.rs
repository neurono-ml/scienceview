//! The tick clock: elapsed-time animation for a terminal frame loop.

mod tick_clock;

pub use tick_clock::SVTickClock;
pub use tick_clock::SVTickOutcome;

#[cfg(test)]
mod tick_clock_tests;
