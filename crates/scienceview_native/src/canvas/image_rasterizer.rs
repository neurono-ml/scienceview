//! RGBA images on the raster canvas.
//!
//! The destination rectangle comes from the solved geometry and the source is a
//! borrowed buffer, so a large heatmap is never copied into the geometry. The
//! interpolation rule the mark chose decides nearest-neighbour or bilinear
//! sampling.

use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_apply_opacity;
use super::colour::sv_blend_pixel;
use super::coverage::sv_device_pixel_range;
use scienceview_core::sv_mark::SVGeometryImage;
use scienceview_core::sv_mark::SVImageInterpolation;
use scienceview_core::sv_scale::SVColour;

impl SVRasterCanvas {
    /// Draws a borrowed RGBA image over its solved rectangle.
    pub(crate) fn draw_image(&mut self, image: &SVGeometryImage<'_>, opacity: f32) {
        let rectangle = image.rectangle();
        let source_width = image.width();
        let source_height = image.height();
        let pixels = image.pixels();
        if source_width == 0
            || source_height == 0
            || pixels.len() < source_width * source_height * 4
            || rectangle.width <= 0.0
            || rectangle.height <= 0.0
        {
            return;
        }

        let scale = self.device_scale();
        let (start_x, end_x) =
            sv_device_pixel_range(rectangle.x * scale, rectangle.right() * scale, self.width());
        let (start_y, end_y) = sv_device_pixel_range(
            rectangle.y * scale,
            rectangle.bottom() * scale,
            self.height(),
        );

        for y in start_y..end_y {
            for x in start_x..end_x {
                let fraction_x = (x as f64 + 0.5 - rectangle.x * scale) / (rectangle.width * scale);
                let fraction_y =
                    (y as f64 + 0.5 - rectangle.y * scale) / (rectangle.height * scale);
                let sampled = sv_sample_image(
                    pixels,
                    source_width,
                    source_height,
                    fraction_x,
                    fraction_y,
                    image.interpolation(),
                );
                let source = sv_apply_opacity(sampled, opacity);
                let blended = sv_blend_pixel(
                    self.pixel(x, y).unwrap_or(SVColour::new(0, 0, 0, 0)),
                    source,
                    1.0,
                );
                self.write_pixel(x, y, blended);
            }
        }

        self.mark_dirty_device(
            rectangle.x * scale,
            rectangle.y * scale,
            rectangle.right() * scale,
            rectangle.bottom() * scale,
        );
    }
}

fn sv_sample_image(
    pixels: &[u8],
    width: usize,
    height: usize,
    fraction_x: f64,
    fraction_y: f64,
    interpolation: SVImageInterpolation,
) -> SVColour {
    let clamp_index = |value: f64, limit: usize| -> usize {
        if !value.is_finite() || value <= 0.0 {
            0
        } else {
            (value as usize).min(limit - 1)
        }
    };
    let read = |x: usize, y: usize| -> SVColour {
        let offset = (y * width + x) * 4;
        SVColour::new(
            pixels[offset],
            pixels[offset + 1],
            pixels[offset + 2],
            pixels[offset + 3],
        )
    };

    let position_x = fraction_x.clamp(0.0, 1.0) * (width as f64 - 1.0);
    let position_y = fraction_y.clamp(0.0, 1.0) * (height as f64 - 1.0);

    match interpolation {
        SVImageInterpolation::Nearest => read(
            clamp_index(position_x.round(), width),
            clamp_index(position_y.round(), height),
        ),
        SVImageInterpolation::Bilinear => {
            let left = position_x.floor();
            let top = position_y.floor();
            let weight_x = position_x - left;
            let weight_y = position_y - top;
            let x0 = clamp_index(left, width);
            let y0 = clamp_index(top, height);
            let x1 = clamp_index(left + 1.0, width);
            let y1 = clamp_index(top + 1.0, height);
            let top_colour = read(x0, y0).interpolate(read(x1, y0), weight_x);
            let bottom_colour = read(x0, y1).interpolate(read(x1, y1), weight_x);
            top_colour.interpolate(bottom_colour, weight_y)
        }
    }
}
