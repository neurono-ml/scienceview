//! Decoration primitives become SVG elements.

use crate::serialize::sv_escape_text;
use crate::serialize::sv_format_number;
use scienceview_core::sv_decoration::{
    SVDecorationPrimitive, SVPoint, SVRectanglePrimitive, SVTextAnchor,
};
use scienceview_core::sv_scale::SVColour;

/// Serializes one decoration primitive.
pub(crate) fn sv_serialize_decoration_primitive(
    primitive: &SVDecorationPrimitive,
    font_family: &str,
) -> String {
    match primitive {
        SVDecorationPrimitive::Line { start, end, stroke } => format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
            sv_format_number(start.x),
            sv_format_number(start.y),
            sv_format_number(end.x),
            sv_format_number(end.y),
            stroke.colour.to_hex_string(),
            sv_format_number(stroke.width),
        ),
        SVDecorationPrimitive::Rectangle(rectangle) => sv_serialize_rectangle(rectangle),
        SVDecorationPrimitive::Text {
            position,
            text,
            font_size,
            weight,
            colour,
            anchor,
            rotation_degrees,
        } => sv_serialize_text(
            *position,
            text,
            *font_size,
            *weight,
            *colour,
            *anchor,
            *rotation_degrees,
            font_family,
        ),
    }
}

fn sv_serialize_rectangle(rectangle: &SVRectanglePrimitive) -> String {
    let mut element = format!(
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
        sv_format_number(rectangle.rectangle.x),
        sv_format_number(rectangle.rectangle.y),
        sv_format_number(rectangle.rectangle.width),
        sv_format_number(rectangle.rectangle.height),
    );

    match rectangle.fill {
        Some(fill) => {
            element.push_str(" fill=\"");
            element.push_str(&fill.to_hex_string());
            element.push('"');
        }
        None => element.push_str(" fill=\"none\""),
    }

    if let Some(stroke) = rectangle.stroke {
        element.push_str(" stroke=\"");
        element.push_str(&stroke.colour.to_hex_string());
        element.push_str("\" stroke-width=\"");
        element.push_str(&sv_format_number(stroke.width));
        element.push('"');
    }

    element.push_str("/>");
    element
}

#[allow(clippy::too_many_arguments)]
fn sv_serialize_text(
    position: SVPoint,
    text: &str,
    font_size: f64,
    weight: u16,
    colour: SVColour,
    anchor: SVTextAnchor,
    rotation_degrees: f64,
    font_family: &str,
) -> String {
    let mut element = format!(
        "<text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"{}\" text-anchor=\"{}\" fill=\"{}\"",
        sv_format_number(position.x),
        sv_format_number(position.y),
        sv_escape_text(font_family),
        sv_format_number(font_size),
        weight,
        sv_text_anchor(anchor),
        colour.to_hex_string(),
    );

    if rotation_degrees.is_finite() && rotation_degrees != 0.0 {
        element.push_str(" transform=\"rotate(");
        element.push_str(&sv_format_number(rotation_degrees));
        element.push(' ');
        element.push_str(&sv_format_number(position.x));
        element.push(' ');
        element.push_str(&sv_format_number(position.y));
        element.push_str(")\"");
    }

    element.push('>');
    element.push_str(&sv_escape_text(text));
    element.push_str("</text>");
    element
}

fn sv_text_anchor(anchor: SVTextAnchor) -> &'static str {
    match anchor {
        SVTextAnchor::Start => "start",
        SVTextAnchor::Middle => "middle",
        SVTextAnchor::End => "end",
    }
}
