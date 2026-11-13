//! Marks on the cell grid: braille for paths and dots, blocks for areas.
//!
//! The mark kernel already produced the geometry; this module only rasterizes
//! it. Every variant the backend declares is approximated — a path and a circle
//! as braille dots, a rectangle as blocks, a text run as glyphs. The image
//! variant never reaches this layer: the capability matrix rejects it while the
//! figure is configured, because a borrowed bitmap cannot be shown honestly on
//! a cell grid.

use super::braille::sv_stamp_braille_dot;
use super::canvas::SVCellCanvas;
use scienceview_core::sv_decoration::SVTextAnchor;
use scienceview_core::sv_mark::SVGeometryPath;
use scienceview_core::sv_mark::SVGeometryRectangle;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_mark::SVMarkShape;
use scienceview_core::sv_mark::SVPathCommand;
use scienceview_core::sv_mark::SVTextAlign;
use scienceview_core::sv_scale::SVColour;

impl SVCellCanvas {
    /// Draws one mark primitive.
    pub(crate) fn draw_mark_primitive(&mut self, primitive: &SVMarkPrimitive<'_>) {
        match primitive.shape() {
            SVMarkShape::Path(path) => self.draw_geometry_path(path, primitive.paint()),
            SVMarkShape::Rectangle(rectangle) => {
                self.draw_geometry_rectangle(rectangle, primitive.paint());
            }
            SVMarkShape::Circle(circle) => {
                self.draw_geometry_circle(circle.centre(), circle.radius(), primitive.paint());
            }
            SVMarkShape::Text(text) => {
                let anchor = match text.align() {
                    SVTextAlign::Left => SVTextAnchor::Start,
                    SVTextAlign::Center => SVTextAnchor::Middle,
                    SVTextAlign::Right => SVTextAnchor::End,
                };
                self.draw_anchored_text(
                    (text.position().x(), text.position().y()),
                    text.text(),
                    text.font_size(),
                    primitive
                        .paint()
                        .fill()
                        .or_else(|| primitive.paint().stroke())
                        .unwrap_or(SVColour::new(0, 0, 0, 255)),
                    anchor,
                );
            }
            SVMarkShape::Image(_) => {}
        }
    }

    fn draw_geometry_rectangle(&mut self, rectangle: &SVGeometryRectangle, paint: &SVMarkPaint) {
        if let Some(fill) = paint.fill() {
            self.draw_filled_rectangle(&rectangle.rectangle(), fill);
        }
        if let Some(stroke) = paint.stroke() {
            self.draw_outline(&rectangle.rectangle(), stroke);
        }
    }

    fn draw_geometry_path(&mut self, path: &SVGeometryPath, paint: &SVMarkPaint) {
        let colour = paint
            .stroke()
            .or_else(|| paint.fill())
            .unwrap_or(SVColour::new(0, 0, 0, 255));

        let mut current = None;
        let mut subpath_start = None;
        for command in path.commands() {
            match command {
                SVPathCommand::MoveTo(point) => {
                    current = Some((point.x(), point.y()));
                    subpath_start = current;
                }
                SVPathCommand::LineTo(point) => {
                    if let Some(start) = current {
                        self.draw_braille_segment(start, (point.x(), point.y()), colour);
                    }
                    current = Some((point.x(), point.y()));
                }
                SVPathCommand::ArcTo(arc) => {
                    let end = self.draw_braille_arc(arc, colour);
                    current = Some(end);
                }
                SVPathCommand::Close => {
                    if let (Some(start), Some(end)) = (subpath_start, current) {
                        self.draw_braille_segment(end, start, colour);
                    }
                    current = subpath_start;
                }
            }
        }

        if paint.fill().is_some() {
            self.draw_braille_fill(path, colour);
        }
    }

    fn draw_braille_arc(
        &mut self,
        arc: &scienceview_core::sv_mark::SVArcTo,
        colour: SVColour,
    ) -> (f64, f64) {
        let steps = 24;
        let mut previous = None;
        for step in 0..=steps {
            let fraction = step as f64 / steps as f64;
            let angle = arc.start_angle() + (arc.end_angle() - arc.start_angle()) * fraction;
            let point = (
                arc.centre().x() + arc.radius() * angle.cos(),
                arc.centre().y() + arc.radius() * angle.sin(),
            );
            if let Some(previous) = previous {
                self.draw_braille_segment(previous, point, colour);
            }
            previous = Some(point);
        }
        previous.unwrap_or((arc.centre().x(), arc.centre().y()))
    }

    fn draw_geometry_circle(
        &mut self,
        centre: scienceview_core::sv_mark::SVMarkPoint,
        radius: f64,
        paint: &SVMarkPaint,
    ) {
        let colour = paint
            .fill()
            .or_else(|| paint.stroke())
            .unwrap_or(SVColour::new(0, 0, 0, 255));
        if radius <= 0.0 {
            self.stamp_subpixel(centre.x(), centre.y(), colour);
            return;
        }

        if paint.fill().is_some() {
            let mut y = centre.y() - radius;
            while y <= centre.y() + radius {
                let mut x = centre.x() - radius;
                while x <= centre.x() + radius {
                    let dx = x - centre.x();
                    let dy = y - centre.y();
                    if dx * dx + dy * dy <= radius * radius {
                        self.stamp_subpixel(x, y, colour);
                    }
                    x += 0.5;
                }
                y += 0.5;
            }
        } else {
            self.draw_braille_circle(centre.x(), centre.y(), radius, colour);
        }
    }

    fn draw_braille_circle(&mut self, x: f64, y: f64, radius: f64, colour: SVColour) {
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

    fn draw_braille_fill(&mut self, path: &SVGeometryPath, colour: SVColour) {
        let points: Vec<(f64, f64)> = path
            .commands()
            .iter()
            .filter_map(|command| match command {
                SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => {
                    Some((point.x(), point.y()))
                }
                _ => None,
            })
            .collect();
        if points.len() < 3 {
            return;
        }
        let (minimum_y, maximum_y) = points.iter().fold((f64::MAX, f64::MIN), |acc, point| {
            (acc.0.min(point.1), acc.1.max(point.1))
        });
        let mut y = minimum_y;
        while y <= maximum_y {
            let crossings = sv_polygon_crossings(&points, y);
            for pair in crossings.chunks_exact(2) {
                let mut x = pair[0];
                while x <= pair[1] {
                    self.stamp_subpixel(x, y, colour);
                    x += 0.5;
                }
            }
            y += 0.5;
        }
    }

    fn draw_braille_segment(&mut self, start: (f64, f64), end: (f64, f64), colour: SVColour) {
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

    fn subpixel_of(&self, x: f64, y: f64) -> (usize, usize) {
        let column = (x / self.viewport.cell_width() * 2.0).floor();
        let row = (y / self.viewport.cell_height() * 4.0).floor();
        (column.max(0.0) as usize, row.max(0.0) as usize)
    }

    fn stamp_subpixel(&mut self, x: f64, y: f64, colour: SVColour) {
        let (column, row) = self.subpixel_of(x, y);
        self.stamp_subcell(column, row, colour);
    }

    fn stamp_subcell(&mut self, sub_column: usize, sub_row: usize, colour: SVColour) {
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

fn sv_polygon_crossings(points: &[(f64, f64)], y: f64) -> Vec<f64> {
    let mut crossings: Vec<f64> = Vec::new();
    for index in 0..points.len() {
        let first = points[index];
        let second = points[(index + 1) % points.len()];
        let (low, high) = if first.1 <= second.1 {
            (first, second)
        } else {
            (second, first)
        };
        if y >= low.1 && y < high.1 && (high.1 - low.1).abs() > f64::EPSILON {
            let fraction = (y - low.1) / (high.1 - low.1);
            crossings.push(low.0 + (high.0 - low.0) * fraction);
        }
    }
    crossings.sort_by(|left, right| left.total_cmp(right));
    crossings
}
