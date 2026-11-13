//! Text on the cell grid: labels, titles and legends.
//!
//! A glyph occupies the cell its anchor falls in; a wide glyph occupies two.
//! A label that would run past the right edge is clipped, and a wide glyph that
//! cannot fit whole is dropped, so a row never overflows its width.

use super::canvas::SVCellCanvas;
use super::cell_display_width::sv_cell_display_width;
use scienceview_core::sv_decoration::SVTextAnchor;
use scienceview_core::sv_scale::SVColour;

impl SVCellCanvas {
    /// Draws a text label anchored at a logical position.
    ///
    /// `position.y` is the baseline the decoration produced; the glyph row is
    /// the cell above it that the font size occupies.
    pub(crate) fn draw_anchored_text(
        &mut self,
        position: (f64, f64),
        text: &str,
        font_size: f64,
        colour: SVColour,
        anchor: SVTextAnchor,
    ) {
        let Some((column, row)) = self.text_origin(position, text, font_size, anchor) else {
            return;
        };
        self.draw_text_at(column, row, text, colour);
    }

    fn text_origin(
        &self,
        position: (f64, f64),
        text: &str,
        font_size: f64,
        anchor: SVTextAnchor,
    ) -> Option<(usize, usize)> {
        if self.width() == 0 || self.height() == 0 {
            return None;
        }

        let font_size = if font_size.is_finite() && font_size > 0.0 {
            font_size
        } else {
            self.viewport.cell_height() * 0.75
        };
        let row = ((position.1 - font_size * 0.8) / self.viewport.cell_height()).floor();
        let row = row.clamp(0.0, (self.height() - 1) as f64) as usize;

        let text_width = sv_text_display_width(text) as f64;
        let shift = match anchor {
            SVTextAnchor::Start => 0.0,
            SVTextAnchor::Middle => text_width / 2.0,
            SVTextAnchor::End => text_width,
        };
        let column = position.0 / self.viewport.cell_width() - shift;
        Some((column.round().max(0.0) as usize, row))
    }

    fn draw_text_at(&mut self, column: usize, row: usize, text: &str, colour: SVColour) {
        let mut current = column;
        for symbol in text.chars() {
            let width = sv_cell_display_width(symbol);
            if width == 0 {
                continue;
            }
            if current + width > self.width() {
                break;
            }
            if let Some(cell) = self.cell_mut(current, row) {
                cell.set_symbol(symbol);
                cell.set_foreground(colour);
            }
            if width == 2 {
                if let Some(cell) = self.cell_mut(current + 1, row) {
                    cell.set_continuation();
                }
            }
            current += width;
        }
    }
}

/// Returns the display width of a whole label.
pub(crate) fn sv_text_display_width(text: &str) -> usize {
    text.chars().map(sv_cell_display_width).sum()
}
