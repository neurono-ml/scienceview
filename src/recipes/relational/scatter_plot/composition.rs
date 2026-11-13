//! One render: resolve the columns, derive the scales and compose the scene.

use super::channels::{
    sv_colour_scale, sv_point_colours, sv_point_sizes, sv_series_groups, sv_size_scale,
};
use super::composition_helpers::{
    sv_check_length, sv_optional_snapshot, sv_position_scale, sv_resolved_edge_width,
};
use super::configuration::SVScatterConfiguration;
use super::configuration::{
    SV_SCATTER_PLOT_HEIGHT, SV_SCATTER_PLOT_LEFT, SV_SCATTER_PLOT_TOP, SV_SCATTER_PLOT_WIDTH,
    SV_SCATTER_VIEWPORT_HEIGHT, SV_SCATTER_VIEWPORT_WIDTH,
};
use super::decorations::sv_scatter_decorations;
use super::geometry::{
    SV_LARGE_SCATTER_THRESHOLD, sv_group_large_scatter, sv_owned_primitive, sv_scatter_paint,
    sv_unreachable_scatter_shape,
};
use super::jitter::sv_jittered_horizontal;
use super::scene::SVScatterScene;
use crate::sv_data::{SVColumnSnapshot, SVColumnView};
use crate::sv_decoration::SVDecorationStyle;
use crate::sv_error::SVError;
use crate::sv_layout::{SVLength, SVResolvedRectangle, SVViewport};
use crate::sv_mark::{SVDotShape, SVMark, SVMarkGeometry, SVMarkRequest, SVMarkTheme};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

pub(super) fn sv_compose_scene(
    configuration: &SVScatterConfiguration<'_>,
) -> Result<SVScatterScene, SVError> {
    let style = SVDecorationStyle::built_in();
    let viewport = SVViewport::new(SV_SCATTER_VIEWPORT_WIDTH, SV_SCATTER_VIEWPORT_HEIGHT);
    let plot = SVResolvedRectangle::new(
        SV_SCATTER_PLOT_LEFT,
        SV_SCATTER_PLOT_TOP,
        SV_SCATTER_PLOT_WIDTH,
        SV_SCATTER_PLOT_HEIGHT,
    );

    let mut horizontal_snapshot = configuration.data_source.column(&configuration.x)?;
    let mut vertical_snapshot = configuration.data_source.column(&configuration.y)?;
    let mut series_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.series)?;
    let mut size_snapshot = sv_optional_snapshot(configuration.data_source, &configuration.size)?;
    let mut colour_snapshot =
        sv_optional_snapshot(configuration.data_source, &configuration.colour)?;

    let horizontal = horizontal_snapshot.column_view();
    let vertical = vertical_snapshot.column_view();
    let series = series_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let size = size_snapshot.as_mut().map(SVColumnSnapshot::column_view);
    let colour = colour_snapshot.as_mut().map(SVColumnSnapshot::column_view);

    let number_of_points = horizontal.length();
    sv_check_length("x", number_of_points, "y", vertical)?;
    if let Some(series) = series {
        sv_check_length("x", number_of_points, "series", series)?;
    }
    if let Some(size) = size {
        sv_check_length("x", number_of_points, "size", size)?;
    }
    if let Some(colour) = colour {
        sv_check_length("x", number_of_points, "colour", colour)?;
    }

    let jittered = configuration
        .jitter
        .as_ref()
        .map(|jitter| sv_jittered_horizontal(horizontal, series, jitter, number_of_points))
        .transpose()?;
    let horizontal_values = jittered.as_deref().map_or(horizontal, SVColumnView::Dense);

    let horizontal_scale = sv_position_scale(horizontal_values, plot.x, plot.right())?;
    let vertical_scale = sv_position_scale(vertical, plot.bottom(), plot.y)?;
    // Axis placement adds the plot origin back, so the decoration scales answer
    // in plot-relative coordinates while the marks read the absolute scales.
    let horizontal_axis_scale = sv_relative_scale(&horizontal_scale, 0.0, plot.width)?;
    let vertical_axis_scale = sv_relative_scale(&vertical_scale, plot.height, 0.0)?;

    let colour_scale = match colour {
        Some(colour) if number_of_points > 0 => Some(sv_colour_scale(colour)?),
        Some(_) | None => None,
    };
    let size_scale = match size {
        Some(size) if number_of_points > 0 => Some(sv_size_scale(
            size,
            configuration.size_scale.minimum(),
            configuration.size_scale.maximum(),
        )?),
        Some(_) | None => None,
    };

    let theme = SVMarkTheme::default();
    let (groups, group_indices) = sv_series_groups(
        series,
        &configuration.series_labels,
        number_of_points,
        theme.mark_colour(),
    )?;
    let point_colours = sv_point_colours(&groups, &group_indices, colour, colour_scale.as_ref())?;
    let point_sizes = sv_point_sizes(size, size_scale.as_ref(), configuration.base_marker_size)?;

    let mark: SVMark = SVMark::dot()
        .shape(configuration.marker)
        .size(SVLength::absolute(configuration.base_marker_size))
        .into();
    let request = SVMarkRequest::new(
        horizontal_values,
        vertical,
        &horizontal_scale,
        &vertical_scale,
        plot,
    );
    let geometry = mark.geometry(&request)?;
    let edge_width = sv_resolved_edge_width(configuration.edge_width, plot);
    let cross = configuration.marker == SVDotShape::Cross;
    let factor_base = configuration.base_marker_size.max(f64::MIN_POSITIVE);

    let mut primitives = Vec::with_capacity(geometry.len());
    for (index, primitive) in geometry.into_primitives().iter().enumerate() {
        let colour = point_colours
            .get(index)
            .copied()
            .unwrap_or_else(|| theme.mark_colour());
        let paint = sv_scatter_paint(
            colour,
            configuration.edge_colour,
            edge_width,
            configuration.alpha,
            cross,
            theme.line_width(),
        );
        let diameter = point_sizes
            .get(index)
            .copied()
            .unwrap_or(configuration.base_marker_size);
        let owned = sv_owned_primitive(primitive, paint, diameter / factor_base)
            .ok_or_else(sv_unreachable_scatter_shape)?;
        primitives.push(owned);
    }

    let large = configuration.marker == SVDotShape::Circle
        && number_of_points >= SV_LARGE_SCATTER_THRESHOLD;
    let primitives = if large {
        sv_group_large_scatter(primitives)
    } else {
        primitives
    };

    let legend_labels: Vec<String> = if configuration.series.is_some() {
        groups.iter().map(|group| group.label.clone()).collect()
    } else {
        Vec::new()
    };
    let (before_marks, after_marks) = sv_scatter_decorations(
        &horizontal_axis_scale,
        &vertical_axis_scale,
        configuration,
        plot,
        viewport,
        &groups,
        &style,
    );

    Ok(SVScatterScene {
        viewport,
        plot_area: plot,
        background: style.background(),
        title: configuration.title.clone(),
        description: Some("Scatter plot generated by scienceview".to_string()),
        decoration_primitives_before_marks: before_marks,
        mark_geometries: vec![SVMarkGeometry::new(primitives)],
        decoration_primitives_after_marks: after_marks,
        legend_placement: configuration.legend,
        legend_labels,
        number_of_series: groups.len(),
        number_of_marks: number_of_points,
    })
}

fn sv_relative_scale(
    scale: &SVScale<'_>,
    start: f64,
    end: f64,
) -> Result<SVScale<'static>, SVError> {
    let domain = scale.domain();
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower(), domain.upper())
        .range(start, end)
        .build()
}
