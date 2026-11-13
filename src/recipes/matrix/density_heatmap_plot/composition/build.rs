//! The composition pass: resolve columns, build grids, scales and decorations.

use super::super::cell_accumulation::sv_build_grids;
use super::super::cells::sv_build_cells;
use super::super::colour_scale::sv_build_colour_scale;
use super::super::configuration::{
    SV_DENSITY_HEATMAP_PLOT_HEIGHT, SV_DENSITY_HEATMAP_PLOT_LEFT, SV_DENSITY_HEATMAP_PLOT_TOP,
    SV_DENSITY_HEATMAP_PLOT_WIDTH, SVDensityHeatmapConfiguration,
};
use super::super::decorations::{sv_build_decorations, sv_viewport};
use super::super::grid::SVDensityHeatmapGrid;
use super::definition::SVDensityHeatmapComposition;
use crate::sv_data::{SVColumnSnapshot, SVDataSource};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_layout::SVResolvedRectangle;
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

pub(crate) fn sv_compose(
    configuration: &SVDensityHeatmapConfiguration<'_>,
) -> Result<SVDensityHeatmapComposition, SVError> {
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
        sv_build_grids(configuration, horizontal, vertical, series, weight)?;
    let (horizontal_scale, vertical_scale) = sv_build_scales(&grids)?;
    let colour_scale = sv_build_colour_scale(configuration, &grids)?;
    let plot = sv_plot_area();
    let mark_geometries = sv_build_cells(
        configuration,
        &grids,
        &colour_scale,
        &horizontal_scale,
        &vertical_scale,
    )?;
    let number_of_drawn_marks = mark_geometries.iter().map(SVMarkGeometry::len).sum();
    let style = SVDecorationStyle::built_in();
    let (decorations_before_marks, decorations_after_marks) = sv_build_decorations(
        configuration,
        &horizontal_scale,
        &vertical_scale,
        &colour_scale,
        plot,
        &style,
    )?;

    Ok(SVDensityHeatmapComposition {
        backend_kind: configuration.backend_kind,
        representation: configuration.representation,
        viewport: sv_viewport(),
        plot_area: plot,
        background: style.background(),
        title: configuration.title.clone(),
        horizontal_scale,
        vertical_scale,
        colour_scale,
        colour_bar: configuration.colour_bar,
        empty_bin_policy: configuration.empty_bin_policy,
        grids,
        series_labels,
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

pub(super) fn sv_plot_area() -> SVResolvedRectangle {
    SVResolvedRectangle::new(
        SV_DENSITY_HEATMAP_PLOT_LEFT,
        SV_DENSITY_HEATMAP_PLOT_TOP,
        SV_DENSITY_HEATMAP_PLOT_WIDTH,
        SV_DENSITY_HEATMAP_PLOT_HEIGHT,
    )
}

fn sv_build_scales(
    grids: &[SVDensityHeatmapGrid],
) -> Result<(SVScale<'static>, SVScale<'static>), SVError> {
    let Some(grid) = grids.first() else {
        return Err(SVError::EmptyInput {
            operation: "density_heatmap",
        });
    };
    let horizontal_domain = sv_domain(grid.x_edges());
    let vertical_domain = sv_domain(grid.y_edges());
    let plot = sv_plot_area();
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
