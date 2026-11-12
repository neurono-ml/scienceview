//! The axes, grid, title and legend a scatter draws around its marks.

use super::SVScatterLegendPlacement;
use super::channels::SVScatterSeriesGroup;
use super::core_implementation::SVScatterConfiguration;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVTitle, SVTitleRole,
};
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::SVScale;

/// Builds the decorations below and above the mark layer.
pub(super) fn sv_scatter_decorations(
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    configuration: &SVScatterConfiguration<'_>,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
    groups: &[SVScatterSeriesGroup],
    style: &SVDecorationStyle,
) -> (Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>) {
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, style).primitives());
    }

    let horizontal_label = configuration
        .x_label
        .clone()
        .unwrap_or_else(|| configuration.x.clone());
    let vertical_label = configuration
        .y_label
        .clone()
        .unwrap_or_else(|| configuration.y.clone());
    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(horizontal_label)
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left)
        .title(vertical_label)
        .build();

    let bottom = sv_place_axis(&bottom_axis, horizontal, plot.width, plot, style);
    let left = sv_place_axis(&left_axis, vertical, plot.height, plot, style);
    let (Some(bottom), Some(left)) = (bottom, left) else {
        return (before_marks, after_marks);
    };

    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot, style);
    before_marks.extend_from_slice(grid.primitives());
    before_marks.push(SVDecorationPrimitive::rectangle(
        crate::sv_decoration::SVRectanglePrimitive::outlined(
            plot,
            crate::sv_decoration::SVStroke::new(style.axis(), 1.0),
        ),
    ));

    after_marks.extend_from_slice(bottom.primitives());
    after_marks.extend_from_slice(left.primitives());

    if configuration.series.is_some() {
        let mut legend = SVLegend::new();
        for (index, group) in groups.iter().enumerate() {
            legend.add_series(format!("series-{index}"), group.label.clone(), group.colour);
        }
        let measurement = legend.measure(style);
        let rectangle = sv_legend_rectangle(
            configuration.legend,
            measurement.width(),
            measurement.height(),
            plot,
            viewport,
        );
        after_marks.extend_from_slice(legend.place(&measurement, rectangle, style).primitives());
    }

    (before_marks, after_marks)
}

fn sv_place_axis(
    axis: &SVAxis,
    scale: &SVScale<'_>,
    extent: f64,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Option<crate::sv_decoration::SVAxisPlacement> {
    let space = SVAxisSpace::new(extent).ok()?;
    let measurement = axis.measure(scale, space, style).ok()?;
    Some(axis.place(&measurement, plot, style))
}

fn sv_legend_rectangle(
    placement: SVScatterLegendPlacement,
    width: f64,
    height: f64,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
) -> SVResolvedRectangle {
    let margin = 8.0;
    let (x, y) = match placement {
        SVScatterLegendPlacement::Auto | SVScatterLegendPlacement::Right => {
            (plot.right() + 12.0, plot.y)
        }
        SVScatterLegendPlacement::Left => (margin, plot.y),
        SVScatterLegendPlacement::Top => (plot.x, (plot.y - height - margin).max(margin)),
        SVScatterLegendPlacement::Bottom => (plot.x, plot.bottom() + 12.0),
        SVScatterLegendPlacement::Inside => (plot.right() - width - margin, plot.y + margin),
    };
    let maximum_x = (viewport.width() - width - margin).max(margin);
    SVResolvedRectangle::new(x.min(maximum_x), y, width, height)
}
