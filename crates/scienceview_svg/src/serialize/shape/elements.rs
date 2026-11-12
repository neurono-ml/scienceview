//! Mark shapes become SVG elements.
//!
//! One primitive is one element. Geometry already arrived in display space, so
//! this module never asks a scale or a layout engine for a number.

use super::path::sv_serialize_path_data;
use crate::image::sv_encode_png_data_uri;
use crate::serialize::sv_escape_text;
use crate::serialize::sv_format_number;
use crate::serialize::sv_mark_paint_attributes;
use scienceview_core::sv_mark::{
    SVGeometryCircle, SVGeometryImage, SVGeometryRectangle, SVGeometryText, SVImageInterpolation,
    SVMarkPrimitive, SVMarkShape, SVTextAlign, SVTextBaseline,
};

/// Serializes one mark primitive.
pub fn sv_serialize_mark_primitive(primitive: &SVMarkPrimitive<'_>, font_family: &str) -> String {
    let paint = sv_mark_paint_attributes(primitive.paint());

    match primitive.shape() {
        SVMarkShape::Path(path) => {
            format!("<path d=\"{}\"{paint}/>", sv_serialize_path_data(path))
        }
        SVMarkShape::Rectangle(rectangle) => sv_serialize_rectangle(rectangle, &paint),
        SVMarkShape::Circle(circle) => sv_serialize_circle(circle, &paint),
        SVMarkShape::Text(text) => sv_serialize_text(text, &paint, font_family),
        SVMarkShape::Image(image) => sv_serialize_image(image, &paint),
    }
}

fn sv_serialize_circle(circle: &SVGeometryCircle, paint: &str) -> String {
    format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"{paint}/>",
        sv_format_number(circle.centre().x()),
        sv_format_number(circle.centre().y()),
        sv_format_number(circle.radius()),
    )
}

fn sv_serialize_rectangle(rectangle: &SVGeometryRectangle, paint: &str) -> String {
    let solved = rectangle.rectangle();
    let mut element = format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
        sv_format_number(solved.x),
        sv_format_number(solved.y),
        sv_format_number(solved.width),
        sv_format_number(solved.height),
    );

    if rectangle.corner_radius() > 0.0 {
        element.push_str(" rx=\"");
        element.push_str(&sv_format_number(rectangle.corner_radius()));
        element.push('"');
    }

    element.push_str(paint);
    element.push_str("/>");
    element
}

fn sv_serialize_text(text: &SVGeometryText, paint: &str, font_family: &str) -> String {
    let family = if text.font_family().is_empty() {
        font_family
    } else {
        text.font_family()
    };

    format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" text-anchor=\"{}\" dominant-baseline=\"{}\"{paint}>{}</text>",
        sv_format_number(text.position().x()),
        sv_format_number(text.position().y()),
        sv_escape_text(family),
        sv_format_number(text.font_size()),
        sv_text_anchor(text.align()),
        sv_text_baseline(text.baseline()),
        sv_escape_text(text.text()),
    )
}

fn sv_serialize_image(image: &SVGeometryImage<'_>, paint: &str) -> String {
    let rectangle = image.rectangle();
    let uri = sv_encode_png_data_uri(image.width(), image.height(), image.pixels());
    let rendering = match image.interpolation() {
        SVImageInterpolation::Nearest => "pixelated",
        SVImageInterpolation::Bilinear => "auto",
    };

    format!(
        "<image x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" href=\"{}\" preserveAspectRatio=\"none\" image-rendering=\"{}\"{paint}/>",
        sv_format_number(rectangle.x),
        sv_format_number(rectangle.y),
        sv_format_number(rectangle.width),
        sv_format_number(rectangle.height),
        uri,
        rendering,
    )
}

fn sv_text_anchor(align: SVTextAlign) -> &'static str {
    match align {
        SVTextAlign::Left => "start",
        SVTextAlign::Center => "middle",
        SVTextAlign::Right => "end",
    }
}

fn sv_text_baseline(baseline: SVTextBaseline) -> &'static str {
    match baseline {
        SVTextBaseline::Top => "hanging",
        SVTextBaseline::Middle => "middle",
        SVTextBaseline::Bottom => "auto",
    }
}
