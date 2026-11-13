//! The optional sample points inside a violin.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{
    SVBarOrientation, SVDotShape, SVMark, SVMarkGeometry, SVMarkRequest,
};
use scienceview_core::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

use super::super::configuration::SVViolinPlotConfiguration;
use super::super::scales::SVViolinScales;
use super::super::statistics::SVViolinSummary;
use super::sv_own_geometry;

const JITTER_AMOUNT: f32 = 0.12;
const JITTER_SEED: u64 = 0x05EE_D117;

pub(super) fn sv_push_points(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVViolinSummary,
    scales: &SVViolinScales<'_>,
    configuration: &SVViolinPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let values = summary.observations();
    let keys = vec![summary.category_position().to_bits() as u32; values.len()];
    let sequence = SVMoveSequence::single(
        SVMove::jitter()
            .amount(JITTER_AMOUNT)
            .seed(JITTER_SEED)
            .into(),
    )?;
    let moved = sequence.apply(&SVMoveRequest::new(SVColumnView::Dense(values), &keys))?;
    let positions: Vec<f64> = (0..values.len())
        .map(|index| summary.category_position() + moved.horizontal_offset(index).unwrap_or(0.0))
        .collect();
    let (primary, secondary, horizontal, vertical) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            positions.as_slice(),
            values,
            &scales.category,
            &scales.value,
        ),
        SVBarOrientation::Horizontal => (
            values,
            positions.as_slice(),
            &scales.value,
            &scales.category,
        ),
    };
    let request = SVMarkRequest::new(
        SVColumnView::Dense(primary),
        SVColumnView::Dense(secondary),
        horizontal,
        vertical,
        plot,
    );
    let dot: SVMark = SVMark::dot()
        .shape(SVDotShape::Circle)
        .colour(summary.colour())
        .into();
    geometries.push(sv_own_geometry(dot.geometry(&request)?, false)?);
    Ok(())
}
