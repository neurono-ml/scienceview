//! The frame loop seam: where the clock comes from, and when a frame is drawn.
//!
//! The clock belongs to the model (design D2). The loop never owns a timer: it
//! asks a caller-supplied [`SVFrameClock`] for the playback position of the
//! next frame and only invokes the frame callback when there is a new position.
//! A paused timeline therefore costs no animation work, and the renderer does
//! not depend on any animation or transition type — the seam accepts any value
//! that can answer "what position is next?".

/// The source of playback positions for the frame loop.
///
/// A closure `FnMut() -> Option<f64>` implements the trait, so a caller wraps
/// its own model without adapting an animation type.
pub trait SVFrameClock: Send {
    /// Returns the position of the next frame, or `None` when nothing changed.
    fn advance(&mut self) -> Option<f64>;
}

impl<F> SVFrameClock for F
where
    F: FnMut() -> Option<f64> + Send,
{
    fn advance(&mut self) -> Option<f64> {
        self()
    }
}

/// A clock that never advances, for a static presentation.
///
/// `show` uses it: the window opens on the first frame and redraws only when
/// the window system asks, so a static figure performs no animation work.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SVPausedClock;

impl SVFrameClock for SVPausedClock {
    fn advance(&mut self) -> Option<f64> {
        None
    }
}

/// What one turn of the frame loop did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVFrameOutcome {
    /// The clock advanced and the callback drew a frame.
    Presented,
    /// The clock was paused or the callback reported nothing to draw.
    Idle,
}

/// Drives frames from a model clock.
pub struct SVFrameLoop<C: SVFrameClock> {
    clock: C,
    presented_frames: u64,
    idle_frames: u64,
}

impl<C: SVFrameClock> SVFrameLoop<C> {
    /// Builds a loop around a clock.
    pub fn new(clock: C) -> Self {
        Self {
            clock,
            presented_frames: 0,
            idle_frames: 0,
        }
    }

    /// Runs one frame.
    ///
    /// The callback returns `true` when it drew something. When it returns
    /// `false`, or the clock reports no new position, the turn is
    /// [`SVFrameOutcome::Idle`] and the loop asks for no repaint.
    pub fn run_frame(&mut self, mut frame_callback: impl FnMut(f64) -> bool) -> SVFrameOutcome {
        let Some(position) = self.clock.advance() else {
            self.idle_frames = self.idle_frames.saturating_add(1);
            return SVFrameOutcome::Idle;
        };

        if frame_callback(position) {
            self.presented_frames = self.presented_frames.saturating_add(1);
            SVFrameOutcome::Presented
        } else {
            self.idle_frames = self.idle_frames.saturating_add(1);
            SVFrameOutcome::Idle
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
