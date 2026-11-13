//! Paths and filled polygons rasterized into braille sub-cell dots.

use crate::cell::SVCellCanvas;
use scienceview_core::sv_mark::SVGeometryPath;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVPathCommand;
use scienceview_core::sv_scale::SVColour;

impl SVCellCanvas {
    pub(crate) fn draw_geometry_path(&mut self, path: &SVGeometryPath, paint: &SVMarkPaint) {
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

    pub(crate) fn draw_braille_arc(
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
    pub(crate) fn draw_braille_fill(&mut self, path: &SVGeometryPath, colour: SVColour) {
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
