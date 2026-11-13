//! The mirrored density silhouette of one violin.

use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_mark::{
    SVBaselineMode, SVMark, SVMarkGeometry, SVMarkRequest, SVMarkTheme,
};

use super::super::configuration::SVViolinPlotConfiguration;
use super::super::grouping::SVViolinSide;
use super::super::scales::SVViolinScales;
use super::super::statistics::SVViolinSummary;
use super::sv_own_geometry;

pub(super) fn sv_push_violin(
    geometries: &mut Vec<SVMarkGeometry<'static>>,
    summary: &SVViolinSummary,
    scales: &SVViolinScales<'_>,
    configuration: &SVViolinPlotConfiguration<'_>,
    plot: SVResolvedRectangle,
) -> Result<(), SVError> {
    let sides: &[SVViolinSide] = match summary.side() {
        SVViolinSide::Both => &[SVViolinSide::Right, SVViolinSide::Left],
        SVViolinSide::Left => &[SVViolinSide::Left],
        SVViolinSide::Right => &[SVViolinSide::Right],
    };
    let theme = SVMarkTheme::default();
    for side in sides {
        let sign = match side {
            SVViolinSide::Left => -1.0,
            SVViolinSide::Right | SVViolinSide::Both => 1.0,
        };
        let offsets: Vec<f64> = (0..summary.density().len())
            .map(|index| summary.category_position() + sign * summary.half_width_at(index))
            .collect();
        let request = SVMarkRequest::new(
            SVColumnView::Dense(summary.grid()),
            SVColumnView::Dense(&offsets),
            &scales.value,
            &scales.category,
            plot,
        );
        let area: SVMark = SVMark::area()
            .colour(summary.colour())
            .baseline(SVBaselineMode::Value(summary.category_position()))
            .opacity(theme.area_opacity())
            .into();
        let geometry = area.geometry(&request)?;
        let transpose = configuration.orientation == SVBarOrientation::Vertical;
        geometries.push(sv_own_geometry(geometry, transpose)?);
    }
    Ok(())
}
