//! The diagonal cells: the distribution of one variable per hue group.

use super::SVPairDiagonalKind;
use super::configuration::SVPairConfiguration;
use super::preparation::{SVPairDomain, SVPairGroups, sv_group_values};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle};
use crate::sv_mark::{SVMark, SVMarkGeometry, SVMarkRequest};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};
use crate::sv_stat::{
    SVBinOptions, SVKernelDensityOptions, SVNormalisation, SVStat, SVStatInput, SVStatResult,
};

/// Builds the geometries of one diagonal cell.
pub(super) fn sv_diagonal_geometries(
    configuration: &SVPairConfiguration<'_>,
    variable: &str,
    cell: SVResolvedRectangle,
    domain: SVPairDomain,
    groups: &SVPairGroups,
) -> Result<(Vec<SVMarkGeometry<'static>>, usize), SVError> {
    let kind = configuration.resolved_diagonal_kind();
    let mut geometries = Vec::with_capacity(groups.groups.len());
    let mut number_of_marks = 0;
    for (group_index, group) in groups.groups.iter().enumerate() {
        let values = sv_group_values(configuration, variable, group_index, groups)?;
        if values.is_empty() {
            continue;
        }
        let (geometry, count) = match kind {
            SVPairDiagonalKind::Histogram | SVPairDiagonalKind::Auto => {
                sv_histogram_cell(&values, cell, domain, group.colour)?
            }
            SVPairDiagonalKind::Density => sv_density_cell(&values, cell, domain, group.colour)?,
        };
        number_of_marks += count;
        geometries.push(SVMarkGeometry::new(geometry));
    }
    Ok((geometries, number_of_marks))
}

fn sv_histogram_cell(
    values: &[f64],
    cell: SVResolvedRectangle,
    domain: SVPairDomain,
    colour: crate::sv_scale::SVColour,
) -> Result<(Vec<crate::sv_mark::SVMarkPrimitive<'static>>, usize), SVError> {
    let stat = SVStat::bin(
        SVBinOptions::new()
            .bin_count(12)
            .normalise(SVNormalisation::Count),
    );
    let result = stat.apply(SVStatInput::new(SVColumnView::Dense(values)))?;
    let SVStatResult::Bins(bins) = result else {
        return Err(SVError::InvalidConfiguration {
            message: "the histogram diagonal did not produce bins".to_string(),
        });
    };
    let horizontal = sv_scale(domain, cell.x, cell.right())?;
    let highest = bins
        .counts()
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, highest * 1.08)
        .range(cell.bottom(), cell.y)
        .build()?;

    let mut primitives = Vec::new();
    let mut number_of_marks = 0;
    for index in 0..bins.counts().len() {
        let (Some(left), Some(right)) = (bins.edges().get(index), bins.edges().get(index + 1))
        else {
            continue;
        };
        let top = bins.counts().get(index).copied().unwrap_or(0.0);
        if top <= 0.0 {
            continue;
        }
        let slot = (horizontal.position(crate::sv_scale::SVScaleValue::Numeric(*right))?
            - horizontal.position(crate::sv_scale::SVScaleValue::Numeric(*left))?)
        .abs();
        if slot <= 0.0 {
            continue;
        }
        let area = SVResolvedRectangle::new(
            horizontal
                .position(crate::sv_scale::SVScaleValue::Numeric(*left))?
                .min(horizontal.position(crate::sv_scale::SVScaleValue::Numeric(*right))?),
            cell.y,
            slot,
            cell.height,
        );
        let mark: SVMark = SVMark::bar()
            .width(1.0)
            .baseline(0.0)
            .colour(colour)
            .opacity(0.75)
            .into();
        let clear = (left + right) / 2.0;
        let centers = [clear];
        let tops = [top];
        let request = SVMarkRequest::new(
            SVColumnView::Dense(&centers),
            SVColumnView::Dense(&tops),
            &horizontal,
            &vertical,
            area,
        );
        for primitive in mark.geometry(&request)?.primitives() {
            primitives.push(sv_own_primitive(primitive));
        }
        number_of_marks += 1;
    }
    Ok((primitives, number_of_marks))
}

fn sv_density_cell(
    values: &[f64],
    cell: SVResolvedRectangle,
    domain: SVPairDomain,
    colour: crate::sv_scale::SVColour,
) -> Result<(Vec<crate::sv_mark::SVMarkPrimitive<'static>>, usize), SVError> {
    let stat = SVStat::kernel_density(SVKernelDensityOptions::new().grid_count(64));
    let result = stat.apply(SVStatInput::new(SVColumnView::Dense(values)))?;
    let SVStatResult::Density(density) = result else {
        return Err(SVError::InvalidConfiguration {
            message: "the density diagonal did not produce a curve".to_string(),
        });
    };
    let horizontal = sv_scale(domain, cell.x, cell.right())?;
    let highest = density
        .density()
        .iter()
        .copied()
        .fold(0.0_f64, f64::max)
        .max(f64::MIN_POSITIVE);
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, highest * 1.08)
        .range(cell.bottom(), cell.y)
        .build()?;
    let mark: SVMark = SVMark::line()
        .colour(colour)
        .width(SVLength::absolute(1.8))
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(density.grid()),
        SVColumnView::Dense(density.density()),
        &horizontal,
        &vertical,
        cell,
    );
    let geometry = mark.geometry(&request)?;
    let primitives = geometry
        .primitives()
        .iter()
        .map(sv_own_primitive)
        .collect::<Vec<_>>();
    Ok((primitives, density.grid().len()))
}

pub(super) fn sv_scale(
    domain: SVPairDomain,
    start: f64,
    end: f64,
) -> Result<SVScale<'static>, SVError> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower, domain.upper)
        .range(start, end)
        .build()
}

pub(super) fn sv_own_primitive(
    primitive: &crate::sv_mark::SVMarkPrimitive<'_>,
) -> crate::sv_mark::SVMarkPrimitive<'static> {
    use crate::sv_mark::{SVGeometryImage, SVImageInterpolation, SVMarkShape};
    let shape = match primitive.shape() {
        SVMarkShape::Path(path) => SVMarkShape::Path(path.clone()),
        SVMarkShape::Rectangle(rectangle) => SVMarkShape::Rectangle(*rectangle),
        SVMarkShape::Circle(circle) => SVMarkShape::Circle(*circle),
        SVMarkShape::Text(text) => SVMarkShape::Text(text.clone()),
        SVMarkShape::Image(_) => SVMarkShape::Image(SVGeometryImage::new(
            SVResolvedRectangle::new(0.0, 0.0, 0.0, 0.0),
            0,
            0,
            &[],
            SVImageInterpolation::Nearest,
        )),
    };
    crate::sv_mark::SVMarkPrimitive::new(shape, primitive.paint().clone())
}
