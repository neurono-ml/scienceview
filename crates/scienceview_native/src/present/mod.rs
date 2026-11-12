//! Publishing a new frame into a window that is already open.
//!
//! A streaming figure renders new data on its own thread; the window must show
//! the result without reopening. A producer/consumer channel carries exactly
//! one pending frame between them, and publishing wakes the window so its idle
//! clock redraws. The channel is pure standard library, so the hand-off is
//! tested headlessly with a recording presenter.

mod frame_channel;
#[cfg(any(feature = "window", test))]
mod present_loop;

#[cfg(any(feature = "window", test))]
pub(crate) use frame_channel::SVFrameChannel;
pub use frame_channel::SVFramePublisher;
#[cfg(any(feature = "window", test))]
pub(crate) use present_loop::SVPresentLoop;
#[cfg(any(feature = "window", test))]
pub(crate) use present_loop::SVPresenter;

#[cfg(test)]
#[path = "frame_channel_tests.rs"]
mod frame_channel_tests;
#[cfg(test)]
#[path = "present_loop_tests.rs"]
mod present_loop_tests;
