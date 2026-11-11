//! The bounded rolling window that backs a streaming representation.

mod policy;
mod rolling_window;

pub use policy::{SVWindowOverflow, SVWindowPolicy};
#[cfg(any(feature = "streaming", feature = "streaming-tokio"))]
pub(crate) use rolling_window::sv_lock;
pub use rolling_window::{SVRollingWindow, SVWindowReadGuard};
