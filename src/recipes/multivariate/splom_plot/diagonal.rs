//! The univariate distribution a diagonal cell draws.
//!
//! The diagonal is the composition `column -> SVStat::Bin | SVStat::KernelDensity
//! -> [SVScale] -> SVMark::Bar | SVMark::Area`. The statistics come from the
//! closed `sv_stat` kernel and the geometry from `sv_mark`; the recipe only
//! solves the cell's two scales and composes them.

use super::SVSplomDiagonal;
use super::configuration::SVSplomConfiguration;
use super::geometry::sv_owned_primitive;
use super::preparation::SVSplomPreparedColumn;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{SVBarOrientation, SVMark, SVMarkGeometry, SVMarkRequest};
use crate::sv_scale::{SVScaleBuilder, SVScaleKind};
use crate::sv_stat::{SVBinOptions, SVKernelDensityOptions, SVStat, SVStatInput, SVStatResult};

/// The number of bins a diagonal histogram draws.
pub(super) const SV_SPLOM_DIAGONAL_BIN_COUNT: usize = 10;

/// Builds the geometry of one diagonal cell.
///
/// # Errors
///
/// Returns the error of the statistic or of the mark when the column cannot be
/// summarised, and [`SVError::InvalidConfiguration`] when the statistic
/// returned an unexpected result variant.
pub(super) fn sv_diagonal_geometry(
    configuration: &SVSplomConfiguration<'_>,
    column: &SVSplomPreparedColumn<'_>,
    domain: (f64, f64),
    cell: SVResolvedRectangle,
) -> Result<SVMarkGeometry<'static>, SVError> {
    match configuration.diagonal {
        SVSplomDiagonal::Empty => Ok(SVMarkGeometry::default()),
        SVSplomDiagonal::Histogram => sv_histogram_geometry(configuration, column, domain, cell),
        SVSplomDiagonal::Density => sv_density_geometry(configuration, column, cell),
    }
}

fn sv_histogram_geometry(
    configuration: &SVSplomConfiguration<'_>,
    column: &SVSplomPreparedColumn<'_>,
    domain: (f64, f64),
    cell: SVResolvedRectangle,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let options = SVBinOptions::new().bin_count(SV_SPLOM_DIAGONAL_BIN_COUNT);
    let stat = SVStat::bin(options);
    let result = stat.apply(SVStatInput::new(column.view()))?;
    let SVStatResult::Bins(bins) = result else {
        return Err(sv_unexpected_stat());
    };

    let edges = bins.edges();
    let mut centres = Vec::with_capacity(bins.counts().len());
    for index in 0..bins.counts().len() {
        let Some(low) = edges.get(index) else {
            continue;
        };
        let Some(high) = edges.get(index + 1) else {
            continue;
        };
        centres.push((low + high) / 2.0);
    }
    let counts = bins.counts().to_vec();
    let maximum = counts.iter().copied().fold(0.0f64, f64::max).max(1.0);

    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.0, domain.1)
        .range(cell.x, cell.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, maximum)
        .range(cell.bottom(), cell.y)
        .build()?;

    let mark: SVMark = SVMark::bar()
        .orientation(SVBarOrientation::Vertical)
        .width(0.85)
        .opacity(configuration.opacity)
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&centres),
        SVColumnView::Dense(&counts),
        &horizontal,
        &vertical,
        cell,
    );
    let geometry = mark.geometry(&request)?;
    let mut owned = Vec::with_capacity(geometry.len());
    for primitive in geometry.primitives() {
        owned.push(sv_owned_primitive(primitive)?);
    }
    Ok(SVMarkGeometry::new(owned))
}

fn sv_density_geometry(
    configuration: &SVSplomConfiguration<'_>,
    column: &SVSplomPreparedColumn<'_>,
    cell: SVResolvedRectangle,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let stat = SVStat::kernel_density(SVKernelDensityOptions::new());
    let result = stat.apply(SVStatInput::new(column.view()))?;
    let SVStatResult::Density(density) = result else {
        return Err(sv_unexpected_stat());
    };
    let grid = density.grid();
    let estimates = density.density();
    let horizontal_domain = sv_domain_of(grid);
    let vertical_domain = (0.0, sv_maximum_of(estimates));

    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(horizontal_domain.0, horizontal_domain.1)
        .range(cell.x, cell.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(vertical_domain.0, vertical_domain.1)
        .range(cell.bottom(), cell.y)
        .build()?;

    let mark: SVMark = SVMark::area().opacity(configuration.opacity).into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(grid),
        SVColumnView::Dense(estimates),
        &horizontal,
        &vertical,
        cell,
    );
    let geometry = mark.geometry(&request)?;
    let mut owned = Vec::with_capacity(geometry.len());
    for primitive in geometry.primitives() {
        owned.push(sv_owned_primitive(primitive)?);
    }
    Ok(SVMarkGeometry::new(owned))
}

fn sv_domain_of(values: &[f64]) -> (f64, f64) {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        low = low.min(value);
        high = high.max(value);
    }
    if !low.is_finite() || !high.is_finite() {
        return (0.0, 1.0);
    }
    if high - low > f64::EPSILON {
        (low, high)
    } else {
        (low - 1.0, high + 1.0)
    }
}

fn sv_maximum_of(values: &[f64]) -> f64 {
    let maximum = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold(0.0f64, f64::max);
    if maximum > 0.0 { maximum } else { 1.0 }
}

fn sv_unexpected_stat() -> SVError {
    SVError::InvalidConfiguration {
        message: "the diagonal statistic returned a result variant it does not produce".to_string(),
    }
}
