//! Error-bar geometry: the interval segment and its caps.

use super::positions::{sv_boundary_positions, sv_horizontal_boundary_positions};
use crate::sv_error::SVError;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVErrorBarDirection, SVErrorBarOptions, SVGeometryPath, SVMarkGeometry, SVMarkPaint,
    SVMarkPrimitive, SVMarkRequest, SVMarkShape, SVPathCommand, SVPoint,
};

/// Builds the geometry of an error-bar mark.
///
/// The lower and upper channels carry the interval. With a vertical direction
/// they are placed on the vertical scale, with a horizontal direction on the
/// horizontal scale, and with `Both` the mark draws an interval on each axis
/// from the same pair, which is how a two-dimensional interval is expressed.
pub(crate) fn sv_error_bar_geometry<'data>(
    options: &SVErrorBarOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVPoint],
) -> Result<SVMarkGeometry<'data>, SVError> {
    let lower_column = request
        .lower()
        .ok_or_else(|| sv_missing_interval("lower"))?;
    let upper_column = request
        .upper()
        .ok_or_else(|| sv_missing_interval("upper"))?;
    let direction = options.direction_value();

    let vertical = if matches!(
        direction,
        SVErrorBarDirection::Vertical | SVErrorBarDirection::Both
    ) {
        Some((
            sv_boundary_positions(request, lower_column, "error bar lower values")?,
            sv_boundary_positions(request, upper_column, "error bar upper values")?,
        ))
    } else {
        None
    };
    let horizontal = if matches!(
        direction,
        SVErrorBarDirection::Horizontal | SVErrorBarDirection::Both
    ) {
        Some((
            sv_horizontal_boundary_positions(request, lower_column, "error bar lower values")?,
            sv_horizontal_boundary_positions(request, upper_column, "error bar upper values")?,
        ))
    } else {
        None
    };

    let theme = request.theme();
    let width = match options.stroke_width_value() {
        Some(width) => sv_resolve_length(Some(width), theme.line_width(), request.plot_area()),
        None => theme.line_width(),
    };
    let cap_size = sv_resolve_length(Some(options.cap_size_value()), 0.0, request.plot_area());
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = SVMarkPaint::stroked(colour, width).with_opacity(opacity);

    let primitives = points
        .iter()
        .copied()
        .enumerate()
        .map(|(index, point)| {
            let mut path = SVGeometryPath::default();
            if let Some((lower, upper)) = &vertical {
                sv_vertical_interval(&mut path, point.x(), lower[index], upper[index], cap_size);
            }
            if let Some((lower, upper)) = &horizontal {
                sv_horizontal_interval(&mut path, lower[index], upper[index], point.y(), cap_size);
            }
            SVMarkPrimitive::new(SVMarkShape::Path(path), paint.clone())
        })
        .collect();

    Ok(SVMarkGeometry::new(primitives))
}

/// Draws a vertical interval with its caps.
fn sv_vertical_interval(path: &mut SVGeometryPath, x: f64, lower: f64, upper: f64, cap_size: f64) {
    path.push(SVPathCommand::MoveTo(SVPoint::new(x, lower)));
    path.push(SVPathCommand::LineTo(SVPoint::new(x, upper)));
    if cap_size > 0.0 {
        path.push(SVPathCommand::MoveTo(SVPoint::new(x - cap_size, lower)));
        path.push(SVPathCommand::LineTo(SVPoint::new(x + cap_size, lower)));
        path.push(SVPathCommand::MoveTo(SVPoint::new(x - cap_size, upper)));
        path.push(SVPathCommand::LineTo(SVPoint::new(x + cap_size, upper)));
    }
}

/// Draws a horizontal interval with its caps.
fn sv_horizontal_interval(
    path: &mut SVGeometryPath,
    lower: f64,
    upper: f64,
    y: f64,
    cap_size: f64,
) {
    path.push(SVPathCommand::MoveTo(SVPoint::new(lower, y)));
    path.push(SVPathCommand::LineTo(SVPoint::new(upper, y)));
    if cap_size > 0.0 {
        path.push(SVPathCommand::MoveTo(SVPoint::new(lower, y - cap_size)));
        path.push(SVPathCommand::LineTo(SVPoint::new(lower, y + cap_size)));
        path.push(SVPathCommand::MoveTo(SVPoint::new(upper, y - cap_size)));
        path.push(SVPathCommand::LineTo(SVPoint::new(upper, y + cap_size)));
    }
}

fn sv_missing_interval(name: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: format!("an error bar needs a {name} interval column"),
    }
}
