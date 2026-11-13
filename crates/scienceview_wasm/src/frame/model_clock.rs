//! The seam that lets the model own the clock.
//!
//! The browser supplies a timestamp; the model decides what position it means.
//! A paused model answers `None`, which is what makes a paused figure cost no
//! animation work.

/// The model-side clock of the WebAssembly target.
///
/// The animation frame never advances a position by itself: it passes its
/// timestamp to [`advance`](SVWasmModelClock::advance), and the model returns
/// the playback position it wants rendered. Returning `None` means nothing
/// changed.
pub trait SVWasmModelClock: Send {
    /// Maps one animation-frame timestamp, in seconds, to a playback position.
    fn advance(&mut self, frame_time: f64) -> Option<f64>;
}

impl<F> SVWasmModelClock for F
where
    F: FnMut(f64) -> Option<f64> + Send,
{
    fn advance(&mut self, frame_time: f64) -> Option<f64> {
        self(frame_time)
    }
}

/// A clock that never advances, for a static or paused presentation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVPausedWasmClock;

impl SVWasmModelClock for SVPausedWasmClock {
    fn advance(&mut self, _frame_time: f64) -> Option<f64> {
        None
    }
}
