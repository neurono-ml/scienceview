//! One density-contour cell: the grid, the filled cells and the contour lines.

use super::super::super::configuration::SVPairConfiguration;
use super::super::super::preparation::{SVPairCellPoints, SVPairDomain, SVPairGroups};
use super::super::sv_group_points;
use super::geometry::{sv_contour_primitives, sv_filled_cell};
use super::grid::sv_joint_grid;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVContinuousColourScaleBuilder, sv_default_colour_map};

pub(crate) fn sv_contour_cell(
    _configuration: &SVPairConfiguration<'_>,
    points: &SVPairCellPoints,
    cell: SVResolvedRectangle,
    horizontal_domain: SVPairDomain,
    vertical_domain: SVPairDomain,
    groups: &SVPairGroups,
) -> Result<(Vec<SVMarkGeometry<'static>>, usize), SVError> {
    let horizontal =
        super::super::super::diagonal::sv_scale(horizontal_domain, cell.x, cell.right())?;
    let vertical = super::super::super::diagonal::sv_scale(vertical_domain, cell.bottom(), cell.y)?;
    let mut geometries = Vec::with_capacity(groups.groups.len());
    let mut number_of_marks = 0;

    for group_index in 0..groups.groups.len() {
        let (xs, ys) = sv_group_points(points, group_index);
        if xs.is_empty() {
            continue;
        }
        let (x_edges, y_edges, counts) = sv_joint_grid(&xs, &ys)?;
        let highest = counts
            .iter()
            .flat_map(|row| row.iter().copied())
            .fold(0.0_f64, f64::max)
            .max(1.0);
        let colour_scale = SVContinuousColourScaleBuilder::new()
            .domain(0.0, highest.max(2.0))
            .colour_map(sv_default_colour_map())
            .build()?;
        let mut primitives = Vec::new();
        for (row, values) in counts.iter().enumerate() {
            for (column, count) in values.iter().enumerate() {
                if *count <= 0.0 {
                    continue;
                }
                primitives.push(sv_filled_cell(
                    row,
                    column,
                    &x_edges,
                    &y_edges,
                    &horizontal,
                    &vertical,
                    colour_scale.colour(*count)?,
                )?);
            }
        }
        let threshold = (highest * 0.5).max(0.5);
        primitives.extend(sv_contour_primitives(
            &counts,
            threshold,
            &x_edges,
            &y_edges,
            &horizontal,
            &vertical,
        )?);
        number_of_marks += primitives.len();
        geometries.push(SVMarkGeometry::new(primitives));
    }
    Ok((geometries, number_of_marks))
}
