//! The dot and error-bar geometries of one point plot.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVLength, SVResolvedRectangle};
use scienceview_core::sv_mark::{
    SVBarOrientation, SVErrorBarDirection, SVMark, SVMarkGeometry, SVMarkRequest,
};
use scienceview_core::sv_scale::SVColour;

use super::ownership::sv_own_geometry;
use crate::sv_recipes::sv_distribution::sv_point_plot::configuration::SVPointPlotConfiguration;
use crate::sv_recipes::sv_distribution::sv_point_plot::scales::SVPointScales;
use crate::sv_recipes::sv_distribution::sv_point_plot::statistics::SVPointEstimate;

#[allow(clippy::too_many_arguments)]
pub(super) fn sv_push_dot(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    estimate: &SVPointEstimate,
    colour: SVColour,
    configuration: &SVPointPlotConfiguration<'_>,
    scales: &SVPointScales<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let (primary, secondary, horizontal, vertical) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            [estimate.position()],
            [estimate.value()],
            &scales.category,
            &scales.value,
        ),
        SVBarOrientation::Horizontal => (
            [estimate.value()],
            [estimate.position()],
            &scales.value,
            &scales.category,
        ),
    };
    let mark: SVMark = SVMark::dot()
        .shape(configuration.marker)
        .colour(colour)
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        horizontal,
        vertical,
        plot,
    );
    geometries.push(sv_own_geometry(mark.geometry(&request)?)?);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn sv_push_error(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    estimate: &SVPointEstimate,
    lower: f64,
    upper: f64,
    colour: SVColour,
    cap_size: f64,
    configuration: &SVPointPlotConfiguration<'_>,
    scales: &SVPointScales<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let (primary, secondary, horizontal, vertical, direction) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            [estimate.position()],
            [estimate.value()],
            &scales.category,
            &scales.value,
            SVErrorBarDirection::Vertical,
        ),
        SVBarOrientation::Horizontal => (
            [estimate.value()],
            [estimate.position()],
            &scales.value,
            &scales.category,
            SVErrorBarDirection::Horizontal,
        ),
    };
    let lower = [lower];
    let upper = [upper];
    let mark: SVMark = SVMark::error_bar()
        .direction(direction)
        .cap_size(SVLength::absolute(cap_size))
        .colour(colour)
        .into();
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        horizontal,
        vertical,
        plot,
    )
    .with_lower(SVColumnView::Dense(&lower))
    .with_upper(SVColumnView::Dense(&upper));
    geometries.push(sv_own_geometry(mark.geometry(&request)?)?);
    Ok(())
}
