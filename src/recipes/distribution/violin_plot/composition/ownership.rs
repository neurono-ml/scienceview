//! Owning a mark geometry and, for a vertical violin, transposing it.
//!
//! The `Area` mark closes against the vertical scale's baseline. A vertical
//! violin is therefore composed with the value channel on the horizontal scale
//! and the category channel on the vertical scale, then transposed back into
//! display space so the closed edge is the category centre.

use std::f64::consts::FRAC_PI_2;

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{
    SVArcTo, SVGeometryCircle, SVGeometryPath, SVGeometryRectangle, SVGeometryText, SVMarkGeometry,
    SVMarkPoint, SVMarkPrimitive, SVMarkShape, SVPathCommand,
};

use super::super::configuration::sv_invalid;

pub(crate) fn sv_own_geometry(
    geometry: SVMarkGeometry<'_>,
    transpose: bool,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let primitives = geometry
        .primitives()
        .iter()
        .map(|primitive| {
            Ok(SVMarkPrimitive::new(
                sv_own_shape(primitive.shape(), transpose)?,
                primitive.paint().clone(),
            ))
        })
        .collect::<Result<Vec<_>, SVError>>()?;
    Ok(SVMarkGeometry::new(primitives))
}

fn sv_own_shape(shape: &SVMarkShape<'_>, transpose: bool) -> Result<SVMarkShape<'static>, SVError> {
    match shape {
        SVMarkShape::Path(path) if transpose => Ok(SVMarkShape::Path(SVGeometryPath::new(
            path.commands().iter().map(sv_transpose_command).collect(),
        ))),
        SVMarkShape::Path(path) => Ok(SVMarkShape::Path(path.clone())),
        SVMarkShape::Rectangle(rectangle) if transpose => {
            Ok(SVMarkShape::Rectangle(sv_transpose_rectangle(*rectangle)))
        }
        SVMarkShape::Rectangle(rectangle) => Ok(SVMarkShape::Rectangle(*rectangle)),
        SVMarkShape::Circle(circle) if transpose => Ok(SVMarkShape::Circle(SVGeometryCircle::new(
            sv_transpose_point(circle.centre()),
            circle.radius(),
        ))),
        SVMarkShape::Circle(circle) => Ok(SVMarkShape::Circle(*circle)),
        SVMarkShape::Text(text) if transpose => Ok(SVMarkShape::Text(SVGeometryText::new(
            sv_transpose_point(text.position()),
            text.text().to_string(),
            text.font_family().to_string(),
            text.font_size(),
            text.align(),
            text.baseline(),
        ))),
        SVMarkShape::Text(text) => Ok(SVMarkShape::Text(text.clone())),
        SVMarkShape::Image(_) => Err(sv_invalid(
            "a violin plot draws paths, markers and lines only; an image primitive is not part of its composition",
        )),
    }
}

fn sv_transpose_command(command: &SVPathCommand) -> SVPathCommand {
    match command {
        SVPathCommand::MoveTo(point) => SVPathCommand::MoveTo(sv_transpose_point(*point)),
        SVPathCommand::LineTo(point) => SVPathCommand::LineTo(sv_transpose_point(*point)),
        SVPathCommand::ArcTo(arc) => SVPathCommand::ArcTo(SVArcTo::new(
            sv_transpose_point(arc.centre()),
            arc.radius(),
            FRAC_PI_2 - arc.end_angle(),
            FRAC_PI_2 - arc.start_angle(),
        )),
        SVPathCommand::Close => SVPathCommand::Close,
    }
}

fn sv_transpose_point(point: SVMarkPoint) -> SVMarkPoint {
    SVMarkPoint::new(point.y(), point.x())
}

fn sv_transpose_rectangle(rectangle: SVGeometryRectangle) -> SVGeometryRectangle {
    let solved = rectangle.rectangle();
    let transposed = scienceview_core::sv_layout::SVResolvedRectangle::new(
        solved.y,
        solved.x,
        solved.height,
        solved.width,
    );
    SVGeometryRectangle::new(transposed, rectangle.corner_radius())
}
