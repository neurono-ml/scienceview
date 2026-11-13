//! The title, border, axes and legend a strip plot draws around its dots.

use super::configuration::SVStripConfiguration;
use super::data_contract::SVStripSeriesGroup;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

/// Builds the decorations below and above the dot layer.
pub(super) fn sv_strip_decorations(
    configuration: &SVStripConfiguration<'_>,
    value_scale: &SVScale<'_>,
    nominal_scale: &SVScale<'_>,
    groups: &[SVStripSeriesGroup],
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

    let relative_vertical = sv_relative_scale(value_scale, plot.height, 0.0)?;
    let relative_horizontal = sv_relative_scale(value_scale, 0.0, plot.width)?;
    let value_title = configuration
        .value_label
        .clone()
        .unwrap_or_else(|| configuration.value.clone());
    let category_title = configuration
        .category_label
        .clone()
        .unwrap_or_else(|| configuration.category.clone());
    let (bottom_scale, left_scale, bottom_title, left_title) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            nominal_scale,
            &relative_vertical,
            category_title,
            value_title,
        ),
        SVBarOrientation::Horizontal => (
            &relative_horizontal,
            nominal_scale,
            value_title,
            category_title,
        ),
    };

    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(bottom_title)
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left).title(left_title).build();
    let bottom = bottom_axis.place(
        &bottom_axis.measure(bottom_scale, SVAxisSpace::new(plot.width)?, style)?,
        plot,
        style,
    );
    let left = left_axis.place(
        &left_axis.measure(left_scale, SVAxisSpace::new(plot.height)?, style)?,
        plot,
        style,
    );

    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot, style);
    before_marks.extend_from_slice(grid.primitives());
    before_marks.push(SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), 1.0)),
    ));
    after_marks.extend_from_slice(bottom.primitives());
    after_marks.extend_from_slice(left.primitives());

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

    Ok((before_marks, after_marks))
}

fn sv_relative_scale(
    scale: &SVScale<'_>,
    start: f64,
    end: f64,
) -> Result<SVScale<'static>, SVError> {
    let domain = scale.domain();
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower(), domain.upper())
        .range(start, end)
        .build()
}
