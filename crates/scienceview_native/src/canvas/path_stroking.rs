//! Stroking a flattened path: capsules, dashes, caps and joins.
//!
//! A stroke stamps one capsule per segment, which anti-aliases the line from
//! its distance to the pixel centre. A butt cap clips the capsule at its
//! endpoint and a square cap extends it by half the stroke width. Joins fill
//! the notch the capsules leave on the outer side of a corner: a round join
//! stamps a disc, a bevel fills the triangle between the two offset points, and
//! a miter extends them to their intersection — falling back to a bevel when
//! that point lies beyond the miter limit.

use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_apply_opacity;
use super::coverage::sv_coverage_from_distance;
use super::coverage::sv_device_pixel_range;
use super::coverage::sv_sampled_coverage;
use super::coverage::sv_segment_distance;
use super::path_flattening::SVFlattenedPath;
use super::path_flattening::sv_dashed_spans;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_mark::SVLineCap;
use scienceview_core::sv_mark::SVLineJoin;
use scienceview_core::sv_scale::SVColour;

/// The ratio of miter length to stroke radius above which a miter falls back to
/// a bevel, matching the default miter limit of a vector format.
const NATIVE_MITER_LIMIT: f64 = 4.0;

impl SVRasterCanvas {
    /// Strokes a flattened path with one colour.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stroke_flattened_path(
        &mut self,
        path: &SVFlattenedPath,
        stroke_width: f64,
        colour: SVColour,
        opacity: f32,
        dash: &SVDashPattern,
        cap: SVLineCap,
        join: SVLineJoin,
    ) {
        if !stroke_width.is_finite() || stroke_width <= 0.0 {
            return;
        }
        let colour = sv_apply_opacity(colour, opacity);
        if colour.alpha() == 0 {
            return;
        }

        let radius = (stroke_width * self.device_scale() / 2.0).max(0.5);
        for (position, subpath) in path.subpaths().iter().enumerate() {
            let closed = path.is_closed(position);
            let spans = sv_dashed_spans(subpath, closed, dash.segments());
            for span in spans {
                self.stroke_span(&span, radius, colour, cap);
                // A dashed span is an open run even when the contour is closed,
                // so only an undivided closed contour wraps its last join.
                let is_loop = closed && span.len() == subpath.len();
                self.stroke_joins(&span, is_loop, radius, colour, join);
            }
        }
    }

    /// Stamps one dash span, one segment at a time.
    fn stroke_span(&mut self, span: &[(f64, f64)], radius: f64, colour: SVColour, cap: SVLineCap) {
        if span.len() == 1 {
            if cap == SVLineCap::Round {
                self.stamp_disc(span[0], radius, colour);
            }
            return;
        }

        for pair in span.windows(2) {
            self.stamp_segment(pair[0], pair[1], radius, colour, cap);
        }
    }

    /// Stamps the join of every interior vertex of a span.
    fn stroke_joins(
        &mut self,
        span: &[(f64, f64)],
        is_loop: bool,
        radius: f64,
        colour: SVColour,
        join: SVLineJoin,
    ) {
        if span.len() < 2 {
            return;
        }
        let start = if is_loop { 0 } else { 1 };
        let end = if is_loop { span.len() } else { span.len() - 1 };
        let length = span.len();
        for index in start..end {
            let vertex = span[index];
            let previous = span[(index + length - 1) % length];
            let next = span[(index + 1) % length];
            match join {
                SVLineJoin::Round => self.stamp_disc(vertex, radius, colour),
                SVLineJoin::Bevel => {
                    self.stamp_join(previous, vertex, next, radius, colour, false);
                }
                SVLineJoin::Miter => {
                    self.stamp_join(previous, vertex, next, radius, colour, true);
                }
            }
        }
    }

    /// Stamps one capsule along a segment.
    fn stamp_segment(
        &mut self,
        start: (f64, f64),
        end: (f64, f64),
        radius: f64,
        colour: SVColour,
        cap: SVLineCap,
    ) {
        let extend = if cap == SVLineCap::Square {
            radius
        } else {
            0.0
        };
        let minimum_x = start.0.min(end.0) - radius - 1.0;
        let minimum_y = start.1.min(end.1) - radius - 1.0;
        let maximum_x = start.0.max(end.0) + radius + 1.0;
        let maximum_y = start.1.max(end.1) + radius + 1.0;
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let (distance, projection) = sv_segment_distance(
                    x as f64 + 0.5,
                    y as f64 + 0.5,
                    start.0,
                    start.1,
                    end.0,
                    end.1,
                    extend,
                );
                if cap == SVLineCap::Butt && !(0.0..=1.0).contains(&projection) {
                    continue;
                }
                let coverage = sv_coverage_from_distance(distance - radius);
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }

        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }

    /// Stamps a filled disc, used by round caps and round joins.
    fn stamp_disc(&mut self, centre: (f64, f64), radius: f64, colour: SVColour) {
        let minimum_x = centre.0 - radius - 1.0;
        let minimum_y = centre.1 - radius - 1.0;
        let maximum_x = centre.0 + radius + 1.0;
        let maximum_y = centre.1 + radius + 1.0;
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let offset_x = x as f64 + 0.5 - centre.0;
                let offset_y = y as f64 + 0.5 - centre.1;
                let distance = (offset_x * offset_x + offset_y * offset_y).sqrt();
                let coverage = sv_coverage_from_distance(distance - radius);
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }

        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }

    /// Fills the notch between two segments at a corner.
    ///
    /// A bevel joins the two outer offset points to the corner, cutting the
    /// outer side flat. A miter joins them through their intersection, so the
    /// corner keeps its point; when that tip is farther from the corner than
    /// the miter limit the shape falls back to the bevel.
    fn stamp_join(
        &mut self,
        previous: (f64, f64),
        vertex: (f64, f64),
        next: (f64, f64),
        radius: f64,
        colour: SVColour,
        miter: bool,
    ) {
        let Some(incoming) = sv_normalized(vertex.0 - previous.0, vertex.1 - previous.1) else {
            return;
        };
        let Some(outgoing) = sv_normalized(next.0 - vertex.0, next.1 - vertex.1) else {
            return;
        };
        let turn = incoming.0 * outgoing.1 - incoming.1 * outgoing.0;
        if turn.abs() <= f64::EPSILON {
            // The segments are collinear; the capsules already meet.
            return;
        }

        // The outer side is the side of the turn's outside: the right side for
        // a left turn and the left side otherwise.
        let side = if turn > 0.0 { 1.0 } else { -1.0 };
        let first_offset = (
            vertex.0 + radius * side * incoming.1,
            vertex.1 - radius * side * incoming.0,
        );
        let second_offset = (
            vertex.0 + radius * side * outgoing.1,
            vertex.1 - radius * side * outgoing.0,
        );

        if miter {
            if let Some(tip) = sv_miter_tip(
                first_offset,
                incoming,
                second_offset,
                outgoing,
                vertex,
                radius,
            ) {
                self.fill_join_polygon(&[vertex, first_offset, tip, second_offset], colour);
                return;
            }
        }

        self.fill_join_polygon(&[vertex, first_offset, second_offset], colour);
    }

    /// Samples a small convex polygon and blends the covered pixels.
    fn fill_join_polygon(&mut self, polygon: &[(f64, f64)], colour: SVColour) {
        let mut minimum_x = f64::INFINITY;
        let mut minimum_y = f64::INFINITY;
        let mut maximum_x = f64::NEG_INFINITY;
        let mut maximum_y = f64::NEG_INFINITY;
        for point in polygon {
            minimum_x = minimum_x.min(point.0);
            minimum_y = minimum_y.min(point.1);
            maximum_x = maximum_x.max(point.0);
            maximum_y = maximum_y.max(point.1);
        }
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let coverage = sv_sampled_coverage(x as f64, y as f64, |point_x, point_y| {
                    sv_point_in_polygon(polygon, point_x, point_y)
                });
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }

        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }
}

/// Normalizes a direction, or returns `None` when it has no length.
fn sv_normalized(x: f64, y: f64) -> Option<(f64, f64)> {
    let length = (x * x + y * y).sqrt();
    if !length.is_finite() || length <= f64::EPSILON {
        return None;
    }
    Some((x / length, y / length))
}

/// Returns the miter tip where the two offset edges meet, unless it lies beyond
/// the miter limit.
fn sv_miter_tip(
    first_offset: (f64, f64),
    incoming: (f64, f64),
    second_offset: (f64, f64),
    outgoing: (f64, f64),
    vertex: (f64, f64),
    radius: f64,
) -> Option<(f64, f64)> {
    let turn = incoming.0 * outgoing.1 - incoming.1 * outgoing.0;
    if turn.abs() <= f64::EPSILON {
        return None;
    }
    let between = (
        second_offset.0 - first_offset.0,
        second_offset.1 - first_offset.1,
    );
    let distance = (between.0 * outgoing.1 - between.1 * outgoing.0) / turn;
    let tip = (
        first_offset.0 + distance * incoming.0,
        first_offset.1 + distance * incoming.1,
    );
    let length = ((tip.0 - vertex.0).powi(2) + (tip.1 - vertex.1).powi(2)).sqrt();
    if length / radius > NATIVE_MITER_LIMIT {
        return None;
    }
    Some(tip)
}

/// Returns `true` when a point lies inside a convex polygon, by the even-odd
/// crossing rule.
fn sv_point_in_polygon(polygon: &[(f64, f64)], point_x: f64, point_y: f64) -> bool {
    let mut inside = false;
    let mut previous = polygon[polygon.len() - 1];
    for &current in polygon {
        if (current.1 > point_y) != (previous.1 > point_y) {
            let fraction = (point_y - current.1) / (previous.1 - current.1);
            let crossing_x = current.0 + fraction * (previous.0 - current.0);
            if point_x < crossing_x {
                inside = !inside;
            }
        }
        previous = current;
    }
    inside
}
