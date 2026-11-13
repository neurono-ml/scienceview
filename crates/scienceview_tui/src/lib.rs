//! `scienceview_tui`: terminal (TUI) rendering backend for scienceview.
//!
//! This crate is one rendering backend of
//! [`scienceview`](https://github.com/neurono-ml/scienceview). Enabling the
//! matching root feature only makes the backend *available*; the figure still
//! selects it explicitly through `backend(...)`, because there is no automatic
//! backend.
//!
//! ## Two layers
//!
//! - A **pure cell rasterizer** ([`SVCellCanvas`], [`SVCellFrame`]) turns the
//!   backend-agnostic mark and decoration primitives into a grid of terminal
//!   cells. It needs no terminal, so every acceptance test compares golden
//!   frames in CI.
//! - An **interactive terminal layer** ([`SVTerminalSession`], behind the
//!   `terminal` feature) presents those cells through ratatui and crossterm,
//!   re-solves the layout on resize and reads keys.
//!
//! ## How a cell approximates a mark
//!
//! Paths and dots use **braille** dots, which give a 2 by 4 sub-cell grid per
//! character. Rectangles use **block** glyphs, and a fractional height becomes
//! a partial block (`▁` through `█`) rather than being rounded away. Text uses
//! the glyphs themselves, with East Asian wide characters taking two cells.
//!
//! ## The clock
//!
//! [`SVTickClock`] advances a caller-supplied frame position by the **elapsed**
//! time, never by a fixed tick, and yields at most one frame per configured
//! interval — a slow terminal falls behind in time, not in accuracy.
//!
//! ## Capabilities
//!
//! The backend declares events, streaming, in-place update and presentation,
//! and no export format. `render` and `update` return the ANSI cell frame,
//! `show` presents it, and `save` reports the missing capability instead of
//! writing a file the matrix never promised. A caller that wants a text file
//! uses [`SVCellFrame::to_text`] or [`SVCellFrame::to_ansi`] directly.
//!
//! ## Rendering through the registry
//!
//! ```ignore
//! sv_register_tui_backend();
//! let frame = figure.render()?;
//! ```

mod backend;
mod cell;
mod clock;
mod figure;
mod present;

pub use backend::SVTuiBackend;
pub use backend::SVTuiBackendProvider;
pub use backend::sv_register_tui_backend;
pub use backend::sv_tui_default_viewport;
pub use backend::sv_tui_viewport;
pub use cell::SVAnsiColourDepth;
pub use cell::SVCell;
pub use cell::SVCellCanvas;
pub use cell::SVCellFrame;
pub use cell::sv_ansi_16_index;
pub use cell::sv_ansi_256_index;
pub use cell::sv_ansi_colour_code;
pub use cell::sv_cell_display_width;
pub use clock::SVTickClock;
pub use clock::SVTickOutcome;

#[cfg(feature = "terminal")]
pub use present::SVTerminalEvent;
#[cfg(feature = "terminal")]
pub use present::SVTerminalSession;

#[cfg(test)]
#[path = "crate_tests/mod.rs"]
mod crate_tests;
