//! The backend implementation, its registry provider and the frame loop seam.

mod frame_loop;
mod native_backend;
mod provider;

pub use frame_loop::SVFrameClock;
pub use frame_loop::SVFrameLoop;
pub use frame_loop::SVFrameOutcome;
pub use frame_loop::SVPausedClock;
pub use native_backend::SVNativeBackend;
pub use native_backend::sv_native_default_viewport;
pub use provider::SVNativeBackendProvider;
pub use provider::sv_register_native_backend;

#[cfg(test)]
#[path = "backend_tests.rs"]
mod backend_tests;
#[cfg(test)]
#[path = "frame_loop_tests.rs"]
mod frame_loop_tests;
