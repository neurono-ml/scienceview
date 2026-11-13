//! The title, axis labels and colour bar the clustered heatmap draws.

use super::super::configuration::SVClusteredHeatmapConfiguration;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVColourBar, SVColourBarSpace, SVDecorationPrimitive,
    SVDecorationStyle, SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVContinuousColourScale, SVScale, SVScaleBuilder, SVScaleKind};

pub(super) fn sv_heatmap_decorations(
    configuration: &SVClusteredHeatmapConfiguration<'_>,
    row_labels: &[String],
    column_labels: &[String],
    plot: SVResolvedRectangle,
    viewport: SVViewport,
    style: &SVDecorationStyle,
) -> Result<(Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>), SVError> {
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, style).primitives());
    }

    before_marks.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), 1.0)),
    ));

    if !column_labels.is_empty() && !row_labels.is_empty() {
        let bottom_categories: Vec<&str> = column_labels.iter().map(String::as_str).collect();
        let left_categories: Vec<&str> = row_labels.iter().map(String::as_str).collect();
        let bottom_inset = plot.width / (2.0 * column_labels.len().max(1) as f64);
        let left_inset = plot.height / (2.0 * row_labels.len().max(1) as f64);
        let bottom = SVAxis::builder(SVAxisSide::Bottom)
            .title(
                configuration
                    .x_label
                    .clone()
                    .unwrap_or_else(|| "column".to_string()),
            )
            .build();
        let left = SVAxis::builder(SVAxisSide::Left)
            .title(
                configuration
                    .y_label
                    .clone()
                    .unwrap_or_else(|| "row".to_string()),
            )
            .build();
        let bottom_scale = SVScaleBuilder::new(SVScaleKind::Nominal)
            .categories(&bottom_categories)
            .range(bottom_inset, plot.width - bottom_inset)
            .build()?;
        let left_scale = SVScaleBuilder::new(SVScaleKind::Nominal)
            .categories(&left_categories)
            .range(left_inset, plot.height - left_inset)
            .build()?;
        let bottom = sv_place_axis(&bottom, &bottom_scale, plot.width, plot, style)?;
        let left = sv_place_axis(&left, &left_scale, plot.height, plot, style)?;
        after_marks.extend_from_slice(bottom.primitives());
        after_marks.extend_from_slice(left.primitives());
    }

    Ok((before_marks, after_marks))
}

/// Places the colour bar over the shared continuous colour scale.
pub(super) fn sv_colour_bar_primitives(
    colour_bar: bool,
    colour_scale: &SVContinuousColourScale,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Result<Vec<SVDecorationPrimitive>, SVError> {
    if !colour_bar {
        return Ok(Vec::new());
    }
    let bar = SVColourBar::builder(colour_scale.clone()).build();
    let measurement = bar.measure(SVColourBarSpace::new(plot.height)?, style)?;
    let rectangle = SVResolvedRectangle::new(
        plot.right() + 24.0,
        plot.y,
        measurement.band_extent(),
        plot.height,
    );
    Ok(bar
        .place(&measurement, rectangle, style)
        .primitives()
        .to_vec())
}

fn sv_place_axis(
    axis: &SVAxis,
    scale: &SVScale<'_>,
    extent: f64,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Result<crate::sv_decoration::SVAxisPlacement, SVError> {
    let space = SVAxisSpace::new(extent)?;
    let measurement = axis.measure(scale, space, style)?;
    Ok(axis.place(&measurement, plot, style))
}
