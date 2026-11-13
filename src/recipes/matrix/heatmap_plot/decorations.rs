//! The title, axes, cell annotations and colour bar a heatmap draws.

use super::SVHeatmapOrientation;
use super::configuration::SVHeatmapConfiguration;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVColourBar, SVColourBarSpace, SVDecorationPrimitive,
    SVDecorationStyle, SVRectanglePrimitive, SVStroke, SVTextAnchor, SVTitle, SVTitleRole,
    sv_estimate_text_width,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

/// Builds the decorations below and above the cells.
pub(super) fn sv_heatmap_decorations(
    configuration: &SVHeatmapConfiguration<'_>,
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

    let (bottom_labels, left_labels, horizontal_slots, vertical_slots) =
        match configuration.orientation {
            SVHeatmapOrientation::RowsAlongVertical => (
                column_labels,
                row_labels,
                column_labels.len(),
                row_labels.len(),
            ),
            SVHeatmapOrientation::RowsAlongHorizontal => (
                row_labels,
                column_labels,
                row_labels.len(),
                column_labels.len(),
            ),
        };
    let bottom_title = configuration
        .x_label
        .clone()
        .unwrap_or_else(|| "column".to_string());
    let left_title = configuration
        .y_label
        .clone()
        .unwrap_or_else(|| "row".to_string());
    if !bottom_labels.is_empty() && !left_labels.is_empty() {
        let bottom_categories: Vec<&str> = bottom_labels.iter().map(String::as_str).collect();
        let left_categories: Vec<&str> = left_labels.iter().map(String::as_str).collect();
        let bottom_inset = plot.width / (2.0 * horizontal_slots.max(1) as f64);
        let left_inset = plot.height / (2.0 * vertical_slots.max(1) as f64);
        let bottom = SVAxis::builder(SVAxisSide::Bottom)
            .title(bottom_title)
            .build();
        let left = SVAxis::builder(SVAxisSide::Left).title(left_title).build();
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
    colour_scale: &crate::sv_scale::SVContinuousColourScale,
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

/// Builds one centred annotation for a drawn cell.
pub(super) fn sv_cell_annotation(
    value: f64,
    fill: Option<crate::sv_scale::SVColour>,
    rectangle: SVResolvedRectangle,
    configuration: &SVHeatmapConfiguration<'_>,
    style: &SVDecorationStyle,
) -> Option<SVDecorationPrimitive> {
    if !configuration.annot {
        return None;
    }
    let text = configuration.annot_format.format(value);
    let font_size = style.label_font_size().min(rectangle.height * 0.5).max(1.0);
    if sv_estimate_text_width(&text, font_size) > rectangle.width * 0.9 {
        return None;
    }
    let colour = sv_annotation_colour(fill, style);
    Some(SVDecorationPrimitive::text(
        crate::sv_decoration::SVPoint::new(
            rectangle.x + rectangle.width / 2.0,
            rectangle.y + rectangle.height / 2.0 + font_size * 0.35,
        ),
        text,
        font_size,
        style.regular_weight(),
        colour,
        SVTextAnchor::Middle,
    ))
}

fn sv_annotation_colour(
    fill: Option<crate::sv_scale::SVColour>,
    style: &SVDecorationStyle,
) -> crate::sv_scale::SVColour {
    let Some(fill) = fill else {
        return style.text();
    };
    let luminance = 0.2126 * f64::from(fill.red())
        + 0.7152 * f64::from(fill.green())
        + 0.0722 * f64::from(fill.blue());
    if luminance < 140.0 {
        style.background()
    } else {
        style.text()
    }
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
