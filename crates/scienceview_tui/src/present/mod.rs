//! Presentation: the interactive terminal session and its fallback.
//!
//! Without the `terminal` feature the backend still renders headless frames;
//! only `show` reports that the interactive layer is not compiled.

#[cfg(feature = "terminal")]
mod buffer_fill;
#[cfg(feature = "terminal")]
#[cfg(test)]
mod buffer_fill_tests;
mod show;
#[cfg(feature = "terminal")]
mod terminal_session;

pub use show::sv_show_tui_request;
#[cfg(feature = "terminal")]
pub use terminal_session::SVTerminalEvent;
#[cfg(feature = "terminal")]
pub use terminal_session::SVTerminalSession;
