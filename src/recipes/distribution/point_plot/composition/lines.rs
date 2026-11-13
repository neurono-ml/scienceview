//! The joining-line geometries of one point plot.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{SVBarOrientation, SVMark, SVMarkGeometry, SVMarkRequest};
use scienceview_core::sv_scale::sv_default_scale_palette;

use super::ownership::sv_own_geometry;
use crate::sv_recipes::sv_distribution::sv_point_plot::configuration::SVPointPlotConfiguration;
use crate::sv_recipes::sv_distribution::sv_point_plot::scales::SVPointScales;
use crate::sv_recipes::sv_distribution::sv_point_plot::statistics::{
    SVPointEstimate, SVPointSummary,
};

pub(super) fn sv_push_lines(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVPointSummary,
    configuration: &SVPointPlotConfiguration<'_>,
    scales: &SVPointScales<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let palette = sv_default_scale_palette();
    for series_index in 0..summary.series().len() {
        let mut points: Vec<&SVPointEstimate> = summary
            .estimates()
            .iter()
            .filter(|estimate| estimate.series_index() == series_index)
            .collect();
        points.sort_by_key(|estimate| estimate.category_index());
        if points.len() < 2 {
            continue;
        }
        let positions: Vec<f64> = points.iter().map(|estimate| estimate.position()).collect();
        let values: Vec<f64> = points.iter().map(|estimate| estimate.value()).collect();
        let (primary, secondary, horizontal, vertical) = match configuration.orientation {
            SVBarOrientation::Vertical => (
                positions.as_slice(),
                values.as_slice(),
                &scales.category,
                &scales.value,
            ),
            SVBarOrientation::Horizontal => (
                values.as_slice(),
                positions.as_slice(),
                &scales.value,
                &scales.category,
            ),
        };
        let colour = configuration
            .colour
            .unwrap_or_else(|| palette.colour_at_index(series_index));
        let mut options = SVMark::line().colour(colour);
        if let Some(width) = configuration.line_width {
            options = options.width(width);
        }
        if let Some(dash) = &configuration.dash {
            options = options.dash(dash.clone());
        }
        let mark: SVMark = options.into();
        let request = SVMarkRequest::new(
            SVColumnView::Dense(primary),
            SVColumnView::Dense(secondary),
            horizontal,
            vertical,
            plot,
        );
        geometries.push(sv_own_geometry(mark.geometry(&request)?)?);
    }
    Ok(())
}
