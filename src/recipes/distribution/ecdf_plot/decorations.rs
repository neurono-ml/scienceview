//! The title, grid, axes and legend an ECDF plot draws.

use super::composition::SVEcdfSeries;
use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVLegendPosition, SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_scale::SVScale;

/// Everything the decoration pass needs from the composed chart.
pub(super) struct SVEcdfDecorationInputs<'text, 'scale> {
    pub(super) title: Option<&'text str>,
    pub(super) horizontal_title: &'text str,
    pub(super) vertical_title: &'text str,
    pub(super) horizontal_scale: &'scale SVScale<'scale>,
    pub(super) vertical_scale: &'scale SVScale<'scale>,
    pub(super) plot: SVResolvedRectangle,
    pub(super) viewport: SVViewport,
    pub(super) series: &'text [SVEcdfSeries],
    pub(super) style: &'text SVDecorationStyle,
}

/// Measures and places every decoration, split around the marks.
pub(super) fn sv_compose_decorations(
    inputs: &SVEcdfDecorationInputs<'_, '_>,
) -> Result<(Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>), SVError> {
    let mut before = Vec::new();
    let mut after = Vec::new();

    if let Some(title) = inputs.title {
        let decoration = SVTitle::new(SVTitleRole::Title, title);
        let measurement = decoration.measure(inputs.style);
        let rectangle =
            SVResolvedRectangle::new(0.0, 8.0, inputs.viewport.width(), measurement.height());
        before.extend_from_slice(
            decoration
                .place(&measurement, rectangle, inputs.style)
                .primitives(),
        );
    }

    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(inputs.horizontal_title)
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left)
        .title(inputs.vertical_title)
        .build();
    let bottom = bottom_axis.place(
        &bottom_axis.measure(
            inputs.horizontal_scale,
            SVAxisSpace::new(inputs.plot.width)?,
            inputs.style,
        )?,
        inputs.plot,
        inputs.style,
    );
    let left = left_axis.place(
        &left_axis.measure(
            inputs.vertical_scale,
            SVAxisSpace::new(inputs.plot.height)?,
            inputs.style,
        )?,
        inputs.plot,
        inputs.style,
    );

    let grid = SVGrid::new().place(Some(&bottom), Some(&left), inputs.plot, inputs.style);
    before.extend_from_slice(grid.primitives());

    after.extend_from_slice(bottom.primitives());
    after.extend_from_slice(left.primitives());

    if inputs.series.len() > 1 {
        let mut legend = SVLegend::new().with_position(SVLegendPosition::Inside);
        for line in inputs.series {
            legend.add_series(line.key(), line.label(), line.colour());
        }
        let measurement = legend.measure(inputs.style);
        let rectangle = SVResolvedRectangle::new(
            inputs.plot.right() - measurement.width() - 12.0,
            inputs.plot.y + 12.0,
            measurement.width(),
            measurement.height(),
        );
        after.extend_from_slice(
            legend
                .place(&measurement, rectangle, inputs.style)
                .primitives(),
        );
    }

    Ok((before, after))
}
