//! Stroking a flattened path: capsules, dashes, caps and joins.
//!
//! A stroke stamps one capsule per segment, which anti-aliases the line from
//! its distance to the pixel centre. Round caps and round joins add a disc; a
//! butt cap clips the capsule at its endpoint and a square cap extends it by
//! half the stroke width.

use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_apply_opacity;
use super::coverage::sv_coverage_from_distance;
use super::coverage::sv_device_pixel_range;
use super::coverage::sv_segment_distance;
use super::path_flattening::SVFlattenedPath;
use super::path_flattening::sv_dashed_spans;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_mark::SVLineCap;
use scienceview_core::sv_mark::SVLineJoin;
use scienceview_core::sv_scale::SVColour;

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
                if join == SVLineJoin::Round {
                    let interior = if closed {
                        span.len()
                    } else {
                        span.len().saturating_sub(1)
                    };
                    for vertex in span.iter().take(interior).skip(1) {
                        self.stamp_disc(*vertex, radius, colour);
                    }
                }
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
}
