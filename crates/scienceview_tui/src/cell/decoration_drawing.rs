//! Decorations on the cell grid: lines, rectangles and text labels.

use super::blocks::sv_corner_glyph;
use super::blocks::sv_line_glyph;
use super::blocks::sv_partial_block;
use super::canvas::SVCellCanvas;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_decoration::SVRectanglePrimitive;
use scienceview_core::sv_scale::SVColour;

impl SVCellCanvas {
    /// Draws one decoration primitive.
    pub fn draw_decoration_primitive(&mut self, primitive: &SVDecorationPrimitive) {
        match primitive {
            SVDecorationPrimitive::Line { start, end, stroke } => {
                self.draw_segment((start.x, start.y), (end.x, end.y), stroke.colour);
            }
            SVDecorationPrimitive::Rectangle(rectangle) => {
                self.draw_rectangle_primitive(rectangle);
            }
            SVDecorationPrimitive::Text {
                position,
                text,
                font_size,
                colour,
                anchor,
                ..
            } => {
                self.draw_anchored_text(
                    (position.x, position.y),
                    text,
                    *font_size,
                    *colour,
                    *anchor,
                );
            }
        }
    }

    fn draw_rectangle_primitive(&mut self, rectangle: &SVRectanglePrimitive) {
        if let Some(fill) = rectangle.fill {
            self.draw_filled_rectangle(&rectangle.rectangle, fill);
        }
        if let Some(stroke) = rectangle.stroke {
            self.draw_outline(&rectangle.rectangle, stroke.colour);
        }
    }

    /// Draws a straight line cell by cell.
    pub(crate) fn draw_segment(&mut self, start: (f64, f64), end: (f64, f64), colour: SVColour) {
        let glyph = sv_line_glyph(start, end);
        let Some((start_column, start_row)) = self.cell_of(start.0, start.1) else {
            return;
        };
        let Some((end_column, end_row)) = self.cell_of(end.0, end.1) else {
            return;
        };

        let steps = start_column
            .abs_diff(end_column)
            .max(start_row.abs_diff(end_row))
            .max(1);
        for step in 0..=steps {
            let fraction = step as f64 / steps as f64;
            let column = start_column as f64 + (end_column as f64 - start_column as f64) * fraction;
            let row = start_row as f64 + (end_row as f64 - start_row as f64) * fraction;
            if let Some(cell) = self.cell_mut(column.round() as usize, row.round() as usize) {
                cell.set_symbol(glyph);
                cell.set_foreground(colour);
            }
        }
    }

    /// Draws the four sides of a rectangle, with box-drawing corners.
    pub(crate) fn draw_outline(
        &mut self,
        rectangle: &scienceview_core::sv_layout::SVResolvedRectangle,
        colour: SVColour,
    ) {
        let Some((left, top)) = self.cell_of(rectangle.x, rectangle.y) else {
            return;
        };
        let Some((right, bottom)) = self.cell_of(
            rectangle.right() - f64::EPSILON,
            rectangle.bottom() - f64::EPSILON,
        ) else {
            return;
        };

        if left == right || top == bottom {
            self.draw_segment(
                (rectangle.x, rectangle.y),
                (rectangle.right(), rectangle.bottom()),
                colour,
            );
            return;
        }

        for column in left..=right {
            if let Some(cell) = self.cell_mut(column, top) {
                cell.set_symbol('─');
                cell.set_foreground(colour);
            }
            if let Some(cell) = self.cell_mut(column, bottom) {
                cell.set_symbol('─');
                cell.set_foreground(colour);
            }
        }
        for row in top..=bottom {
            if let Some(cell) = self.cell_mut(left, row) {
                cell.set_symbol('│');
                cell.set_foreground(colour);
            }
            if let Some(cell) = self.cell_mut(right, row) {
                cell.set_symbol('│');
                cell.set_foreground(colour);
            }
        }

        for (column, row, top_side, left_side) in [
            (left, top, true, true),
            (right, top, true, false),
            (left, bottom, false, true),
            (right, bottom, false, false),
        ] {
            if let Some(cell) = self.cell_mut(column, row) {
                cell.set_symbol(sv_corner_glyph(top_side, left_side));
                cell.set_foreground(colour);
            }
        }
    }

    /// Fills a rectangle with block glyphs, using a partial block when the
    /// rectangle ends inside a cell row.
    pub(crate) fn draw_filled_rectangle(
        &mut self,
        rectangle: &scienceview_core::sv_layout::SVResolvedRectangle,
        colour: SVColour,
    ) {
        if rectangle.width <= 0.0 || rectangle.height <= 0.0 {
            return;
        }

        let cell_width = self.viewport.cell_width();
        let cell_height = self.viewport.cell_height();
        let left = (rectangle.x / cell_width).floor().max(0.0);
        let right = (rectangle.right() / cell_width).ceil();
        let top = (rectangle.y / cell_height).floor().max(0.0);
        let bottom = (rectangle.bottom() / cell_height).ceil();
        let top_exact = (rectangle.y / cell_height).max(0.0);
        let bottom_exact = rectangle.bottom() / cell_height;

        let mut column = left as usize;
        while column < right as usize && column < self.width() {
            let mut row = top as usize;
            while row < bottom as usize && row < self.height() {
                let overlap_start = top_exact.max(row as f64);
                let overlap_end = bottom_exact.min(row as f64 + 1.0);
                let coverage = (overlap_end - overlap_start).clamp(0.0, 1.0);
                let glyph = if coverage >= 1.0 {
                    '█'
                } else {
                    sv_partial_block(coverage)
                };
                if glyph != ' ' {
                    if let Some(cell) = self.cell_mut(column, row) {
                        cell.set_symbol(glyph);
                        cell.set_foreground(colour);
                    }
                }
                row += 1;
            }
            column += 1;
        }
    }
}
