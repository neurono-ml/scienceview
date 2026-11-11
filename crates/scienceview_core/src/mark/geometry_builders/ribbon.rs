//! Ribbon geometry: the filled region between two boundary lines.

use super::paths::{sv_closed_path, sv_open_path};
use super::positions::sv_boundary_positions;
use crate::sv_error::SVError;
use crate::sv_mark::paint::{sv_resolve_colour, sv_resolve_length, sv_resolve_opacity};
use crate::sv_mark::{
    SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive, SVMarkRequest, SVMarkShape,
    SVRibbonBoundary, SVRibbonOptions,
};

/// Builds the geometry of a ribbon mark.
pub(crate) fn sv_ribbon_geometry<'data>(
    options: &SVRibbonOptions,
    request: &SVMarkRequest<'data, '_>,
    points: &[SVMarkPoint],
    expected_length: usize,
) -> Result<SVMarkGeometry<'data>, SVError> {
    let upper_column = request
        .upper()
        .ok_or_else(|| sv_missing_boundary("upper"))?;
    let lower_column = request
        .lower()
        .ok_or_else(|| sv_missing_boundary("lower"))?;
    let upper = sv_boundary_positions(
        request,
        upper_column,
        expected_length,
        "ribbon upper values",
    )?;
    let lower = sv_boundary_positions(
        request,
        lower_column,
        expected_length,
        "ribbon lower values",
    )?;
    if points.is_empty() {
        return Ok(SVMarkGeometry::default());
    }

    let theme = request.theme();
    let colour = sv_resolve_colour(options.colour_value(), theme);
    let opacity = sv_resolve_opacity(options.opacity_value(), theme.area_opacity());
    let primitive = match options.boundary_value() {
        SVRibbonBoundary::Both => {
            let mut vertices: Vec<SVMarkPoint> = points
                .iter()
                .zip(&upper)
                .map(|(point, boundary)| SVMarkPoint::new(point.x(), *boundary))
                .collect();
            vertices.extend(
                points
                    .iter()
                    .zip(&lower)
                    .rev()
                    .map(|(point, boundary)| SVMarkPoint::new(point.x(), *boundary)),
            );
            let paint = SVMarkPaint::filled(colour).with_opacity(opacity);
            SVMarkPrimitive::new(SVMarkShape::Path(sv_closed_path(&vertices)), paint)
        }
        SVRibbonBoundary::Upper | SVRibbonBoundary::Lower => {
            let boundary = if matches!(options.boundary_value(), SVRibbonBoundary::Upper) {
                &upper
            } else {
                &lower
            };
            let vertices: Vec<SVMarkPoint> = points
                .iter()
                .zip(boundary)
                .map(|(point, boundary)| SVMarkPoint::new(point.x(), *boundary))
                .collect();
            let width = sv_resolve_length(None, theme.line_width(), request.plot_area());
            let paint = SVMarkPaint::stroked(colour, width).with_opacity(opacity);
            SVMarkPrimitive::new(SVMarkShape::Path(sv_open_path(&vertices)), paint)
        }
    };

    Ok(SVMarkGeometry::new(vec![primitive]))
}

fn sv_missing_boundary(name: &str) -> SVError {
    SVError::InvalidConfiguration {
        message: format!("a ribbon needs a {name} boundary column"),
    }
}
