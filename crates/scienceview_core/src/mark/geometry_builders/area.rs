//! Area geometry: a closed region between a polyline and its baseline.

use super::paths::sv_closed_path;
use super::positions::sv_scale_position;
use crate::sv_error::SVError;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_opacity};
use crate::sv_mark::{
    SVAreaOptions, SVBaselineMode, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive,
    SVMarkRequest, SVMarkShape,
};
use crate::sv_scale::SVScale;

/// Places the baseline an area closes against.
pub(crate) fn sv_baseline_position(
    scale: &SVScale<'_>,
    baseline: SVBaselineMode,
) -> Result<f64, SVError> {
    let value = match baseline {
        SVBaselineMode::Zero => 0.0,
        SVBaselineMode::Minimum => scale.domain().lower(),
        SVBaselineMode::Maximum => scale.domain().upper(),
        SVBaselineMode::Value(value) => value,
    };

    sv_scale_position(scale, value)
}

/// Builds the geometry of an area mark.
pub(crate) fn sv_area_geometry<'data>(
    options: &SVAreaOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVMarkPoint],
) -> Result<SVMarkGeometry<'data>, SVError> {
    if points.is_empty() {
        return Ok(SVMarkGeometry::default());
    }

    let baseline = sv_baseline_position(request.vertical_scale(), options.baseline_value())?;
    let mut vertices: Vec<SVMarkPoint> = points.to_vec();
    vertices.extend(
        points
            .iter()
            .rev()
            .map(|point| SVMarkPoint::new(point.x(), baseline)),
    );

    let theme = request.theme();
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), theme.area_opacity());
    let paint = SVMarkPaint::filled(colour).with_opacity(opacity);
    let primitive = SVMarkPrimitive::new(SVMarkShape::Path(sv_closed_path(&vertices)), paint);

    Ok(SVMarkGeometry::new(vec![primitive]))
}
