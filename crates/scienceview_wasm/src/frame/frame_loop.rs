//! One turn of the animation-frame loop: ask the clock, then draw or idle.

use super::model_clock::SVWasmModelClock;

/// What one animation frame did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVWasmFrameOutcome {
    /// The clock advanced and the draw callback presented a frame.
    Presented,
    /// The clock was paused, or the callback reported nothing to draw.
    Idle,
}

/// Drives frames from the model clock of the WebAssembly target.
pub struct SVWasmFrameLoop<C: SVWasmModelClock> {
    clock: C,
    presented_frames: u64,
    idle_frames: u64,
}

impl<C: SVWasmModelClock> SVWasmFrameLoop<C> {
    /// Builds a loop around a model clock.
    pub fn new(clock: C) -> Self {
        Self {
            clock,
            presented_frames: 0,
            idle_frames: 0,
        }
    }

    /// Runs one animation frame at `frame_time` seconds.
    ///
    /// The callback returns `true` when it drew something. When it returns
    /// `false`, or the clock reports no position, the turn is
    /// [`SVWasmFrameOutcome::Idle`] and the caller asks for no repaint.
    pub fn run_frame(
        &mut self,
        frame_time: f64,
        mut draw: impl FnMut(f64) -> bool,
    ) -> SVWasmFrameOutcome {
        let Some(position) = self.clock.advance(frame_time) else {
            self.idle_frames = self.idle_frames.saturating_add(1);
            return SVWasmFrameOutcome::Idle;
        };

        if draw(position) {
            self.presented_frames = self.presented_frames.saturating_add(1);
            SVWasmFrameOutcome::Presented
        } else {
            self.idle_frames = self.idle_frames.saturating_add(1);
            SVWasmFrameOutcome::Idle
        }
    }

    /// Returns how many frames were presented.
    pub const fn presented_frames(&self) -> u64 {
        self.presented_frames
    }

    /// Returns how many turns did no animation work.
    pub const fn idle_frames(&self) -> u64 {
        self.idle_frames
    }
}
