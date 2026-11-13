//! The composition pass: resolve columns, build grids, scales and decorations.

use super::super::configuration::{
    SV_DENSITY_CONTOUR_PLOT_HEIGHT, SV_DENSITY_CONTOUR_PLOT_LEFT, SV_DENSITY_CONTOUR_PLOT_TOP,
    SV_DENSITY_CONTOUR_PLOT_WIDTH, SVDensityContourConfiguration,
};
use super::super::decorations::{sv_build_decorations, sv_viewport};
use super::super::grid::{SVDensityGrid, sv_build_density_grids};
use super::super::levels::sv_select_levels;
use super::contour_geometry::sv_build_contours;
use super::definition::SVDensityContourComposition;
use crate::sv_data::{SVColumnSnapshot, SVDataSource};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

pub(crate) fn sv_compose(
    configuration: &SVDensityContourConfiguration<'_>,
) -> Result<SVDensityContourComposition, SVError> {
    let mut horizontal_snapshot = configuration.data_source.column(&configuration.x)?;
    let mut vertical_snapshot = configuration.data_source.column(&configuration.y)?;
    let mut series_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.series)?;
    let mut weight_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.weight)?;

    let horizontal = horizontal_snapshot.column_view();
    let vertical = vertical_snapshot.column_view();
    let series = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let weight = weight_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let (grids, series_labels) =
        sv_build_density_grids(configuration, horizontal, vertical, series, weight)?;
    let levels = sv_select_levels(configuration, &grids)?;
    let (horizontal_scale, vertical_scale) = sv_build_scales(&grids)?;
    let plot = SVResolvedRectangle::new(
        SV_DENSITY_CONTOUR_PLOT_LEFT,
        SV_DENSITY_CONTOUR_PLOT_TOP,
        SV_DENSITY_CONTOUR_PLOT_WIDTH,
        SV_DENSITY_CONTOUR_PLOT_HEIGHT,
    );
    let mark_geometries = sv_build_contours(
        configuration,
        &grids,
        &levels,
        &horizontal_scale,
        &vertical_scale,
        plot,
    )?;
    let number_of_drawn_marks = mark_geometries.iter().map(|geometry| geometry.len()).sum();
    let style = SVDecorationStyle::built_in();
    let (decorations_before_marks, decorations_after_marks) = sv_build_decorations(
        configuration,
        &horizontal_scale,
        &vertical_scale,
        &levels,
        plot,
        &style,
    )?;

    Ok(SVDensityContourComposition {
        backend_kind: configuration.backend_kind,
        representation: configuration.representation,
        viewport: sv_viewport(),
        plot_area: plot,
        background: style.background(),
        title: configuration.title.clone(),
        horizontal_scale,
        vertical_scale,
        colour_scale: levels.colour_scale.clone(),
        grids,
        series_labels,
        level_thresholds: levels.thresholds.clone(),
        decorations_before_marks,
        decorations_after_marks,
        mark_geometries,
        number_of_drawn_marks,
    })
}

fn sv_optional_snapshot<'source>(
    data_source: &'source dyn SVDataSource,
    name: &Option<String>,
) -> Result<Option<SVColumnSnapshot<'source>>, SVError> {
    match name {
        Some(name) => Ok(Some(data_source.column(name)?)),
        None => Ok(None),
    }
}

fn sv_build_scales(
    grids: &[SVDensityGrid],
) -> Result<(SVScale<'static>, SVScale<'static>), SVError> {
    let Some(grid) = grids.first() else {
        return Err(SVError::EmptyInput {
            operation: "density_contour",
        });
    };
    let horizontal_domain = sv_domain(grid.x_edges());
    let vertical_domain = sv_domain(grid.y_edges());
    let plot = SVResolvedRectangle::new(
        SV_DENSITY_CONTOUR_PLOT_LEFT,
        SV_DENSITY_CONTOUR_PLOT_TOP,
        SV_DENSITY_CONTOUR_PLOT_WIDTH,
        SV_DENSITY_CONTOUR_PLOT_HEIGHT,
    );
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(horizontal_domain.0, horizontal_domain.1)
        .range(plot.x, plot.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(vertical_domain.0, vertical_domain.1)
        .range(plot.bottom(), plot.y)
        .build()?;
    Ok((horizontal, vertical))
}

fn sv_domain(edges: &[f64]) -> (f64, f64) {
    let lower = edges.first().copied().unwrap_or(0.0);
    let upper = edges.last().copied().unwrap_or(1.0);
    if upper > lower {
        (lower, upper)
    } else {
        (lower, lower + 1.0)
    }
}
