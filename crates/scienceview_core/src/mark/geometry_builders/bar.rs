//! Bar geometry: a rectangle grown from a baseline along the category slot.

use super::positions::sv_scale_position;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVBarOptions, SVBarOrientation, SVGeometryRectangle, SVMarkGeometry, SVMarkPaint, SVMarkPoint,
    SVMarkPrimitive, SVMarkRequest, SVMarkShape,
};

/// Builds the geometry of a bar mark.
pub(crate) fn sv_bar_geometry<'data>(
    options: &SVBarOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVMarkPoint],
) -> Result<SVMarkGeometry<'data>, SVError> {
    if points.is_empty() {
        return Ok(SVMarkGeometry::default());
    }

    let plot_area = request.plot_area();
    let count = points.len() as f64;
    let slot = match options.orientation_value() {
        SVBarOrientation::Vertical => plot_area.width / count,
        SVBarOrientation::Horizontal => plot_area.height / count,
    };
    let width = slot * options.width_value() * (1.0 - f64::from(options.group_gap_value()));
    let corner_radius = sv_resolve_length(Some(options.corner_radius_value()), 0.0, plot_area);
    let baseline = sv_baseline(options, request)?;
    let theme = request.theme();
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), 1.0);
    let paint = SVMarkPaint::filled(colour).with_opacity(opacity);

    let primitives = points
        .iter()
        .copied()
        .map(|point| {
            let rectangle = sv_bar_rectangle(options.orientation_value(), point, baseline, width);
            SVMarkPrimitive::new(
                SVMarkShape::Rectangle(SVGeometryRectangle::new(rectangle, corner_radius)),
                paint.clone(),
            )
        })
        .collect();

    Ok(SVMarkGeometry::new(primitives))
}

/// Places the bar baseline on the axis the bar grows along.
fn sv_baseline(options: &SVBarOptions, request: &SVMarkRequest<'_, '_>) -> Result<f64, SVError> {
    let value = options.baseline_value();
    match options.orientation_value() {
        SVBarOrientation::Vertical => sv_scale_position(request.vertical_scale(), value),
        SVBarOrientation::Horizontal => sv_scale_position(request.horizontal_scale(), value),
    }
}

/// Builds one bar rectangle.
fn sv_bar_rectangle(
    orientation: SVBarOrientation,
    point: SVMarkPoint,
    baseline: f64,
    width: f64,
) -> SVResolvedRectangle {
    match orientation {
        SVBarOrientation::Vertical => SVResolvedRectangle::new(
            point.x() - width / 2.0,
            point.y().min(baseline),
            width,
            (point.y() - baseline).abs(),
        ),
        SVBarOrientation::Horizontal => SVResolvedRectangle::new(
            point.x().min(baseline),
            point.y() - width / 2.0,
            (point.x() - baseline).abs(),
            width,
        ),
    }
}
