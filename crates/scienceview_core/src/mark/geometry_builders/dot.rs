//! Dot geometry: one shape per instance.

use super::paths::{sv_polygon_offsets, sv_regular_polygon};
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVDotOptions, SVDotShape, SVDotSizeMode, SVFillMode, SVGeometryCircle, SVGeometryPath,
    SVGeometryRectangle, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkRequest, SVMarkShape,
    SVPathCommand, SVPoint,
};
use crate::sv_scale::SVColour;

/// Builds the geometry of a dot mark.
pub(crate) fn sv_dot_geometry<'data>(
    options: &SVDotOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVPoint],
) -> Result<SVMarkGeometry<'data>, SVError> {
    let theme = request.theme();
    let plot_area = request.plot_area();
    let diameter = sv_resolve_length(options.size_value(), theme.mark_size(), plot_area);
    let radius = match options.size_mode_value() {
        SVDotSizeMode::Diameter => diameter / 2.0,
        SVDotSizeMode::Area => (diameter / std::f64::consts::PI).sqrt(),
    };
    let stroke_width = sv_resolve_length(Some(options.stroke_width_value()), 0.0, plot_area);
    let is_cross = options.shape_value() == SVDotShape::Cross;
    let effective_stroke_width = if is_cross && stroke_width <= 0.0 {
        theme.line_width()
    } else {
        stroke_width
    };
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = sv_dot_paint(
        options.fill_value(),
        is_cross,
        colour,
        stroke_width,
        effective_stroke_width,
        opacity,
    );

    let primitives = points
        .iter()
        .copied()
        .map(|point| {
            let shape = sv_dot_shape(options.shape_value(), point, radius);
            SVMarkPrimitive::new(shape, paint.clone())
        })
        .collect();

    Ok(SVMarkGeometry::new(primitives))
}

/// Resolves the paint of a dot from its fill mode.
///
/// A cross is stroked-only: its two segments have no interior, so a fill would
/// draw nothing. The shape therefore always receives a stroke, and a zero
/// stroke width falls back to a visible default before this function runs.
fn sv_dot_paint(
    fill_mode: SVFillMode,
    is_cross: bool,
    colour: SVColour,
    stroke_width: f64,
    effective_stroke_width: f64,
    opacity: f32,
) -> SVMarkPaint {
    let paint = match fill_mode {
        _ if is_cross => SVMarkPaint::stroked(colour, effective_stroke_width),
        SVFillMode::Filled => SVMarkPaint::filled(colour),
        SVFillMode::Stroked => SVMarkPaint::stroked(colour, stroke_width),
        SVFillMode::Both => SVMarkPaint::filled(colour).with_stroke(colour, stroke_width),
    };

    paint.with_opacity(opacity)
}

/// Builds the shape of one dot.
fn sv_dot_shape(shape: SVDotShape, centre: SVPoint, radius: f64) -> SVMarkShape<'static> {
    match shape {
        SVDotShape::Circle => SVMarkShape::Circle(SVGeometryCircle::new(centre, radius)),
        SVDotShape::Square => SVMarkShape::Rectangle(SVGeometryRectangle::new(
            SVResolvedRectangle::new(
                centre.x() - radius,
                centre.y() - radius,
                radius * 2.0,
                radius * 2.0,
            ),
            0.0,
        )),
        SVDotShape::Triangle => SVMarkShape::Path(sv_polygon(centre, radius, 3)),
        SVDotShape::Diamond => SVMarkShape::Path(sv_polygon(centre, radius, 4)),
        SVDotShape::Hexagon => SVMarkShape::Path(sv_polygon(centre, radius, 6)),
        SVDotShape::Cross => SVMarkShape::Path(sv_cross(centre, radius)),
    }
}

/// Builds a regular polygon path around a centre.
fn sv_polygon(centre: SVPoint, radius: f64, sides: usize) -> SVGeometryPath {
    sv_regular_polygon(centre, &sv_polygon_offsets(radius, sides), 0.0)
}

/// Builds the two segments of a cross.
fn sv_cross(centre: SVPoint, arm: f64) -> SVGeometryPath {
    let mut path = SVGeometryPath::default();
    path.push(SVPathCommand::MoveTo(SVPoint::new(
        centre.x() - arm,
        centre.y(),
    )));
    path.push(SVPathCommand::LineTo(SVPoint::new(
        centre.x() + arm,
        centre.y(),
    )));
    path.push(SVPathCommand::MoveTo(SVPoint::new(
        centre.x(),
        centre.y() - arm,
    )));
    path.push(SVPathCommand::LineTo(SVPoint::new(
        centre.x(),
        centre.y() + arm,
    )));

    path
}
