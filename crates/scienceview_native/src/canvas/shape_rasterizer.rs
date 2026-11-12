//! Rectangles, circles and raster images on the raster canvas.
//!
//! A rectangle and a circle are drawn from their signed distance to the pixel
//! centre, which anti-aliases the boundary without sampling; an image samples
//! the caller's borrowed RGBA buffer with the interpolation rule the mark
//! chose. Nothing here re-derives a position: the shapes arrive already solved.

use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_apply_opacity;
use super::coverage::sv_coverage_from_distance;
use super::coverage::sv_device_pixel_range;
use scienceview_core::sv_mark::SVGeometryCircle;
use scienceview_core::sv_mark::SVGeometryRectangle;
use scienceview_core::sv_scale::SVColour;

impl SVRasterCanvas {
    /// Fills a rectangle, honouring its corner radius.
    pub(crate) fn fill_rectangle(
        &mut self,
        rectangle: &SVGeometryRectangle,
        colour: SVColour,
        opacity: f32,
    ) {
        let colour = sv_apply_opacity(colour, opacity);
        if colour.alpha() == 0 {
            return;
        }
        let scale = self.device_scale();
        let solved = rectangle.rectangle();
        let centre_x = (solved.x + solved.width / 2.0) * scale;
        let centre_y = (solved.y + solved.height / 2.0) * scale;
        let half_width = solved.width * scale / 2.0;
        let half_height = solved.height * scale / 2.0;
        let corner = (rectangle.corner_radius() * scale)
            .min(half_width)
            .min(half_height);

        let minimum_x = centre_x - half_width - 1.0;
        let minimum_y = centre_y - half_height - 1.0;
        let maximum_x = centre_x + half_width + 1.0;
        let maximum_y = centre_y + half_height + 1.0;
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let offset_x = (x as f64 + 0.5 - centre_x).abs() - (half_width - corner);
                let offset_y = (y as f64 + 0.5 - centre_y).abs() - (half_height - corner);
                let outside_x = offset_x.max(0.0);
                let outside_y = offset_y.max(0.0);
                let distance = (outside_x * outside_x + outside_y * outside_y).sqrt()
                    + offset_x.max(offset_y).min(0.0)
                    - corner;
                let coverage = sv_coverage_from_distance(distance);
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }

        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }

    /// Fills a circle.
    pub(crate) fn fill_circle(
        &mut self,
        circle: &SVGeometryCircle,
        colour: SVColour,
        opacity: f32,
    ) {
        let colour = sv_apply_opacity(colour, opacity);
        if colour.alpha() == 0 {
            return;
        }
        let scale = self.device_scale();
        let centre_x = circle.centre().x() * scale;
        let centre_y = circle.centre().y() * scale;
        let radius = circle.radius() * scale;

        let minimum_x = centre_x - radius - 1.0;
        let minimum_y = centre_y - radius - 1.0;
        let maximum_x = centre_x + radius + 1.0;
        let maximum_y = centre_y + radius + 1.0;
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let offset_x = x as f64 + 0.5 - centre_x;
                let offset_y = y as f64 + 0.5 - centre_y;
                let distance = (offset_x * offset_x + offset_y * offset_y).sqrt();
                let coverage = sv_coverage_from_distance(distance - radius);
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }

        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }

    /// Strokes a circle as a ring.
    pub(crate) fn stroke_circle(
        &mut self,
        circle: &SVGeometryCircle,
        stroke_width: f64,
        colour: SVColour,
        opacity: f32,
    ) {
        if !stroke_width.is_finite() || stroke_width <= 0.0 {
            return;
        }
        let colour = sv_apply_opacity(colour, opacity);
        if colour.alpha() == 0 {
            return;
        }
        let scale = self.device_scale();
        let centre_x = circle.centre().x() * scale;
        let centre_y = circle.centre().y() * scale;
        let radius = circle.radius() * scale;
        let half_width = (stroke_width * scale / 2.0).max(0.5);

        let minimum_x = centre_x - radius - half_width - 1.0;
        let minimum_y = centre_y - radius - half_width - 1.0;
        let maximum_x = centre_x + radius + half_width + 1.0;
        let maximum_y = centre_y + radius + half_width + 1.0;
        let (start_x, end_x) = sv_device_pixel_range(minimum_x, maximum_x, self.width());
        let (start_y, end_y) = sv_device_pixel_range(minimum_y, maximum_y, self.height());

        for y in start_y..end_y {
            for x in start_x..end_x {
                let offset_x = x as f64 + 0.5 - centre_x;
                let offset_y = y as f64 + 0.5 - centre_y;
                let distance = (offset_x * offset_x + offset_y * offset_y).sqrt();
                let coverage = sv_coverage_from_distance((distance - radius).abs() - half_width);
                if coverage > 0.0 {
                    self.blend_pixel(x, y, colour, coverage);
                }
            }
        }

        self.mark_dirty_device(minimum_x, minimum_y, maximum_x, maximum_y);
    }
}
