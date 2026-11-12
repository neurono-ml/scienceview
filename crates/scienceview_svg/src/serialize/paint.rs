//! Paint attributes in a fixed order.

use crate::serialize::sv_format_number;
use scienceview_core::sv_mark::{SVDashPattern, SVLineCap, SVLineJoin, SVMarkPaint};

/// Serializes the paint of a mark primitive.
///
/// The attribute order never depends on which attributes are present: `fill`
/// always comes first, and the stroke group follows in a fixed order. A solid
/// stroke omits `stroke-dasharray` rather than writing an empty one.
pub(crate) fn sv_mark_paint_attributes(paint: &SVMarkPaint) -> String {
    let mut attributes = String::new();

    attributes.push_str(" fill=\"");
    match paint.fill() {
        Some(fill) => attributes.push_str(&fill.to_hex_string()),
        None => attributes.push_str("none"),
    }
    attributes.push('"');

    if let Some(stroke) = paint.stroke() {
        attributes.push_str(" stroke=\"");
        attributes.push_str(&stroke.to_hex_string());
        attributes.push_str("\" stroke-width=\"");
        attributes.push_str(&sv_format_number(paint.stroke_width()));
        attributes.push('"');
        attributes.push_str(" stroke-linecap=\"");
        attributes.push_str(sv_line_cap(paint.cap()));
        attributes.push_str("\" stroke-linejoin=\"");
        attributes.push_str(sv_line_join(paint.join()));
        attributes.push('"');

        let dash = paint.dash();
        if !dash.is_solid() {
            attributes.push_str(" stroke-dasharray=\"");
            attributes.push_str(&sv_dash(dash));
            attributes.push('"');
        }
    }

    let opacity = f64::from(paint.opacity());
    if (opacity - 1.0).abs() > f64::EPSILON {
        attributes.push_str(" opacity=\"");
        attributes.push_str(&sv_format_number(opacity));
        attributes.push('"');
    }

    attributes
}

fn sv_dash(dash: &SVDashPattern) -> String {
    dash.segments()
        .iter()
        .copied()
        .map(sv_format_number)
        .collect::<Vec<_>>()
        .join(" ")
}

fn sv_line_cap(cap: SVLineCap) -> &'static str {
    match cap {
        SVLineCap::Butt => "butt",
        SVLineCap::Round => "round",
        SVLineCap::Square => "square",
    }
}

fn sv_line_join(join: SVLineJoin) -> &'static str {
    match join {
        SVLineJoin::Miter => "miter",
        SVLineJoin::Round => "round",
        SVLineJoin::Bevel => "bevel",
    }
}
