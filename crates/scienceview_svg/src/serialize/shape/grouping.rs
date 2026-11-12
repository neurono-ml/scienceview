//! Grouping a run of equal-paint circles into one path.

use super::elements::sv_serialize_mark_primitive;
use crate::serialize::sv_format_number;
use crate::serialize::sv_mark_paint_attributes;
use scienceview_core::sv_mark::{SVGeometryCircle, SVMarkGeometry, SVMarkPaint, SVMarkShape};

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
