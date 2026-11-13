//! Marks on the cell grid: braille for paths and dots, blocks for areas.
//!
//! The mark kernel already produced the geometry; this module only rasterizes
//! it. Every variant the backend declares is approximated — a path and a circle
//! as braille dots, a rectangle as blocks, a text run as glyphs. The image
//! variant never reaches this layer: the capability matrix rejects it while the
//! figure is configured, because a borrowed bitmap cannot be shown honestly on
//! a cell grid.

use crate::cell::SVCellCanvas;
use scienceview_core::sv_decoration::SVTextAnchor;
use scienceview_core::sv_mark::SVGeometryRectangle;
use scienceview_core::sv_mark::SVMarkPaint;
use scienceview_core::sv_mark::SVMarkPrimitive;
use scienceview_core::sv_mark::SVMarkShape;
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

    pub(crate) fn draw_geometry_rectangle(
        &mut self,
        rectangle: &SVGeometryRectangle,
        paint: &SVMarkPaint,
    ) {
        if let Some(fill) = paint.fill() {
            self.draw_filled_rectangle(&rectangle.rectangle(), fill);
        }
        if let Some(stroke) = paint.stroke() {
            self.draw_outline(&rectangle.rectangle(), stroke);
        }
    }

    pub(crate) fn draw_geometry_circle(
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
}
