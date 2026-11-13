//! The axes, grid, title and optional colour bar an image plot draws.

use super::configuration::sv_resolved_extent;
use super::configuration::{SV_IMAGE_COLOUR_BAR_GAP, SVImageConfiguration};
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVColourBar, SVColourBarSpace, SVDecorationPrimitive,
    SVDecorationStyle, SVGrid, SVTitle, SVTitleRole,
};
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVContinuousColourScale, SVScale, SVScaleBuilder, SVScaleKind};

/// Builds the decorations below and above the raster.
pub(super) fn sv_image_decorations(
    configuration: &SVImageConfiguration<'_>,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
    colour_scale: Option<&SVContinuousColourScale>,
) -> (Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>) {
    let style = SVDecorationStyle::built_in();
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    let horizontal_label = configuration
        .x_label
        .clone()
        .unwrap_or_else(|| "column".to_string());
    let vertical_label = configuration
        .y_label
        .clone()
        .unwrap_or_else(|| "row".to_string());
    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(horizontal_label)
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left)
        .title(vertical_label)
        .build();

    let Some(horizontal) = sv_horizontal_axis_scale(configuration, plot.width) else {
        return (before_marks, after_marks);
    };
    let Some(vertical) = sv_vertical_axis_scale(configuration, plot.height) else {
        return (before_marks, after_marks);
    };

    let bottom = sv_place_axis(&bottom_axis, &horizontal, plot.width, plot, &style);
    let left = sv_place_axis(&left_axis, &vertical, plot.height, plot, &style);
    let (Some(bottom), Some(left)) = (bottom, left) else {
        return (before_marks, after_marks);
    };

    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot, &style);
    before_marks.extend_from_slice(grid.primitives());
    before_marks.push(SVDecorationPrimitive::rectangle(
        crate::sv_decoration::SVRectanglePrimitive::outlined(
            plot,
            crate::sv_decoration::SVStroke::new(style.axis(), 1.0),
        ),
    ));

    after_marks.extend_from_slice(bottom.primitives());
    after_marks.extend_from_slice(left.primitives());

    if configuration.colour_bar {
        if let Some(colour_scale) = colour_scale {
            sv_push_colour_bar(&mut after_marks, colour_scale, plot, &style);
        }
    }

    (before_marks, after_marks)
}

fn sv_horizontal_axis_scale(
    configuration: &SVImageConfiguration<'_>,
    width: f64,
) -> Option<SVScale<'static>> {
    let extent = sv_resolved_extent(configuration);
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(extent.horizontal_lower(), extent.horizontal_upper())
        .range(0.0, width)
        .build()
        .ok()
}

fn sv_vertical_axis_scale(
    configuration: &SVImageConfiguration<'_>,
    height: f64,
) -> Option<SVScale<'static>> {
    let extent = sv_resolved_extent(configuration);
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(extent.vertical_lower(), extent.vertical_upper())
        .range(height, 0.0)
        .build()
        .ok()
}

fn sv_push_colour_bar(
    primitives: &mut Vec<SVDecorationPrimitive>,
    colour_scale: &SVContinuousColourScale,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) {
    let bar = SVColourBar::builder(colour_scale.clone()).build();
    let Ok(space) = SVColourBarSpace::new(plot.height) else {
        return;
    };
    let Ok(measurement) = bar.measure(space, style) else {
        return;
    };
    let rectangle = SVResolvedRectangle::new(
        plot.right() + SV_IMAGE_COLOUR_BAR_GAP,
        plot.y,
        measurement.band_extent(),
        plot.height,
    );
    primitives.extend_from_slice(bar.place(&measurement, rectangle, style).primitives());
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
