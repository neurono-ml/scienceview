//! The decorations that make a band legible: axes, grid, title and legend.

use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_scale::{SVColour, SVScale};

use super::configuration::SVBandPlotConfiguration;

/// Builds the decoration primitives in draw order.
pub(crate) fn sv_compose_decorations(
    configuration: &SVBandPlotConfiguration<'_>,
    horizontal: &SVScale<'_>,
    vertical: &SVScale<'_>,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
    series: &[(String, SVColour)],
) -> Result<Vec<SVDecorationPrimitive>, SVError> {
    let style = SVDecorationStyle::built_in();
    let mut primitives = Vec::new();

    if let Some(title_text) = configuration.title {
        let title = SVTitle::new(SVTitleRole::Title, title_text);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        primitives.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom);
    let bottom_axis = match configuration.x_title {
        Some(title) => bottom_axis.title(title),
        None => bottom_axis,
    }
    .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left);
    let left_axis = match configuration.y_title {
        Some(title) => left_axis.title(title),
        None => left_axis,
    }
    .build();

    let bottom = bottom_axis.place(
        &bottom_axis.measure(horizontal, SVAxisSpace::new(plot.width)?, &style)?,
        plot,
        &style,
    );
    let left = left_axis.place(
        &left_axis.measure(vertical, SVAxisSpace::new(plot.height)?, &style)?,
        plot,
        &style,
    );
    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot, &style);

    primitives.extend_from_slice(grid.primitives());
    primitives.extend_from_slice(bottom.primitives());
    primitives.extend_from_slice(left.primitives());

    if series.len() > 1 {
        let mut legend = SVLegend::new();
        for (index, (name, colour)) in series.iter().enumerate() {
            legend.add_series(format!("series-{index}"), name, *colour);
        }
        let measurement = legend.measure(&style);
        let rectangle = SVResolvedRectangle::new(
            plot.right() + 16.0,
            plot.y,
            measurement.width(),
            measurement.height(),
        );
        primitives.extend_from_slice(legend.place(&measurement, rectangle, &style).primitives());
    }

    Ok(primitives)
}
