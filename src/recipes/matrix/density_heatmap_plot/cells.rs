//! Turning the binned grid into one rectangle per cell.

use super::colour_scale::SVHeatmapColourScale;
use super::configuration::SVDensityHeatmapConfiguration;
use super::grid::SVDensityHeatmapGrid;
use super::options::SVEmptyBinPolicy;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVGeometryRectangle, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive, SVMarkShape,
};
use crate::sv_scale::{SVScale, SVScaleValue};

/// Builds one mark geometry per series, each holding one rectangle per cell.
pub(crate) fn sv_build_cells(
    configuration: &SVDensityHeatmapConfiguration<'_>,
    grids: &[SVDensityHeatmapGrid],
    colour_scale: &SVHeatmapColourScale,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let lowest_colour = colour_scale.lowest_colour()?;
    let mut geometries = Vec::with_capacity(grids.len());
    for grid in grids {
        let mut primitives = Vec::new();
        for y_index in 0..grid.y_cell_count() {
            for x_index in 0..grid.x_cell_count() {
                let value = grid.value_at(x_index, y_index);
                let empty = grid.is_empty_at(x_index, y_index);
                if empty && configuration.empty_bin_policy == SVEmptyBinPolicy::Transparent {
                    continue;
                }
                let fill = if empty {
                    lowest_colour
                } else {
                    colour_scale.colour(value)?
                };
                let rectangle = sv_cell_rectangle(grid, x_index, y_index, horizontal, vertical)?;
                let paint = SVMarkPaint::filled(fill).with_opacity(configuration.opacity);
                primitives.push(SVMarkPrimitive::new(
                    SVMarkShape::Rectangle(SVGeometryRectangle::new(rectangle, 0.0)),
                    paint,
                ));
            }
        }
        geometries.push(SVMarkGeometry::new(primitives));
    }
    Ok(geometries)
}

fn sv_cell_rectangle(
    grid: &SVDensityHeatmapGrid,
    x_index: usize,
    y_index: usize,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVResolvedRectangle, SVError> {
    let left = horizontal.position(SVScaleValue::Numeric(grid.x_edges()[x_index]))?;
    let right = horizontal.position(SVScaleValue::Numeric(grid.x_edges()[x_index + 1]))?;
    let top = vertical.position(SVScaleValue::Numeric(grid.y_edges()[y_index + 1]))?;
    let bottom = vertical.position(SVScaleValue::Numeric(grid.y_edges()[y_index]))?;
    Ok(SVResolvedRectangle::new(
        left,
        top,
        right - left,
        bottom - top,
    ))
}
