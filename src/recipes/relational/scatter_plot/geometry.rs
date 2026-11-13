//! Channel application on top of the dot kernel's geometry.
//!
//! The dot kernel owns placement and the default shape. A recipe is what maps
//! the `series`, `colour` and `size` channels onto that geometry, so this
//! module recolours and rescales primitives without re-deriving a position and
//! groups a large scatter into one compound path per paint.

use crate::sv_error::SVError;
use crate::sv_mark::{
    SVArcTo, SVGeometryCircle, SVGeometryPath, SVGeometryRectangle, SVMarkPaint, SVMarkPoint,
    SVMarkPrimitive, SVMarkShape, SVPathCommand,
};
use crate::sv_scale::SVColour;
use std::f64::consts::PI;

/// Above this many circles a scatter draws one compound path per paint.
pub(super) const SV_LARGE_SCATTER_THRESHOLD: usize = 512;

/// The paint one scatter point receives.
pub(super) fn sv_scatter_paint(
    colour: SVColour,
    edge_colour: Option<SVColour>,
    edge_width: f64,
    opacity: f32,
    cross: bool,
    fallback_line_width: f64,
) -> SVMarkPaint {
    let stroke_colour = edge_colour.unwrap_or(colour);
    let stroke_width = if edge_width > 0.0 { edge_width } else { 0.0 };

    let paint = if cross {
        SVMarkPaint::stroked(stroke_colour, stroke_width.max(fallback_line_width))
    } else if stroke_width > 0.0 {
        SVMarkPaint::filled(colour).with_stroke(stroke_colour, stroke_width)
    } else {
        SVMarkPaint::filled(colour)
    };
    paint.with_opacity(opacity)
}

/// Rebuilds a primitive with an owned shape and an explicit paint.
///
/// A scatter only draws dot shapes, so the raster variant is unreachable; it is
/// skipped instead of being copied.
pub(super) fn sv_owned_primitive(
    primitive: &SVMarkPrimitive<'_>,
    paint: SVMarkPaint,
    size_factor: f64,
) -> Option<SVMarkPrimitive<'static>> {
    let shape = sv_scale_shape(primitive.shape(), size_factor)?;
    Some(SVMarkPrimitive::new(shape, paint))
}

fn sv_scale_shape(shape: &SVMarkShape<'_>, factor: f64) -> Option<SVMarkShape<'static>> {
    let factor = if factor.is_finite() && factor > 0.0 {
        factor
    } else {
        1.0
    };
    Some(match shape {
        SVMarkShape::Circle(circle) => SVMarkShape::Circle(SVGeometryCircle::new(
            circle.centre(),
            circle.radius() * factor,
        )),
        SVMarkShape::Rectangle(rectangle) => {
            let bounds = rectangle.rectangle();
            let centre = SVMarkPoint::new(
                bounds.x + bounds.width / 2.0,
                bounds.y + bounds.height / 2.0,
            );
            let scaled = crate::sv_layout::SVResolvedRectangle::new(
                centre.x() - bounds.width * factor / 2.0,
                centre.y() - bounds.height * factor / 2.0,
                bounds.width * factor,
                bounds.height * factor,
            );
            SVMarkShape::Rectangle(SVGeometryRectangle::new(
                scaled,
                rectangle.corner_radius() * factor,
            ))
        }
        SVMarkShape::Path(path) => SVMarkShape::Path(sv_scale_path(path, factor)),
        SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
        SVMarkShape::Image(_) => return None,
    })
}

fn sv_scale_path(path: &SVGeometryPath, factor: f64) -> SVGeometryPath {
    let points: Vec<SVMarkPoint> = path
        .commands()
        .iter()
        .filter_map(|command| match command {
            SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) => Some(*point),
            SVPathCommand::ArcTo(arc) => Some(arc.centre()),
            SVPathCommand::Close => None,
        })
        .collect();
    let centre = sv_centre(&points);

    let commands = path
        .commands()
        .iter()
        .map(|command| match command {
            SVPathCommand::MoveTo(point) => {
                SVPathCommand::MoveTo(sv_scale_point(*point, centre, factor))
            }
            SVPathCommand::LineTo(point) => {
                SVPathCommand::LineTo(sv_scale_point(*point, centre, factor))
            }
            SVPathCommand::ArcTo(arc) => SVPathCommand::ArcTo(SVArcTo::new(
                sv_scale_point(arc.centre(), centre, factor),
                arc.radius() * factor,
                arc.start_angle(),
                arc.end_angle(),
            )),
            SVPathCommand::Close => SVPathCommand::Close,
        })
        .collect();
    SVGeometryPath::new(commands)
}

fn sv_centre(points: &[SVMarkPoint]) -> SVMarkPoint {
    let Some(first) = points.first() else {
        return SVMarkPoint::default();
    };
    let (mut minimum_x, mut maximum_x) = (first.x(), first.x());
    let (mut minimum_y, mut maximum_y) = (first.y(), first.y());
    for point in points {
        minimum_x = minimum_x.min(point.x());
        maximum_x = maximum_x.max(point.x());
        minimum_y = minimum_y.min(point.y());
        maximum_y = maximum_y.max(point.y());
    }
    SVMarkPoint::new((minimum_x + maximum_x) / 2.0, (minimum_y + maximum_y) / 2.0)
}

fn sv_scale_point(point: SVMarkPoint, centre: SVMarkPoint, factor: f64) -> SVMarkPoint {
    SVMarkPoint::new(
        centre.x() + (point.x() - centre.x()) * factor,
        centre.y() + (point.y() - centre.y()) * factor,
    )
}

/// Groups a large circle scatter into one compound path per paint.
///
/// The document stays small without changing what is drawn: each circle
/// becomes two half arcs in a path that shares one paint with its peers.
pub(super) fn sv_group_large_scatter(
    primitives: Vec<SVMarkPrimitive<'static>>,
) -> Vec<SVMarkPrimitive<'static>> {
    let mut groups: Vec<(SVMarkPaint, Vec<(SVMarkPoint, f64)>)> = Vec::new();
    for primitive in &primitives {
        let SVMarkShape::Circle(circle) = primitive.shape() else {
            return primitives;
        };
        let paint = primitive.paint().clone();
        let position = match groups.iter().position(|(candidate, _)| *candidate == paint) {
            Some(position) => position,
            None => {
                groups.push((paint, Vec::new()));
                groups.len() - 1
            }
        };
        if let Some((_, circles)) = groups.get_mut(position) {
            circles.push((circle.centre(), circle.radius()));
        }
    }

    groups
        .into_iter()
        .map(|(paint, circles)| {
            let mut commands = Vec::with_capacity(circles.len() * 4);
            for (centre, radius) in circles {
                commands.push(SVPathCommand::MoveTo(SVMarkPoint::new(
                    centre.x() + radius,
                    centre.y(),
                )));
                commands.push(SVPathCommand::ArcTo(SVArcTo::new(centre, radius, 0.0, PI)));
                commands.push(SVPathCommand::ArcTo(SVArcTo::new(
                    centre,
                    radius,
                    PI,
                    2.0 * PI,
                )));
                commands.push(SVPathCommand::Close);
            }
            SVMarkPrimitive::new(SVMarkShape::Path(SVGeometryPath::new(commands)), paint)
        })
        .collect()
}

/// Rejects the unreachable raster shape explicitly.
pub(super) fn sv_unreachable_scatter_shape() -> SVError {
    SVError::InvalidConfiguration {
        message: "a scatter dot produced a raster primitive".to_string(),
    }
}
