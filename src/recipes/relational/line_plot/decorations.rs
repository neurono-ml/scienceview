//! Axes, grid, frame, legend and title around the line plot.

use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVLegendPosition, SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_scale::SVScale;

use super::SVLinePlot;
use super::composition::SV_LINE_PLOT_VIEWPORT_WIDTH;
use super::series::SVLinePlotSeries;

pub(super) fn sv_build_decorations(
    plot: &SVLinePlot<'_>,
    series: &[SVLinePlotSeries],
    horizontal_scale: &SVScale<'_>,
    vertical_scale: &SVScale<'_>,
    style: &SVDecorationStyle,
) -> Result<Vec<SVDecorationPrimitive>, SVError> {
    let plot_area = SVResolvedRectangle::new(
        super::composition::SV_LINE_PLOT_PLOT_LEFT,
        super::composition::SV_LINE_PLOT_PLOT_TOP,
        super::composition::SV_LINE_PLOT_PLOT_WIDTH,
        super::composition::SV_LINE_PLOT_PLOT_HEIGHT,
    );

    let mut bottom = SVAxis::builder(SVAxisSide::Bottom);
    if let Some(title) = &plot.x_axis_title {
        bottom = bottom.title(title.clone());
    }
    let bottom = bottom.build();
    let mut left = SVAxis::builder(SVAxisSide::Left);
    if let Some(title) = &plot.y_axis_title {
        left = left.title(title.clone());
    }
    let left = left.build();

    let bottom_placement = bottom.place(
        &bottom.measure(horizontal_scale, SVAxisSpace::new(plot_area.width)?, style)?,
        plot_area,
        style,
    );
    let left_placement = left.place(
        &left.measure(vertical_scale, SVAxisSpace::new(plot_area.height)?, style)?,
        plot_area,
        style,
    );

    let mut primitives = Vec::new();
    primitives.extend_from_slice(
        SVGrid::new()
            .place(
                Some(&bottom_placement),
                Some(&left_placement),
                plot_area,
                style,
            )
            .primitives(),
    );
    primitives.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(
            plot_area,
            SVStroke::new(style.axis(), style.axis_line_width()),
        ),
    ));
    primitives.extend_from_slice(bottom_placement.primitives());
    primitives.extend_from_slice(left_placement.primitives());
    primitives.extend(sv_legend_primitives(plot, series, plot_area, style));

    if let Some(title) = &plot.title {
        let title = SVTitle::new(SVTitleRole::Title, title.clone());
        let measurement = title.measure(style);
        let rectangle =
            SVResolvedRectangle::new(0.0, 8.0, SV_LINE_PLOT_VIEWPORT_WIDTH, measurement.height());
        primitives.extend_from_slice(title.place(&measurement, rectangle, style).primitives());
    }
    Ok(primitives)
}

fn sv_legend_primitives(
    plot: &SVLinePlot<'_>,
    series: &[SVLinePlotSeries],
    plot_area: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Vec<SVDecorationPrimitive> {
    if !plot.legend || series.len() < 2 {
        return Vec::new();
    }

    let mut legend = SVLegend::new().with_position(SVLegendPosition::Inside);
    for entry in series {
        legend.add_series(entry.key.clone(), entry.label.clone(), entry.colour);
    }
    let measurement = legend.measure(style);
    let rectangle = SVResolvedRectangle::new(
        (plot_area.right() - measurement.width() - style.gap()).max(plot_area.x),
        plot_area.y + style.gap(),
        measurement.width(),
        measurement.height(),
    );
    legend
        .place(&measurement, rectangle, style)
        .primitives()
        .to_vec()
}
