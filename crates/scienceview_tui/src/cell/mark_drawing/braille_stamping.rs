//! Sub-cell rasterization primitives: arcs, circles and single dots.

use crate::cell::SVCellCanvas;
use crate::cell::braille::sv_stamp_braille_dot;
use scienceview_core::sv_scale::SVColour;

impl SVCellCanvas {
    pub(crate) fn draw_braille_circle(&mut self, x: f64, y: f64, radius: f64, colour: SVColour) {
        let steps = 32;
        let mut previous = None;
        for step in 0..=steps {
            let angle = std::f64::consts::TAU * step as f64 / steps as f64;
            let point = (x + radius * angle.cos(), y + radius * angle.sin());
            if let Some(previous) = previous {
                self.draw_braille_segment(previous, point, colour);
            }
            previous = Some(point);
        }
    }
    pub(crate) fn draw_braille_segment(
        &mut self,
        start: (f64, f64),
        end: (f64, f64),
        colour: SVColour,
    ) {
        let (start_column, start_sub_row) = self.subpixel_of(start.0, start.1);
        let (end_column, end_sub_row) = self.subpixel_of(end.0, end.1);
        let steps = start_column
            .abs_diff(end_column)
            .max(start_sub_row.abs_diff(end_sub_row))
            .max(1);
        for step in 0..=steps {
            let fraction = step as f64 / steps as f64;
            let x = start_column as f64 + (end_column as f64 - start_column as f64) * fraction;
            let y = start_sub_row as f64 + (end_sub_row as f64 - start_sub_row as f64) * fraction;
            self.stamp_subcell(x.round() as usize, y.round() as usize, colour);
        }
    }
    pub(crate) fn subpixel_of(&self, x: f64, y: f64) -> (usize, usize) {
        let column = (x / self.viewport.cell_width() * 2.0).floor();
        let row = (y / self.viewport.cell_height() * 4.0).floor();
        (column.max(0.0) as usize, row.max(0.0) as usize)
    }

    pub(crate) fn stamp_subpixel(&mut self, x: f64, y: f64, colour: SVColour) {
        let (column, row) = self.subpixel_of(x, y);
        self.stamp_subcell(column, row, colour);
    }

    pub(crate) fn stamp_subcell(&mut self, sub_column: usize, sub_row: usize, colour: SVColour) {
        let column = sub_column / 2;
        let row = sub_row / 4;
        if column >= self.width() || row >= self.height() {
            return;
        }
        if let Some(cell) = self.cell_mut(column, row) {
            sv_stamp_braille_dot(cell, sub_column % 2, sub_row % 4, colour);
        }
    }
}
