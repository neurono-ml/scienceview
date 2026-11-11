//! Wedge geometry: a pie sector, or an annulus when a radius is given.

use crate::sv_error::SVError;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVArcTo, SVGeometryPath, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive,
    SVMarkRequest, SVMarkShape, SVPathCommand, SVWedgeOptions,
};

/// Builds the geometry of a wedge mark.
pub(crate) fn sv_wedge_geometry<'data>(
    options: &SVWedgeOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVMarkPoint],
) -> Result<SVMarkGeometry<'data>, SVError> {
    let plot_area = request.plot_area();
    let radius = sv_resolve_length(
        options.radius_value(),
        plot_area.width.min(plot_area.height) / 2.0,
        plot_area,
    );
    if radius <= 0.0 {
        return Ok(SVMarkGeometry::default());
    }
    let inner_radius = options.inner_radius_value();
    if inner_radius >= radius {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "a wedge inner radius ({inner_radius}) must stay below its radius ({radius})"
            ),
        });
    }

    let theme = request.theme();
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = SVMarkPaint::filled(colour).with_opacity(opacity);
    let start_angle = options.start_angle_value();
    let end_angle = options.end_angle_value();

    let primitives = points
        .iter()
        .copied()
        .map(|centre| {
            let path = sv_wedge_path(centre, radius, inner_radius, start_angle, end_angle);
            SVMarkPrimitive::new(SVMarkShape::Path(path), paint.clone())
        })
        .collect();

    Ok(SVMarkGeometry::new(primitives))
}

/// Builds the path of one sector or annulus.
fn sv_wedge_path(
    centre: SVMarkPoint,
    radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> SVGeometryPath {
    let mut path = SVGeometryPath::default();
    let outer_start = sv_point_at_angle(centre, radius, start_angle);

    if inner_radius <= 0.0 {
        path.push(SVPathCommand::MoveTo(centre));
        path.push(SVPathCommand::LineTo(outer_start));
        path.push(SVPathCommand::ArcTo(SVArcTo::new(
            centre,
            radius,
            start_angle,
            end_angle,
        )));
        path.push(SVPathCommand::Close);
        return path;
    }

    let inner_end = sv_point_at_angle(centre, inner_radius, end_angle);
    path.push(SVPathCommand::MoveTo(outer_start));
    path.push(SVPathCommand::ArcTo(SVArcTo::new(
        centre,
        radius,
        start_angle,
        end_angle,
    )));
    path.push(SVPathCommand::LineTo(inner_end));
    path.push(SVPathCommand::ArcTo(SVArcTo::new(
        centre,
        inner_radius,
        end_angle,
        start_angle,
    )));
    path.push(SVPathCommand::Close);

    path
}

/// Returns the point at `radius` and `angle` around a centre.
fn sv_point_at_angle(centre: SVMarkPoint, radius: f64, angle: f64) -> SVMarkPoint {
    SVMarkPoint::new(
        centre.x() + radius * angle.cos(),
        centre.y() + radius * angle.sin(),
    )
}
