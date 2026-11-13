//! The optional inner box, quartile edges and median line.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{SVBarOrientation, SVMark, SVMarkGeometry, SVMarkRequest};
use scienceview_core::sv_scale::SVColour;

use super::super::configuration::SVViolinPlotConfiguration;
use super::super::scales::SVViolinScales;
use super::super::statistics::SVViolinSummary;
use super::sv_own_geometry;

const BOX_WIDTH_FRACTION: f64 = 0.35;

pub(super) fn sv_push_inner_box(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVViolinSummary,
    scales: &SVViolinScales<'_>,
    configuration: &SVViolinPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let centre = summary.category_position();
    let half = (summary.maximum_half_width() * BOX_WIDTH_FRACTION).max(0.01);
    let colour = SVColour::new(40, 40, 40, 255);
    let segments = [
        (
            (centre - half, summary.lower_quartile()),
            (centre + half, summary.lower_quartile()),
        ),
        (
            (centre + half, summary.lower_quartile()),
            (centre + half, summary.upper_quartile()),
        ),
        (
            (centre + half, summary.upper_quartile()),
            (centre - half, summary.upper_quartile()),
        ),
        (
            (centre - half, summary.upper_quartile()),
            (centre - half, summary.lower_quartile()),
        ),
        (
            (centre - half, summary.median()),
            (centre + half, summary.median()),
        ),
    ];
    for (from, to) in segments {
        sv_push_segment(
            geometries,
            configuration.orientation,
            from,
            to,
            scales,
            plot,
            colour,
        )?;
    }
    Ok(())
}

fn sv_push_segment(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    orientation: SVBarOrientation,
    from: (f64, f64),
    to: (f64, f64),
    scales: &SVViolinScales<'_>,
    plot: SVResolvedRectangle,
    colour: SVColour,
) -> Result<(), SVError> {
    let (primary, secondary, horizontal, vertical) = match orientation {
        SVBarOrientation::Vertical => (
            [from.0, to.0],
            [from.1, to.1],
            &scales.category,
            &scales.value,
        ),
        SVBarOrientation::Horizontal => (
            [from.1, to.1],
            [from.0, to.0],
            &scales.value,
            &scales.category,
        ),
    };
    let request = SVMarkRequest::new(
        SVColumnView::Dense(&primary),
        SVColumnView::Dense(&secondary),
        horizontal,
        vertical,
        plot,
    );
    let line: SVMark = SVMark::line().colour(colour).into();
    geometries.push(sv_own_geometry(line.geometry(&request)?, false)?);
    Ok(())
}
