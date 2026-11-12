//! Drawing the backend-agnostic decoration primitives on the raster canvas.
//!
//! Decorations are lines, rectangles and text boxes with resolved geometry, so
//! the rasterizer draws them with the same low-level shapes the marks use.

use super::canvas_definition::SVRasterCanvas;
use super::path_flattening::sv_flatten_path;
use super::path_flattening::sv_flattened_polygon;
use super::text_rasterizer::SVTextHorizontalAnchor;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_mark::SVDashPattern;
use scienceview_core::sv_mark::SVGeometryRectangle;
use scienceview_core::sv_mark::SVLineCap;
use scienceview_core::sv_mark::SVLineJoin;
use scienceview_core::sv_mark::SVMarkPoint;
use scienceview_core::sv_mark::SVPathCommand;
use scienceview_core::sv_mark::SVTextBaseline;

impl SVRasterCanvas {
    /// Draws every decoration primitive in draw order.
    pub fn draw_decoration_primitives(&mut self, primitives: &[SVDecorationPrimitive]) {
        for primitive in primitives {
            self.draw_decoration_primitive(primitive);
        }
    }

    /// Draws one decoration primitive.
    pub fn draw_decoration_primitive(&mut self, primitive: &SVDecorationPrimitive) {
        match primitive {
            SVDecorationPrimitive::Line { start, end, stroke } => {
                let commands = [
                    SVPathCommand::MoveTo(SVMarkPoint::new(start.x, start.y)),
                    SVPathCommand::LineTo(SVMarkPoint::new(end.x, end.y)),
                ];
                let flattened = sv_flatten_path(&commands, self.device_scale());
                self.stroke_flattened_path(
                    &flattened,
                    stroke.width,
                    stroke.colour,
                    1.0,
                    &SVDashPattern::Solid,
                    SVLineCap::Butt,
                    SVLineJoin::Round,
                );
            }
            SVDecorationPrimitive::Rectangle(rectangle) => {
                let shape = SVGeometryRectangle::new(rectangle.rectangle, 0.0);
                if let Some(fill) = rectangle.fill {
                    self.fill_rectangle(&shape, fill, 1.0);
                }
                if let Some(stroke) = rectangle.stroke {
                    let solved = rectangle.rectangle;
                    let corners = [
                        (solved.x, solved.y),
                        (solved.right(), solved.y),
                        (solved.right(), solved.bottom()),
                        (solved.x, solved.bottom()),
                    ]
                    .map(|(x, y)| (x * self.device_scale(), y * self.device_scale()));
                    let flattened = sv_flattened_polygon(&corners);
                    self.stroke_flattened_path(
                        &flattened,
                        stroke.width,
                        stroke.colour,
                        1.0,
                        &SVDashPattern::Solid,
                        SVLineCap::Butt,
                        SVLineJoin::Round,
                    );
                }
            }
            SVDecorationPrimitive::Text {
                position,
                text,
                font_size,
                weight,
                colour,
                anchor,
                rotation_degrees,
            } => {
                self.draw_text_run(
                    position.x,
                    position.y,
                    text,
                    *font_size,
                    *weight,
                    *colour,
                    SVTextHorizontalAnchor::from(*anchor),
                    SVTextBaseline::Middle,
                    *rotation_degrees,
                    1.0,
                );
            }
        }
    }
}
