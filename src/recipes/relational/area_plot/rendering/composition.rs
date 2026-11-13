//! Composing the scales, axes and decorations into one scene.

use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid,
    SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_scale::{SVScaleBuilder, SVScaleKind};

use super::super::configuration::SVAreaPlotConfiguration;
use super::super::preparation::SVPreparedArea;
use super::geometry::{sv_horizontal_domain, sv_push_segment, sv_vertical_domain};
use super::scene::SVAreaRendering;

/// Places the prepared areas and returns the backend-agnostic figure.
///
/// # Errors
///
/// Returns the scale, axis or mark configuration error when a kernel cannot
/// place the prepared values.
pub(crate) fn sv_compose_area(
    configuration: &SVAreaPlotConfiguration<'_>,
    prepared: SVPreparedArea,
) -> Result<SVAreaRendering, SVError> {
    let viewport = configuration.viewport;
    let style = SVDecorationStyle::built_in();
    let plot = sv_plot_area(viewport);
    let (x_lower, x_upper) = sv_horizontal_domain(&prepared);
    let horizontal_axis = SVScaleBuilder::new(configuration.x_scale_kind)
        .domain(x_lower, x_upper)
        .range(plot.x, plot.right())
        .temporal_unit(configuration.temporal_unit)
        .build()?;
    // A temporal scale answers timestamp values, while the mark kernel places
    // numeric positions. Both scales share domain and range, so the temporal
    // axis and the numeric placement cannot disagree.
    let horizontal = if configuration.x_scale_kind == SVScaleKind::Temporal {
        SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(x_lower, x_upper)
            .range(plot.x, plot.right())
            .build()?
    } else {
        horizontal_axis.clone()
    };
    let (y_lower, y_upper) = sv_vertical_domain(configuration, &prepared);
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(y_lower, y_upper)
        .range(plot.bottom(), plot.y)
        .build()?;

    let bottom = sv_build_axis(SVAxisSide::Bottom, configuration.x_title);
    let left = sv_build_axis(SVAxisSide::Left, configuration.y_title);
    let bottom_placement = bottom.place(
        &bottom.measure(&horizontal_axis, SVAxisSpace::new(plot.width)?, &style)?,
        plot,
        &style,
    );
    let left_placement = left.place(
        &left.measure(&vertical, SVAxisSpace::new(plot.height)?, &style)?,
        plot,
        &style,
    );

    let mut grid_and_frame = Vec::new();
    let grid = SVGrid::new().place(Some(&bottom_placement), Some(&left_placement), plot, &style);
    grid_and_frame.extend_from_slice(grid.primitives());
    grid_and_frame.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), style.axis_line_width())),
    ));

    let theme = configuration.theme.clone().unwrap_or_default();
    let mut primitives = Vec::new();
    for series in &prepared.series {
        for segment in &series.segments {
            sv_push_segment(
                configuration,
                series.colour,
                segment,
                plot,
                &horizontal,
                &vertical,
                &theme,
                &mut primitives,
            )?;
        }
    }

    let mut axes_and_title = Vec::new();
    axes_and_title.extend_from_slice(bottom_placement.primitives());
    axes_and_title.extend_from_slice(left_placement.primitives());
    if let Some(title) = configuration.title {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        axes_and_title.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    Ok(SVAreaRendering {
        viewport,
        title: configuration.title.map(str::to_string),
        background: style.background(),
        grid_and_frame,
        primitives,
        axes_and_title,
        representation: prepared.representation,
        point_count: prepared.point_count,
        stack: configuration.stack,
        baseline: configuration.baseline,
        opacity: configuration.opacity,
        line: configuration.line,
        interpolation: configuration.interpolation,
        missing_value_policy: configuration.missing_value_policy,
    })
}

fn sv_build_axis(side: SVAxisSide, title: Option<&str>) -> SVAxis {
    let builder = SVAxis::builder(side);
    match title {
        Some(title) => builder.title(title).build(),
        None => builder.build(),
    }
}

fn sv_plot_area(viewport: SVViewport) -> SVResolvedRectangle {
    let left = 64.0;
    let top = 56.0;
    let right_margin = 48.0;
    let bottom_margin = 72.0;
    SVResolvedRectangle::new(
        left,
        top,
        (viewport.width() - left - right_margin).max(1.0),
        (viewport.height() - top - bottom_margin).max(1.0),
    )
}
