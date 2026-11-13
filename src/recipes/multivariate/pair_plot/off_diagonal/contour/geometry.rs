//! Turning the joint grid into filled cells and contour lines.

use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVGeometryPath, SVGeometryRectangle, SVMarkPaint, SVMarkPoint, SVMarkPrimitive, SVMarkShape,
    SVPathCommand,
};
use crate::sv_scale::{SVScale, SVScaleValue};

pub(super) fn sv_cell_rectangle(
    row: usize,
    column: usize,
    x_edges: &[f64],
    y_edges: &[f64],
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVResolvedRectangle, SVError> {
    let left = horizontal.position(SVScaleValue::Numeric(x_edges[column]))?;
    let right = horizontal.position(SVScaleValue::Numeric(x_edges[column + 1]))?;
    let bottom = vertical.position(SVScaleValue::Numeric(y_edges[row]))?;
    let top = vertical.position(SVScaleValue::Numeric(y_edges[row + 1]))?;
    Ok(SVResolvedRectangle::new(
        left.min(right),
        top.min(bottom),
        (right - left).abs(),
        (bottom - top).abs(),
    ))
}

pub(super) fn sv_filled_cell(
    row: usize,
    column: usize,
    x_edges: &[f64],
    y_edges: &[f64],
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    colour: crate::sv_scale::SVColour,
) -> Result<SVMarkPrimitive<'static>, SVError> {
    let area = sv_cell_rectangle(row, column, x_edges, y_edges, horizontal, vertical)?;
    Ok(SVMarkPrimitive::new(
        SVMarkShape::Rectangle(SVGeometryRectangle::new(area, 0.0)),
        SVMarkPaint::filled(colour).with_opacity(0.55),
    ))
}

pub(super) fn sv_contour_primitives(
    counts: &[Vec<f64>],
    threshold: f64,
    x_edges: &[f64],
    y_edges: &[f64],
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<Vec<SVMarkPrimitive<'static>>, SVError> {
    let rows = counts.len().saturating_sub(1);
    let columns = counts.first().map_or(0, Vec::len).saturating_sub(1);
    let mut commands = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            let corners = [
                (x_edges[column], y_edges[row], counts[row][column]),
                (x_edges[column + 1], y_edges[row], counts[row][column + 1]),
                (
                    x_edges[column + 1],
                    y_edges[row + 1],
                    counts[row + 1][column + 1],
                ),
                (x_edges[column], y_edges[row + 1], counts[row + 1][column]),
            ];
            let crossings: Vec<(f64, f64)> = (0..4)
                .filter_map(|index| {
                    sv_crossing(corners[index], corners[(index + 1) % 4], threshold)
                })
                .collect();
            for pair in crossings.chunks_exact(2) {
                commands.push(SVPathCommand::MoveTo(sv_point(
                    pair[0], horizontal, vertical,
                )?));
                commands.push(SVPathCommand::LineTo(sv_point(
                    pair[1], horizontal, vertical,
                )?));
            }
        }
    }
    if commands.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![SVMarkPrimitive::new(
        SVMarkShape::Path(SVGeometryPath::new(commands)),
        SVMarkPaint::stroked(crate::sv_scale::SVColour::new(35, 55, 75, 255), 1.2),
    )])
}

fn sv_point(
    point: (f64, f64),
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVMarkPoint, SVError> {
    Ok(SVMarkPoint::new(
        horizontal.position(SVScaleValue::Numeric(point.0))?,
        vertical.position(SVScaleValue::Numeric(point.1))?,
    ))
}

fn sv_crossing(
    first: (f64, f64, f64),
    second: (f64, f64, f64),
    threshold: f64,
) -> Option<(f64, f64)> {
    let (first_x, first_y, first_value) = first;
    let (second_x, second_y, second_value) = second;
    let difference = second_value - first_value;
    if difference.abs() <= f64::EPSILON {
        return None;
    }
    let fraction = (threshold - first_value) / difference;
    if !(0.0..=1.0).contains(&fraction) {
        return None;
    }
    Some((
        first_x + (second_x - first_x) * fraction,
        first_y + (second_y - first_y) * fraction,
    ))
}
