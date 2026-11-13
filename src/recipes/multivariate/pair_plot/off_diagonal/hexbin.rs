//! Hexagonal binning inside one pair cell.

use super::super::configuration::SVPairConfiguration;
use super::super::preparation::{SVPairCellPoints, SVPairDomain, SVPairGroups};
use super::sv_group_points;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVGeometryPath, SVMarkGeometry, SVMarkPaint, SVMarkPoint, SVMarkPrimitive, SVMarkShape,
    SVPathCommand,
};
use crate::sv_scale::{
    SVContinuousColourScaleBuilder, SVScale, SVScaleValue, sv_default_colour_map,
};
use std::collections::HashMap;
use std::f64::consts::{FRAC_PI_2, TAU};

const SV_PAIR_HEXBIN_GRID_SIZE: f64 = 10.0;

pub(super) fn sv_hexbin_cell(
    configuration: &SVPairConfiguration<'_>,
    points: &SVPairCellPoints,
    cell: SVResolvedRectangle,
    horizontal_domain: SVPairDomain,
    vertical_domain: SVPairDomain,
    groups: &SVPairGroups,
) -> Result<(Vec<SVMarkGeometry<'static>>, usize), SVError> {
    let _ = configuration;
    let horizontal = super::super::diagonal::sv_scale(horizontal_domain, cell.x, cell.right())?;
    let vertical = super::super::diagonal::sv_scale(vertical_domain, cell.bottom(), cell.y)?;
    let mut geometries = Vec::with_capacity(groups.groups.len());
    let mut number_of_marks = 0;

    for group_index in 0..groups.groups.len() {
        let (xs, ys) = sv_group_points(points, group_index);
        if xs.is_empty() {
            continue;
        }
        let (centres, counts) = sv_lattice(&xs, &ys, horizontal_domain);
        let highest = counts.iter().copied().fold(1.0_f64, f64::max).max(2.0);
        let colour_scale = SVContinuousColourScaleBuilder::new()
            .domain(1.0, highest)
            .colour_map(sv_default_colour_map())
            .build()?;
        let radius = sv_radius(horizontal_domain);
        let mut primitives = Vec::with_capacity(centres.len());
        for (centre, count) in centres.iter().zip(counts.iter()) {
            let colour = colour_scale.colour(*count)?;
            primitives.push(SVMarkPrimitive::new(
                SVMarkShape::Path(sv_hexagon_path(
                    centre.0,
                    centre.1,
                    radius,
                    &horizontal,
                    &vertical,
                )?),
                SVMarkPaint::filled(colour).with_opacity(0.9),
            ));
        }
        number_of_marks += counts.len();
        geometries.push(SVMarkGeometry::new(primitives));
    }
    Ok((geometries, number_of_marks))
}

fn sv_radius(domain: SVPairDomain) -> f64 {
    (domain.upper - domain.lower) / (3.0_f64.sqrt() * SV_PAIR_HEXBIN_GRID_SIZE)
}

fn sv_lattice(xs: &[f64], ys: &[f64], domain: SVPairDomain) -> (Vec<(f64, f64)>, Vec<f64>) {
    let radius = sv_radius(domain).max(f64::MIN_POSITIVE);
    let centre = (domain.lower + domain.upper) / 2.0;
    let mut index: HashMap<(i64, i64), usize> = HashMap::new();
    let mut centres: Vec<(f64, f64)> = Vec::new();
    let mut counts: Vec<f64> = Vec::new();

    for (x, y) in xs.iter().zip(ys.iter()) {
        let column_fraction = (3.0_f64.sqrt() / 3.0) * (x - centre) / radius;
        let row_fraction = (2.0 / 3.0) * (y - centre) / radius;
        let (column, row) = sv_round_axial(column_fraction, row_fraction);
        let position = match index.get(&(column, row)).copied() {
            Some(position) => position,
            None => {
                let horizontal = radius * 3.0_f64.sqrt() * (column as f64 + row as f64 / 2.0);
                let vertical = radius * 1.5 * row as f64;
                centres.push((centre + horizontal, centre + vertical));
                counts.push(0.0);
                let position = centres.len() - 1;
                index.insert((column, row), position);
                position
            }
        };
        counts[position] += 1.0;
    }
    (centres, counts)
}

fn sv_round_axial(column: f64, row: f64) -> (i64, i64) {
    let x = column;
    let z = row;
    let y = -x - z;
    let mut rounded_x = x.round();
    let rounded_y = y.round();
    let mut rounded_z = z.round();
    let difference_x = (rounded_x - x).abs();
    let difference_y = (rounded_y - y).abs();
    let difference_z = (rounded_z - z).abs();
    if difference_x > difference_y && difference_x > difference_z {
        rounded_x = -rounded_y - rounded_z;
    } else if difference_z > difference_y && difference_z > difference_x {
        rounded_z = -rounded_x - rounded_y;
    }
    (rounded_x as i64, rounded_z as i64)
}

fn sv_hexagon_path(
    centre_x: f64,
    centre_y: f64,
    radius: f64,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
) -> Result<SVGeometryPath, SVError> {
    let mut commands = Vec::with_capacity(7);
    for corner in 0..6 {
        let angle = -FRAC_PI_2 + TAU * corner as f64 / 6.0;
        let point = SVMarkPoint::new(
            horizontal.position(SVScaleValue::Numeric(centre_x + radius * angle.cos()))?,
            vertical.position(SVScaleValue::Numeric(centre_y + radius * angle.sin()))?,
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
