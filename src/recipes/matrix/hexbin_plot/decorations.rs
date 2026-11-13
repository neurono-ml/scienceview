//! Axes, grid, title and colour bar around the hexagons.

use super::configuration::{
    SV_HEXBIN_COLOUR_BAR_LEFT, SV_HEXBIN_COLOUR_BAR_WIDTH, SV_HEXBIN_VIEWPORT_HEIGHT,
    SV_HEXBIN_VIEWPORT_WIDTH, SVHexbinConfiguration,
};
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVColourBar, SVColourBarSpace, SVDecorationPrimitive,
    SVDecorationStyle, SVGrid, SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVContinuousColourScale, SVScale};

/// Builds the decoration primitives below and above the hexagon layer.
pub(super) fn sv_build_decorations(
    configuration: &SVHexbinConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    colour_scale: &SVContinuousColourScale,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Result<(Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>), SVError> {
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(
            configuration
                .x_label
                .clone()
                .unwrap_or_else(|| configuration.x.clone()),
        )
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left)
        .title(
            configuration
                .y_label
                .clone()
                .unwrap_or_else(|| configuration.y.clone()),
        )
        .build();
    let bottom = bottom_axis.place(
        &bottom_axis.measure(horizontal, SVAxisSpace::new(plot.width)?, style)?,
        plot,
        style,
    );
    let left = left_axis.place(
        &left_axis.measure(vertical, SVAxisSpace::new(plot.height)?, style)?,
        plot,
        style,
    );

    before_marks.extend_from_slice(
        SVGrid::new()
            .place(Some(&bottom), Some(&left), plot, style)
            .primitives(),
    );
    before_marks.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), style.axis_line_width())),
    ));
    after_marks.extend_from_slice(bottom.primitives());
    after_marks.extend_from_slice(left.primitives());
    after_marks.extend(sv_colour_bar_primitives(colour_scale, plot, style)?);

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(style);
        let rectangle =
            SVResolvedRectangle::new(0.0, 8.0, SV_HEXBIN_VIEWPORT_WIDTH, measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, style).primitives());
    }

    Ok((before_marks, after_marks))
}

fn sv_colour_bar_primitives(
    colour_scale: &SVContinuousColourScale,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Result<Vec<SVDecorationPrimitive>, SVError> {
    let bar = SVColourBar::builder(colour_scale.clone())
        .number_of_bands(5)
        .build();
    let space = SVColourBarSpace::new(plot.height)?;
    let measurement = bar.measure(space, style)?;
    let rectangle = SVResolvedRectangle::new(
        SV_HEXBIN_COLOUR_BAR_LEFT,
        plot.y,
        SV_HEXBIN_COLOUR_BAR_WIDTH,
        plot.height,
    );
    Ok(bar
        .place(&measurement, rectangle, style)
        .primitives()
        .to_vec())
}

/// Returns the viewport the chart occupies.
pub(super) fn sv_viewport() -> SVViewport {
    SVViewport::new(SV_HEXBIN_VIEWPORT_WIDTH, SV_HEXBIN_VIEWPORT_HEIGHT)
}
