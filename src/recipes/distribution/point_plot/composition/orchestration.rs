//! Orchestration of one point-plot composition.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::{SVBarOrientation, SVMarkGeometry};
use scienceview_core::sv_scale::sv_default_scale_palette;

use super::lines::sv_push_lines;
use super::points::{sv_push_dot, sv_push_error};
use crate::sv_recipes::sv_distribution::sv_point_plot::configuration::SVPointPlotConfiguration;
use crate::sv_recipes::sv_distribution::sv_point_plot::placement::SV_POINT_DODGE_WIDTH;
use crate::sv_recipes::sv_distribution::sv_point_plot::scales::SVPointScales;
use crate::sv_recipes::sv_distribution::sv_point_plot::statistics::SVPointSummary;

/// Builds every mark geometry of one point plot.
pub(crate) fn sv_compose_geometries(
    summary: &SVPointSummary,
    scales: &SVPointScales<'_>,
    configuration: &SVPointPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let palette = sv_default_scale_palette();
    let category_count = summary.categories().len().max(1) as f64;
    let slot = match configuration.orientation {
        SVBarOrientation::Vertical => plot.width / category_count,
        SVBarOrientation::Horizontal => plot.height / category_count,
    };
    let cap_size = if configuration.caps {
        slot * f64::from(SV_POINT_DODGE_WIDTH) * f64::from(configuration.cap_width)
    } else {
        0.0
    };

    let mut geometries = Vec::new();
    for estimate in summary.estimates() {
        let colour = configuration
            .colour
            .unwrap_or_else(|| palette.colour_at_index(estimate.series_index()));
        sv_push_dot(
            &mut geometries,
            estimate,
            colour,
            configuration,
            scales,
            plot,
        )?;
        if let Some(error) = estimate.error() {
            sv_push_error(
                &mut geometries,
                estimate,
                error.lower(),
                error.upper(),
                colour,
                cap_size,
                configuration,
                scales,
                plot,
            )?;
        }
    }
    if configuration.join {
        sv_push_lines(&mut geometries, summary, configuration, scales, plot)?;
    }
    Ok(geometries)
}
