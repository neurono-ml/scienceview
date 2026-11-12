//! The winit window and its softbuffer surface.

mod native_application;
mod native_window;
mod present_surface;

pub use native_window::SVNativeWindow;
pub(crate) use native_window::sv_present_native_request;

#[cfg(test)]
#[path = "window_tests.rs"]
mod window_tests;
