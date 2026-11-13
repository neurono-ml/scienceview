//! One cell of the terminal grid: a glyph plus optional foreground and
//! background colours.

use scienceview_core::sv_scale::SVColour;

/// One terminal cell.
///
/// A wide glyph (for example a CJK ideograph) occupies its own cell and marks
/// the following cell as a continuation, so a row keeps its column alignment
/// even though the glyph is two columns wide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVCell {
    symbol: char,
    foreground: Option<SVColour>,
    background: Option<SVColour>,
    continuation: bool,
}

impl SVCell {
    /// A blank cell painted with the terminal defaults.
    pub const fn blank() -> Self {
        Self {
            symbol: ' ',
            foreground: None,
            background: None,
            continuation: false,
        }
    }

    /// A cell carrying one glyph.
    pub const fn new(symbol: char) -> Self {
        Self {
            symbol,
            foreground: None,
            background: None,
            continuation: false,
        }
    }

    /// Returns the glyph.
    pub const fn symbol(&self) -> char {
        self.symbol
    }

    /// Returns the foreground colour, when the cell overrides the default.
    pub const fn foreground(&self) -> Option<SVColour> {
        self.foreground
    }

    /// Returns the background colour, when the cell overrides the default.
    pub const fn background(&self) -> Option<SVColour> {
        self.background
    }

    /// Returns `true` when this cell is the second column of a wide glyph.
    pub const fn is_continuation(&self) -> bool {
        self.continuation
    }

    /// Returns `true` when the cell draws nothing of its own.
    pub fn is_blank(&self) -> bool {
        !self.continuation && self.symbol == ' '
    }

    /// Replaces the glyph.
    pub fn set_symbol(&mut self, symbol: char) {
        self.symbol = symbol;
        self.continuation = false;
    }

    /// Marks the cell as the continuation of the wide glyph to its left.
    pub fn set_continuation(&mut self) {
        self.symbol = ' ';
        self.continuation = true;
        self.foreground = None;
    }

    /// Sets the foreground colour.
    pub fn set_foreground(&mut self, colour: SVColour) {
        self.foreground = Some(colour);
    }

    /// Sets the background colour.
    pub fn set_background(&mut self, colour: SVColour) {
        self.background = Some(colour);
    }

    /// Removes both colours.
    pub fn clear_colours(&mut self) {
        self.foreground = None;
        self.background = None;
    }
}
