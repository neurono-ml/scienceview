//! The off-diagonal cells: scatter, hexbin and binned density contour.
//!
//! Every shape is built from the closed kernel vocabulary: points go through
//! `SVMark::Dot`, the histogram and density grids through `SVStat::Bin`, and
//! the hexagon and contour paths are chart-owned geometry placed through the
//! scale kernel. The hexbin lattice mirrors the lattice `SVHexbinPlot` owns,
//! and the contour is a binned contour rather than the KDE-based
//! `SVDensityContourPlot`; ADR #182 records why the cells own this geometry
//! instead of delegating it (the composed recipes solve their own figure and
//! cannot preserve the pair grid's shared per-variable domain).

mod contour;
mod hexbin;

use super::configuration::SVPairConfiguration;
use super::preparation::{SVPairCellPoints, SVPairDomain, SVPairGroups};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{SVMark, SVMarkGeometry, SVMarkRequest};

/// Builds the geometries of one off-diagonal cell.
pub(super) fn sv_off_diagonal_geometries(
    configuration: &SVPairConfiguration<'_>,
    points: &SVPairCellPoints,
    cell: SVResolvedRectangle,
    horizontal_domain: SVPairDomain,
    vertical_domain: SVPairDomain,
    groups: &SVPairGroups,
) -> Result<(Vec<SVMarkGeometry<'static>>, usize), SVError> {
    let displaced;
    let points = match configuration.jitter.as_ref() {
        Some(jitter) => {
            displaced = sv_displaced_points(points, jitter)?;
            &displaced
        }
        None => points,
    };
    match configuration.off_diagonal_kind {
        super::SVPairOffDiagonalKind::Scatter => sv_scatter_cell(
            configuration,
            points,
            cell,
            horizontal_domain,
            vertical_domain,
            groups,
        ),
        super::SVPairOffDiagonalKind::Hexbin => hexbin::sv_hexbin_cell(
            configuration,
            points,
            cell,
            horizontal_domain,
            vertical_domain,
            groups,
        ),
        super::SVPairOffDiagonalKind::DensityContour => contour::sv_contour_cell(
            configuration,
            points,
            cell,
            horizontal_domain,
            vertical_domain,
            groups,
        ),
    }
}

/// Applies the configured displacement move to the horizontal channel.
fn sv_displaced_points(
    points: &SVPairCellPoints,
    jitter: &crate::sv_move::SVMove,
) -> Result<SVPairCellPoints, SVError> {
    if points.horizontal.is_empty() {
        return Ok(points.clone());
    }
    let keys: Vec<u32> = points
        .groups
        .iter()
        .map(|group| u32::try_from(*group).unwrap_or(u32::MAX))
        .collect();
    let sequence = crate::sv_move::SVMoveSequence::new(vec![jitter.clone()])?;
    let moved = sequence.apply(&crate::sv_move::SVMoveRequest::new(
        SVColumnView::Dense(&points.horizontal),
        &keys,
    ))?;
    let horizontal = points
        .horizontal
        .iter()
        .enumerate()
        .map(|(index, value)| value + moved.horizontal_offset(index).unwrap_or(0.0))
        .collect();
    Ok(SVPairCellPoints {
        horizontal,
        vertical: points.vertical.clone(),
        groups: points.groups.clone(),
    })
}

fn sv_scatter_cell(
    configuration: &SVPairConfiguration<'_>,
    points: &SVPairCellPoints,
    cell: SVResolvedRectangle,
    horizontal_domain: SVPairDomain,
    vertical_domain: SVPairDomain,
    groups: &SVPairGroups,
) -> Result<(Vec<SVMarkGeometry<'static>>, usize), SVError> {
    let mut geometries = Vec::with_capacity(groups.groups.len());
    let mut number_of_marks = 0;
    for (group_index, group) in groups.groups.iter().enumerate() {
        let (horizontal, vertical) = sv_group_points(points, group_index);
        if horizontal.is_empty() {
            continue;
        }
        let horizontal_scale = super::diagonal::sv_scale(horizontal_domain, cell.x, cell.right())?;
        let vertical_scale = super::diagonal::sv_scale(vertical_domain, cell.bottom(), cell.y)?;
        let mark: SVMark = SVMark::dot()
            .shape(configuration.marker)
            .size(SVLength::absolute(5.0))
            .colour(group.colour)
            .opacity(configuration.alpha)
            .into();
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&horizontal),
            SVColumnView::Dense(&vertical),
            &horizontal_scale,
            &vertical_scale,
            cell,
        );
        let geometry = mark.geometry(&request)?;
        let primitives = geometry
            .primitives()
            .iter()
            .map(super::diagonal::sv_own_primitive)
            .collect();
        number_of_marks += horizontal.len();
        geometries.push(SVMarkGeometry::new(primitives));
    }
    Ok((geometries, number_of_marks))
}

/// Splits the finite points of one group into two vectors.
pub(super) fn sv_group_points(
    points: &SVPairCellPoints,
    group_index: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();
    for (index, group) in points.groups.iter().enumerate() {
        if *group != group_index {
            continue;
        }
        horizontal.push(points.horizontal[index]);
        vertical.push(points.vertical[index]);
    }
    (horizontal, vertical)
}
