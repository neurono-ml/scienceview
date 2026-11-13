//! Placing the prepared ribbons through the ribbon mark.
//!
//! Every ribbon is a quadrilateral between two adjacent axes: its left edge
//! spans a slice of the left category band and its right edge spans a slice of
//! the right category band, so the slice thickness encodes the aggregated
//! count. The recipe only computes the slice boundaries; the path itself comes
//! from [`SVMark::ribbon`], which is the single place a ribbon becomes
//! geometry.

use super::configuration::SVParallelCategoriesConfiguration;
use super::preparation::{SVPreparedParallelCategories, SVPreparedRibbon};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::{
    SVGeometryImage, SVImageInterpolation, SVMark, SVMarkGeometry, SVMarkPaint, SVMarkPrimitive,
    SVMarkRequest, SVMarkShape, SVMarkTheme,
};
use crate::sv_scale::SVScale;

/// One band allocation on one axis.
struct SVBandAllocation {
    center: f64,
    height: f64,
}

/// Builds one mark geometry per aggregated ribbon.
pub(super) fn sv_compose_ribbon_geometries(
    prepared: &SVPreparedParallelCategories,
    configuration: &SVParallelCategoriesConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let bands = sv_bands(prepared, configuration, plot);
    let (left_totals, right_totals) = sv_side_totals(prepared);
    let mut left_offsets = sv_zero_offsets(prepared);
    let mut right_offsets = sv_zero_offsets(prepared);
    let theme = SVMarkTheme::default();
    let mut geometries = Vec::with_capacity(prepared.ribbons.len());

    for ribbon in &prepared.ribbons {
        let geometry = sv_ribbon_geometry(
            ribbon,
            &bands,
            &left_totals,
            &right_totals,
            &mut left_offsets,
            &mut right_offsets,
            configuration,
            horizontal,
            vertical,
            plot,
            &theme,
        )?;
        geometries.push(geometry);
    }
    Ok(geometries)
}

fn sv_bands(
    prepared: &SVPreparedParallelCategories,
    configuration: &SVParallelCategoriesConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Vec<Vec<SVBandAllocation>> {
    prepared
        .axes
        .iter()
        .map(|axis| {
            let count = axis.categories.len().max(1) as f64;
            let slot = plot.height / count;
            let height = slot * (1.0 - f64::from(configuration.band_gap));
            (0..axis.categories.len())
                .map(|index| SVBandAllocation {
                    center: plot.y + slot * (index as f64 + 0.5),
                    height,
                })
                .collect()
        })
        .collect()
}

fn sv_side_totals(prepared: &SVPreparedParallelCategories) -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
    let mut left = sv_zero_offsets(prepared);
    let mut right = sv_zero_offsets(prepared);
    for ribbon in &prepared.ribbons {
        if let Some(total) = left
            .get_mut(ribbon.left_axis)
            .and_then(|axis| axis.get_mut(ribbon.left_category))
        {
            *total += ribbon.count;
        }
        if let Some(total) = right
            .get_mut(ribbon.right_axis)
            .and_then(|axis| axis.get_mut(ribbon.right_category))
        {
            *total += ribbon.count;
        }
    }
    (left, right)
}

fn sv_zero_offsets(prepared: &SVPreparedParallelCategories) -> Vec<Vec<u64>> {
    prepared
        .axes
        .iter()
        .map(|axis| vec![0; axis.categories.len()])
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn sv_ribbon_geometry(
    ribbon: &SVPreparedRibbon,
    bands: &[Vec<SVBandAllocation>],
    left_totals: &[Vec<u64>],
    right_totals: &[Vec<u64>],
    left_offsets: &mut [Vec<u64>],
    right_offsets: &mut [Vec<u64>],
    configuration: &SVParallelCategoriesConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
    theme: &SVMarkTheme,
) -> Result<SVMarkGeometry<'static>, SVError> {
    let (top_left, bottom_left) = sv_slice(
        bands,
        left_totals,
        left_offsets,
        ribbon.left_axis,
        ribbon.left_category,
        ribbon.count,
    );
    let (top_right, bottom_right) = sv_slice(
        bands,
        right_totals,
        right_offsets,
        ribbon.right_axis,
        ribbon.right_category,
        ribbon.count,
    );

    let x = [ribbon.left_axis as f64, ribbon.right_axis as f64];
    let upper = [sv_normalise(top_left, plot), sv_normalise(top_right, plot)];
    let lower = [
        sv_normalise(bottom_left, plot),
        sv_normalise(bottom_right, plot),
    ];
    let center = [
        sv_normalise((top_left + bottom_left) / 2.0, plot),
        sv_normalise((top_right + bottom_right) / 2.0, plot),
    ];

    let options = SVMark::ribbon()
        .colour(ribbon.colour)
        .opacity(configuration.opacity);
    let mark: SVMark = options.into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&x),
        SVColumnView::Dense(&center),
        horizontal,
        vertical,
        plot,
    )
    .with_lower(SVColumnView::Dense(&lower))
    .with_upper(SVColumnView::Dense(&upper))
    .with_theme(theme.clone());
    sv_own_geometry(&mark.geometry(&request)?)
}

fn sv_slice(
    bands: &[Vec<SVBandAllocation>],
    totals: &[Vec<u64>],
    offsets: &mut [Vec<u64>],
    axis: usize,
    category: usize,
    count: u64,
) -> (f64, f64) {
    let band = bands.get(axis).and_then(|axis| axis.get(category)).map_or(
        SVBandAllocation {
            center: 0.0,
            height: 0.0,
        },
        |band| SVBandAllocation {
            center: band.center,
            height: band.height,
        },
    );
    let total = totals
        .get(axis)
        .and_then(|axis| axis.get(category))
        .copied()
        .unwrap_or(0)
        .max(1);
    let offset = offsets
        .get_mut(axis)
        .and_then(|axis| axis.get_mut(category))
        .map_or(0, |offset| *offset);
    let height = band.height * count as f64 / total as f64;
    let top = band.center - band.height / 2.0 + band.height * offset as f64 / total as f64;
    if let Some(slot) = offsets
        .get_mut(axis)
        .and_then(|axis| axis.get_mut(category))
    {
        *slot += count;
    }
    (top, top + height)
}

fn sv_normalise(pixel: f64, plot: SVResolvedRectangle) -> f64 {
    if plot.height <= 0.0 {
        return 0.0;
    }
    (plot.bottom() - pixel) / plot.height
}

/// Owns one mark geometry so the scene can outlive the composition buffers.
fn sv_own_geometry(geometry: &SVMarkGeometry<'_>) -> Result<SVMarkGeometry<'static>, SVError> {
    let mut primitives = Vec::with_capacity(geometry.len());
    for primitive in geometry.primitives() {
        let paint: SVMarkPaint = primitive.paint().clone();
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
        primitives.push(SVMarkPrimitive::new(shape, paint));
    }
    Ok(SVMarkGeometry::new(primitives))
}
