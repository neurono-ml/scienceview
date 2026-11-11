//! Mark shapes become SVG elements.
//!
//! One primitive is one element. Geometry already arrived in display space, so
//! this module never asks a scale or a layout engine for a number.

use crate::image::sv_encode_png_data_uri;
use crate::serialize::sv_escape_text;
use crate::serialize::sv_format_number;
use crate::serialize::sv_mark_paint_attributes;
use scienceview_core::sv_mark::{
    SVArcTo, SVGeometryCircle, SVGeometryImage, SVGeometryPath, SVGeometryRectangle,
    SVGeometryText, SVImageInterpolation, SVMarkGeometry, SVMarkPaint, SVMarkPoint,
    SVMarkPrimitive, SVMarkShape, SVPathCommand, SVTextAlign, SVTextBaseline,
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

/// Serializes a whole geometry, grouping consecutive equal-paint circles.
///
/// A scatter of ten thousand markers would otherwise be ten thousand parse
/// nodes. Circles that share one paint become a single `<path>` with one
/// subpath per marker, so the geometry is unchanged and the document stays
/// openable. Any other shape, or a change of paint, closes the run.
pub(crate) fn sv_serialize_mark_geometry(
    geometry: &SVMarkGeometry<'_>,
    font_family: &str,
) -> Vec<String> {
    let mut elements = Vec::new();
    let mut circle_run: Option<(SVMarkPaint, String)> = None;

    for primitive in geometry.primitives() {
        match primitive.shape() {
            SVMarkShape::Circle(circle) if circle.radius() > 0.0 => {
                let subpath = sv_circle_subpath(circle);
                match &mut circle_run {
                    Some((paint, data)) if *paint == *primitive.paint() => {
                        data.push(' ');
                        data.push_str(&subpath);
                    }
                    Some(_) => {
                        sv_flush_circle_run(&mut elements, &mut circle_run);
                        circle_run = Some((primitive.paint().clone(), subpath));
                    }
                    None => circle_run = Some((primitive.paint().clone(), subpath)),
                }
            }
            _ => {
                sv_flush_circle_run(&mut elements, &mut circle_run);
                elements.push(sv_serialize_mark_primitive(primitive, font_family));
            }
        }
    }

    sv_flush_circle_run(&mut elements, &mut circle_run);
    elements
}

fn sv_flush_circle_run(elements: &mut Vec<String>, run: &mut Option<(SVMarkPaint, String)>) {
    if let Some((paint, data)) = run.take() {
        let attributes = sv_mark_paint_attributes(&paint);
        elements.push(format!("<path d=\"{data}\"{attributes}/>"));
    }
}

fn sv_circle_subpath(circle: &SVGeometryCircle) -> String {
    let centre = circle.centre();
    let radius = sv_format_number(circle.radius());
    let diameter = sv_format_number(circle.radius() * 2.0);

    format!(
        "M {} {} m -{} 0 a {} {} 0 1 0 {} 0 a {} {} 0 1 0 -{} 0 Z",
        sv_format_number(centre.x()),
        sv_format_number(centre.y()),
        radius,
        radius,
        radius,
        diameter,
        radius,
        radius,
        diameter,
    )
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

/// Serializes path commands as one `d` attribute value.
pub(crate) fn sv_serialize_path_data(path: &SVGeometryPath) -> String {
    let mut data = String::new();
    let mut current: Option<SVMarkPoint> = None;

    for command in path.commands() {
        match command {
            SVPathCommand::MoveTo(point) => {
                sv_push_command(&mut data, "M", *point);
                current = Some(*point);
            }
            SVPathCommand::LineTo(point) => {
                sv_push_command(&mut data, "L", *point);
                current = Some(*point);
            }
            SVPathCommand::ArcTo(arc) => {
                let start = sv_arc_start(arc);
                if current.is_none() {
                    sv_push_command(&mut data, "M", start);
                }
                sv_push_arc(&mut data, arc);
                current = Some(sv_arc_end(arc));
            }
            SVPathCommand::Close => data.push_str(" Z"),
        }
    }

    data.trim_start().to_string()
}

fn sv_push_command(data: &mut String, command: &str, point: SVMarkPoint) {
    data.push(' ');
    data.push_str(command);
    data.push(' ');
    data.push_str(&sv_format_number(point.x()));
    data.push(' ');
    data.push_str(&sv_format_number(point.y()));
}

fn sv_push_arc(data: &mut String, arc: &SVArcTo) {
    let end = sv_arc_end(arc);
    let sweep = arc.end_angle() >= arc.start_angle();
    let large_arc = (arc.end_angle() - arc.start_angle()).abs() > std::f64::consts::PI;
    let radius = sv_format_number(arc.radius());

    data.push_str(" A ");
    data.push_str(&radius);
    data.push(' ');
    data.push_str(&radius);
    data.push_str(" 0 ");
    data.push_str(if large_arc { "1" } else { "0" });
    data.push(' ');
    data.push_str(if sweep { "1" } else { "0" });
    data.push(' ');
    data.push_str(&sv_format_number(end.x()));
    data.push(' ');
    data.push_str(&sv_format_number(end.y()));
}

fn sv_arc_start(arc: &SVArcTo) -> SVMarkPoint {
    sv_point_at_angle(arc.centre(), arc.radius(), arc.start_angle())
}

fn sv_arc_end(arc: &SVArcTo) -> SVMarkPoint {
    sv_point_at_angle(arc.centre(), arc.radius(), arc.end_angle())
}

fn sv_point_at_angle(centre: SVMarkPoint, radius: f64, angle: f64) -> SVMarkPoint {
    SVMarkPoint::new(
        centre.x() + radius * angle.cos(),
        centre.y() + radius * angle.sin(),
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
