//! The title, border, axis and legend a rug draws around its ticks.

use super::channels::SVRugSeriesGroup;
use super::configuration::SVRugConfiguration;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVLegend,
    SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::SVScale;

/// Builds the decorations below and above the tick layer.
pub(super) fn sv_rug_decorations(
    axis_scale: &SVScale<'_>,
    configuration: &SVRugConfiguration<'_>,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
    groups: &[SVRugSeriesGroup],
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

    before_marks.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), 1.0)),
    ));

    let label = configuration
        .value_label
        .clone()
        .unwrap_or_else(|| configuration.value.clone());
    let (side, extent) = match configuration.orientation {
        SVBarOrientation::Vertical => (SVAxisSide::Bottom, plot.width),
        SVBarOrientation::Horizontal => (SVAxisSide::Left, plot.height),
    };
    let axis = SVAxis::builder(side).title(label).build();
    if let Ok(space) = SVAxisSpace::new(extent) {
        if let Ok(measurement) = axis.measure(axis_scale, space, style) {
            after_marks.extend_from_slice(axis.place(&measurement, plot, style).primitives());
        }
    }

    if configuration.series.is_some() {
        let mut legend = SVLegend::new();
        for (index, group) in groups.iter().enumerate() {
            legend.add_series(format!("series-{index}"), group.label.clone(), group.colour);
        }
        let measurement = legend.measure(style);
        let margin = 8.0;
        let x = (plot.right() + 12.0)
            .min((viewport.width() - measurement.width() - margin).max(margin));
        let rectangle =
            SVResolvedRectangle::new(x, plot.y, measurement.width(), measurement.height());
        after_marks.extend_from_slice(legend.place(&measurement, rectangle, style).primitives());
    }

    (before_marks, after_marks)
}
