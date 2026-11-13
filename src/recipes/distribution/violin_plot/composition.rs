//! Composing the violin silhouettes, inner boxes and sample points.
//!
//! The silhouette is the `Area` mark of [`sv_mark`](scienceview_core::sv_mark):
//! its baseline is the category centre and the density is scaled to the
//! category half-width by the recipe. The inner box and the sample points use
//! the `Line` and `Dot` marks.

use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::SVMarkGeometry;

use super::configuration::SVViolinPlotConfiguration;
use super::scales::SVViolinScales;
use super::statistics::SVViolinSummaries;

mod area;
mod inner_box;
mod ownership;
mod points;

pub(crate) use ownership::sv_own_geometry;

/// Builds every mark geometry of one violin plot.
pub(crate) fn sv_compose_violin_geometries(
    summaries: &SVViolinSummaries,
    scales: &SVViolinScales<'_>,
    configuration: &SVViolinPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<Vec<SVMarkGeometry<'static>>, SVError> {
    let mut geometries = Vec::new();
    for summary in &summaries.summaries {
        area::sv_push_violin(&mut geometries, summary, scales, configuration, plot)?;
        if configuration.show_box {
            inner_box::sv_push_inner_box(&mut geometries, summary, scales, configuration, plot)?;
        }
        if configuration.show_points {
            points::sv_push_points(&mut geometries, summary, scales, configuration, plot)?;
        }
    }
    Ok(geometries)
}
