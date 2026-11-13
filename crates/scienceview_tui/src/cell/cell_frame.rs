//! A rendered terminal frame: the comparable output of the cell rasterizer.
//!
//! The frame is the acceptance surface of the backend. It carries its size in
//! cells and the glyph plus colours of every cell, so a test asserts on a value
//! instead of scraping a terminal, and the same value is what `render` returns
//! to a caller as bytes.

use super::ansi::SVAnsiColourDepth;
use super::ansi::sv_ansi_colour_code;
use super::cell_definition::SVCell;
use scienceview_core::sv_scale::SVColour;
use std::fmt::Write;

/// One rendered frame on a terminal cell grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVCellFrame {
    width: usize,
    height: usize,
    cells: Vec<SVCell>,
}

impl SVCellFrame {
    /// Builds a blank frame of `width` by `height` cells.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![SVCell::blank(); width * height],
        }
    }

    /// Returns the frame width in cells.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the frame height in cells.
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns `true` when the frame has no cell.
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns one cell, or `None` outside the frame.
    pub fn cell(&self, column: usize, row: usize) -> Option<&SVCell> {
        if column >= self.width || row >= self.height {
            return None;
        }
        self.cells.get(row * self.width + column)
    }

    /// Returns one cell for mutation, or `None` outside the frame.
    pub(crate) fn cell_mut(&mut self, column: usize, row: usize) -> Option<&mut SVCell> {
        if column >= self.width || row >= self.height {
            return None;
        }
        self.cells.get_mut(row * self.width + column)
    }

    /// Returns the cells in row-major order.
    pub fn cells(&self) -> &[SVCell] {
        &self.cells
    }

    /// Returns the cells for mutation, in row-major order.
    pub(crate) fn cells_mut(&mut self) -> &mut [SVCell] {
        &mut self.cells
    }

    /// Returns one row as plain text.
    ///
    /// A continuation cell contributes nothing, so the string occupies exactly
    /// the frame's display columns even when a glyph is two columns wide.
    pub fn row_text(&self, row: usize) -> String {
        if row >= self.height {
            return String::new();
        }
        let start = row * self.width;
        let end = start + self.width;
        self.cells
            .get(start..end)
            .map(|cells| {
                cells
                    .iter()
                    .filter(|cell| !cell.is_continuation())
                    .map(SVCell::symbol)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns the whole frame as plain text, one row per line.
    ///
    /// This is the honest export path of a text backend: the caller writes the
    /// string itself, because the capability matrix declares no export format.
    pub fn to_text(&self) -> String {
        (0..self.height)
            .map(|row| self.row_text(row))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Returns the whole frame as ANSI text, one row per line.
    ///
    /// Every colour change emits an SGR sequence and every row ends with a
    /// reset, so the output can be written to a file and replayed by `cat`.
    pub fn to_ansi(&self, depth: SVAnsiColourDepth) -> String {
        let mut output = String::new();

        for row in 0..self.height {
            let mut foreground: Option<SVColour> = None;
            let mut background: Option<SVColour> = None;

            for column in 0..self.width {
                let Some(cell) = self.cell(column, row) else {
                    continue;
                };
                if cell.is_continuation() {
                    continue;
                }

                if cell.foreground() != foreground {
                    if let Some(code) = cell
                        .foreground()
                        .and_then(|colour| sv_ansi_colour_code(colour, depth))
                    {
                        let _ = write!(output, "\u{1b}[{code}m");
                    } else {
                        output.push_str("\u{1b}[39m");
                    }
                    foreground = cell.foreground();
                }
                if cell.background() != background {
                    if let Some(code) = cell
                        .background()
                        .and_then(|colour| sv_ansi_background_code(colour, depth))
                    {
                        let _ = write!(output, "\u{1b}[{code}m");
                    } else {
                        output.push_str("\u{1b}[49m");
                    }
                    background = cell.background();
                }

                output.push(cell.symbol());
            }

            output.push_str("\u{1b}[0m\n");
        }

        output
    }

    /// Consumes the frame and returns its ANSI text as bytes.
    pub fn into_ansi_bytes(self, depth: SVAnsiColourDepth) -> Vec<u8> {
        self.to_ansi(depth).into_bytes()
    }
}

fn sv_ansi_background_code(colour: SVColour, depth: SVAnsiColourDepth) -> Option<String> {
    sv_ansi_colour_code(colour, depth).map(|code| match depth {
        SVAnsiColourDepth::TrueColor => code.replacen("38;2;", "48;2;", 1),
        SVAnsiColourDepth::Ansi256 => code.replacen("38;5;", "48;5;", 1),
        SVAnsiColourDepth::Ansi16 => {
            let foreground: u16 = code.parse().unwrap_or(39);
            let background = if foreground >= 90 {
                foreground - 90 + 100
            } else {
                foreground - 30 + 40
            };
            background.to_string()
        }
    })
}
