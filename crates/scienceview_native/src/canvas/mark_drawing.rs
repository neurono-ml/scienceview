//! Drawing the backend-agnostic mark primitives on the raster canvas.
//!
//! A mark primitive is already a shape paired with its paint; the rasterizer
//! only flattens paths, applies the fill and the stroke, and lets the paint's
//! opacity reach every blend. It never re-derives a position from data.

use super::canvas_definition::SVRasterCanvas;
use super::path_flattening::sv_flatten_path;
use super::path_flattening::sv_flattened_polygon;
use super::text_rasterizer::SVTextHorizontalAnchor;
use scienceview_core::sv_mark::SVGeometryCircle;
use scienceview_core::sv_mark::SVGeometryRectangle;
use scienceview_core::sv_mark::SVMarkGeometry;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_mark::SVMarkShape;
use scienceview_core::sv_scale::SVColour;

impl SVRasterCanvas {
    /// Draws every primitive of a mark geometry in draw order.
    pub fn draw_mark_geometry(&mut self, geometry: &SVMarkGeometry<'_>) {
        for primitive in geometry.primitives() {
            self.draw_mark_primitive(primitive);
        }
    }

    /// Draws one primitive with its paint.
    pub fn draw_mark_primitive(&mut self, primitive: &SVMarkPrimitive<'_>) {
        let paint = primitive.paint();
        let opacity = paint.opacity();

        match primitive.shape() {
            SVMarkShape::Path(path) => {
                let flattened = sv_flatten_path(path.commands(), self.device_scale());
                sv_paint_flattened(self, &flattened, paint);
            }
            SVMarkShape::Rectangle(rectangle) => {
                sv_paint_rectangle(self, rectangle, paint);
            }
            SVMarkShape::Circle(circle) => {
                sv_paint_circle(self, circle, paint);
            }
            SVMarkShape::Text(text) => {
                let colour = paint
                    .fill()
                    .or_else(|| paint.stroke())
                    .unwrap_or(SVColour::new(0, 0, 0, 255));
                self.draw_text_run(
                    text.position().x(),
                    text.position().y(),
                    text.text(),
                    text.font_size(),
                    400,
                    colour,
                    SVTextHorizontalAnchor::from(text.align()),
                    text.baseline(),
                    0.0,
                    opacity,
                );
            }
            SVMarkShape::Image(image) => {
                self.draw_image(image, opacity);
            }
        }
    }
}

fn sv_paint_flattened(
    canvas: &mut SVRasterCanvas,
    flattened: &super::path_flattening::SVFlattenedPath,
    paint: &SVMarkPaint,
) {
    if let Some(fill) = paint.fill() {
        canvas.fill_flattened_path(flattened, fill, paint.opacity());
    }
    if let Some(stroke) = paint.stroke() {
        canvas.stroke_flattened_path(
            flattened,
            paint.stroke_width(),
            stroke,
            paint.opacity(),
            paint.dash(),
            paint.cap(),
            paint.join(),
        );
    }
}

fn sv_paint_rectangle(
    canvas: &mut SVRasterCanvas,
    rectangle: &SVGeometryRectangle,
    paint: &SVMarkPaint,
) {
    if let Some(fill) = paint.fill() {
        canvas.fill_rectangle(rectangle, fill, paint.opacity());
    }
    let Some(stroke) = paint.stroke() else {
        return;
    };
    let solved = rectangle.rectangle();
    let corners = [
        (solved.x, solved.y),
        (solved.right(), solved.y),
        (solved.right(), solved.bottom()),
        (solved.x, solved.bottom()),
    ]
    .map(|(x, y)| (x * canvas.device_scale(), y * canvas.device_scale()));
    let flattened = sv_flattened_polygon(&corners);
    canvas.stroke_flattened_path(
        &flattened,
        paint.stroke_width(),
        stroke,
        paint.opacity(),
        paint.dash(),
        paint.cap(),
        paint.join(),
    );
}

fn sv_paint_circle(canvas: &mut SVRasterCanvas, circle: &SVGeometryCircle, paint: &SVMarkPaint) {
    if let Some(fill) = paint.fill() {
        canvas.fill_circle(circle, fill, paint.opacity());
    }
    if let Some(stroke) = paint.stroke() {
        canvas.stroke_circle(circle, paint.stroke_width(), stroke, paint.opacity());
    }
}
