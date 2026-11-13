//! The terminal cell grid: cells, frames, ANSI colour and the rasterizer.

mod ansi;
mod blocks;
mod braille;
mod canvas;
mod cell_definition;
mod cell_display_width;
mod cell_frame;
mod decoration_drawing;
mod mark_drawing;
mod text_drawing;

pub use ansi::SVAnsiColourDepth;
pub use ansi::sv_ansi_16_index;
pub use ansi::sv_ansi_256_index;
pub use ansi::sv_ansi_colour_code;
pub use canvas::SVCellCanvas;
pub use cell_definition::SVCell;
pub use cell_display_width::sv_cell_display_width;
pub use cell_frame::SVCellFrame;

#[cfg(test)]
mod ansi_tests;
#[cfg(test)]
mod canvas_tests;
#[cfg(test)]
mod cell_frame_tests;
