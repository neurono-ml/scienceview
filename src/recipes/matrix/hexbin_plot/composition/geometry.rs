//! Turning one accumulated hexagon into a backend-agnostic path.

use super::lattice::SVHexbinDomain;
use crate::sv_error::SVError;
use crate::sv_mark::{
    SVGeometryPath, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive, SVMarkShape,
    SVPathCommand,
};
use crate::sv_scale::{SVScale, SVScaleValue};

/// Builds one filled hexagon path for every cell, in draw order.
pub(super) fn sv_build_geometry(
    cells: &[super::definition::SVHexbinCell],
    domain: SVHexbinDomain,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    orientation: f64,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let mut primitives = Vec::with_capacity(cells.len());
    for cell in cells {
        let path = sv_cell_path(
            cell.centre_x,
            cell.centre_y,
            domain.radius,
            orientation,
            horizontal,
            vertical,
        )?;
        primitives.push(SVMarkPrimitive::new(
            SVMarkShape::Path(path),
            SVMarkPaint::filled(cell.colour),
        ));
    }
    if primitives.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![SVMarkGeometry::new(primitives)])
}

fn sv_cell_path(
    centre_x: f64,
    centre_y: f64,
    radius: f64,
    orientation: f64,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVGeometryPath, SVError> {
    let mut commands = Vec::with_capacity(7);
    for corner in 0..6 {
        let angle = -std::f64::consts::FRAC_PI_2
            + std::f64::consts::TAU * corner as f64 / 6.0
            + orientation;
        let x = centre_x + radius * angle.cos();
        let y = centre_y + radius * angle.sin();
        let point = SVMarkPoint::new(
            horizontal.position(SVScaleValue::Numeric(x))?,
            vertical.position(SVScaleValue::Numeric(y))?,
        );
        if corner == 0 {
            commands.push(SVPathCommand::MoveTo(point));
        } else {
            commands.push(SVPathCommand::LineTo(point));
        }
    }
    commands.push(SVPathCommand::Close);
    Ok(SVGeometryPath::new(commands))
}
