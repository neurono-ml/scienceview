//! Text drawn as boxes until a glyph rasterizer exists.
//!
//! The decoration layer already lays text out as a measured box, so the native
//! renderer draws each character as one filled box inside that run. It is a
//! legible placeholder, not a font: the run keeps its alignment, baseline,
//! rotation and weight, and a later glyph rasterizer replaces the box without
//! changing any caller.

use super::canvas_definition::SVRasterCanvas;
use super::colour::sv_apply_opacity;
use super::path_flattening::sv_flattened_polygon;
use scienceview_core::sv_mark::SVTextAlign;
use scienceview_core::sv_mark::SVTextBaseline;
use scienceview_core::sv_scale::SVColour;

/// Where a text run starts relative to its anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SVTextHorizontalAnchor {
    /// The anchor is the left edge.
    Start,
    /// The anchor is the horizontal centre.
    Middle,
    /// The anchor is the right edge.
    End,
}

impl From<SVTextAlign> for SVTextHorizontalAnchor {
    fn from(align: SVTextAlign) -> Self {
        match align {
            SVTextAlign::Left => Self::Start,
            SVTextAlign::Center => Self::Middle,
            SVTextAlign::Right => Self::End,
        }
    }
}

impl From<scienceview_core::sv_decoration::SVTextAnchor> for SVTextHorizontalAnchor {
    fn from(anchor: scienceview_core::sv_decoration::SVTextAnchor) -> Self {
        use scienceview_core::sv_decoration::SVTextAnchor;
        match anchor {
            SVTextAnchor::Start => Self::Start,
            SVTextAnchor::Middle => Self::Middle,
            SVTextAnchor::End => Self::End,
        }
    }
}

impl SVRasterCanvas {
    /// Draws one run of text as boxes.
    ///
    /// `rotation_degrees` is clockwise around the anchor. Bold runs are drawn
    /// slightly wider, so a title and an axis label stay distinguishable.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_text_run(
        &mut self,
        position_x: f64,
        position_y: f64,
        text: &str,
        font_size: f64,
        weight: u16,
        colour: SVColour,
        anchor: SVTextHorizontalAnchor,
        baseline: SVTextBaseline,
        rotation_degrees: f64,
        opacity: f32,
    ) {
        if text.is_empty() || !font_size.is_finite() || font_size <= 0.0 {
            return;
        }
        let colour = sv_apply_opacity(colour, opacity);
        if colour.alpha() == 0 {
            return;
        }

        let bold_scale = if weight >= 600 { 1.1 } else { 1.0 };
        let advance = font_size * 0.6 * bold_scale;
        let glyph_width = advance * 0.82;
        let glyph_height = font_size * 0.72;
        let character_count = text.chars().count() as f64;
        let total_width = advance * character_count;

        let origin_x = match anchor {
            SVTextHorizontalAnchor::Start => position_x,
            SVTextHorizontalAnchor::Middle => position_x - total_width / 2.0,
            SVTextHorizontalAnchor::End => position_x - total_width,
        };
        let top_y = match baseline {
            SVTextBaseline::Top => position_y,
            SVTextBaseline::Middle => position_y - glyph_height / 2.0,
            SVTextBaseline::Bottom => position_y - glyph_height,
        };

        let angle = rotation_degrees.to_radians();
        let cosine = angle.cos();
        let sine = angle.sin();
        let scale = self.device_scale();

        for (index, character) in text.chars().enumerate() {
            if character.is_whitespace() {
                continue;
            }
            let left = origin_x + index as f64 * advance + (advance - glyph_width) / 2.0;
            let right = left + glyph_width;
            let bottom = top_y + glyph_height;

            let corners = [
                (left, top_y),
                (right, top_y),
                (right, bottom),
                (left, bottom),
            ]
            .map(|(corner_x, corner_y)| {
                let offset_x = corner_x - position_x;
                let offset_y = corner_y - position_y;
                (
                    (position_x + offset_x * cosine - offset_y * sine) * scale,
                    (position_y + offset_x * sine + offset_y * cosine) * scale,
                )
            });

            let polygon = sv_flattened_polygon(&corners);
            self.fill_flattened_path(&polygon, colour, 1.0);
        }
    }
}
