//! Translating a composed panel into the rectangle the joint layout solved.
//!
//! A recipe composed on its own solves its marks against its own plot area.
//! The joint plot reuses that composition and maps its primitives from the
//! source panel rectangle to the target one; no mark is recomputed and no
//! kernel is duplicated.

use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVArcTo, SVGeometryCircle, SVGeometryPath, SVGeometryRectangle, SVGeometryText, SVMarkGeometry,
    SVMarkPoint, SVMarkPrimitive, SVMarkShape, SVPathCommand,
};

/// Maps every primitive of `geometry` from `source` onto `target`.
pub(super) fn sv_translate_geometry(
    geometry: &SVMarkGeometry<'static>,
    source: SVResolvedRectangle,
    target: SVResolvedRectangle,
) -> SVMarkGeometry<'static> {
    let mapping = SVAffineMapping::new(source, target);
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            SVMarkPrimitive::new(
                sv_map_shape(primitive.shape(), &mapping),
                primitive.paint().clone(),
            )
        })
        .collect();
    SVMarkGeometry::new(primitives)
}

/// Maps a list of geometries, preserving draw order.
pub(super) fn sv_translate_geometries(
    geometries: &[SVMarkGeometry<'static>],
    source: SVResolvedRectangle,
    target: SVResolvedRectangle,
) -> Vec<SVMarkGeometry<'static>> {
    geometries
        .iter()
        .map(|geometry| sv_translate_geometry(geometry, source, target))
        .collect()
}

struct SVAffineMapping {
    horizontal_scale: f64,
    vertical_scale: f64,
    horizontal_offset: f64,
    vertical_offset: f64,
}

impl SVAffineMapping {
    fn new(source: SVResolvedRectangle, target: SVResolvedRectangle) -> Self {
        let horizontal_scale = sv_ratio(target.width, source.width);
        let vertical_scale = sv_ratio(target.height, source.height);
        Self {
            horizontal_scale,
            vertical_scale,
            horizontal_offset: target.x - source.x * horizontal_scale,
            vertical_offset: target.y - source.y * vertical_scale,
        }
    }

    fn point(&self, point: SVMarkPoint) -> SVMarkPoint {
        SVMarkPoint::new(
            point.x() * self.horizontal_scale + self.horizontal_offset,
            point.y() * self.vertical_scale + self.vertical_offset,
        )
    }

    fn horizontal(&self, value: f64) -> f64 {
        value * self.horizontal_scale
    }

    fn vertical(&self, value: f64) -> f64 {
        value * self.vertical_scale
    }
}

fn sv_ratio(target: f64, source: f64) -> f64 {
    if source.is_finite() && source.abs() > f64::EPSILON && target.is_finite() {
        target / source
    } else {
        1.0
    }
}

fn sv_map_shape(shape: &SVMarkShape<'_>, mapping: &SVAffineMapping) -> SVMarkShape<'static> {
    match shape {
        SVMarkShape::Path(path) => SVMarkShape::Path(sv_map_path(path, mapping)),
        SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(SVGeometryRectangle::new(
            sv_map_rectangle(rectangle.rectangle(), mapping),
            rectangle.corner_radius() * mapping.horizontal_scale.min(mapping.vertical_scale),
        )),
        SVMarkShape::Circle(circle) => SVMarkShape::Circle(SVGeometryCircle::new(
            mapping.point(circle.centre()),
            circle.radius() * mapping.horizontal_scale.min(mapping.vertical_scale),
        )),
        SVMarkShape::Text(text) => SVMarkShape::Text(SVGeometryText::new(
            mapping.point(text.position()),
            text.text(),
            text.font_family(),
            text.font_size(),
            text.align(),
            text.baseline(),
        )),
        // A composed panel never carries a borrowed raster; translate to an
        // empty path rather than pretending the pixels are owned.
        SVMarkShape::Image(_) => SVMarkShape::Path(SVGeometryPath::default()),
    }
}

fn sv_map_path(path: &SVGeometryPath, mapping: &SVAffineMapping) -> SVGeometryPath {
    let commands = path
        .commands()
        .iter()
        .map(|command| match *command {
            SVPathCommand::MoveTo(point) => SVPathCommand::MoveTo(mapping.point(point)),
            SVPathCommand::LineTo(point) => SVPathCommand::LineTo(mapping.point(point)),
            SVPathCommand::ArcTo(arc) => SVPathCommand::ArcTo(SVArcTo::new(
                mapping.point(arc.centre()),
                arc.radius() * mapping.horizontal_scale.min(mapping.vertical_scale),
                arc.start_angle(),
                arc.end_angle(),
            )),
            SVPathCommand::Close => SVPathCommand::Close,
        })
        .collect();
    SVGeometryPath::new(commands)
}

fn sv_map_rectangle(
    rectangle: SVResolvedRectangle,
    mapping: &SVAffineMapping,
) -> SVResolvedRectangle {
    SVResolvedRectangle::new(
        rectangle.x * mapping.horizontal_scale + mapping.horizontal_offset,
        rectangle.y * mapping.vertical_scale + mapping.vertical_offset,
        mapping.horizontal(rectangle.width),
        mapping.vertical(rectangle.height),
    )
}
