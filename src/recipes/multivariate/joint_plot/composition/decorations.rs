//! The axes, grid, panel frames, title and legend of a joint figure.

use super::super::channels::SVJointSeriesGroup;
use super::super::configuration::SVJointConfiguration;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVRectanglePrimitive, SVStroke, SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

/// Builds the decorations below and above the mark layer.
#[allow(clippy::too_many_arguments)]
pub(super) fn sv_joint_decorations(
    horizontal: &SVScale<'static>,
    vertical: &SVScale<'static>,
    configuration: &SVJointConfiguration<'_>,
    central: SVResolvedRectangle,
    horizontal_marginal: SVResolvedRectangle,
    vertical_marginal: SVResolvedRectangle,
    viewport: SVViewport,
    groups: &[SVJointSeriesGroup],
) -> Result<(Vec<SVDecorationPrimitive>, Vec<SVDecorationPrimitive>), SVError> {
    let style = SVDecorationStyle::built_in();
    let mut before_marks = Vec::new();
    let mut after_marks = Vec::new();

    if let Some(title) = configuration.title.as_deref() {
        let title = SVTitle::new(SVTitleRole::Title, title);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        before_marks.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    for panel in [horizontal_marginal, central, vertical_marginal] {
        before_marks.push(SVDecorationPrimitive::rectangle(
            SVRectanglePrimitive::outlined(panel, SVStroke::new(style.axis(), 1.0)),
        ));
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

    let horizontal_axis_scale = sv_relative_scale(horizontal, 0.0, central.width)?;
    let vertical_axis_scale = sv_relative_scale(vertical, central.height, 0.0)?;
    let bottom = sv_place_axis(
        &bottom_axis,
        &horizontal_axis_scale,
        central.width,
        central,
        &style,
    );
    let left = sv_place_axis(
        &left_axis,
        &vertical_axis_scale,
        central.height,
        central,
        &style,
    );
    let (Some(bottom), Some(left)) = (bottom, left) else {
        return Ok((before_marks, after_marks));
    };

    let grid = SVGrid::new().place(Some(&bottom), Some(&left), central, &style);
    before_marks.extend_from_slice(grid.primitives());
    after_marks.extend_from_slice(bottom.primitives());
    after_marks.extend_from_slice(left.primitives());

    if !groups.is_empty() && configuration.series.is_some() {
        let mut legend = SVLegend::new();
        for (index, group) in groups.iter().enumerate() {
            legend.add_series(format!("series-{index}"), group.label.clone(), group.colour);
        }
        let measurement = legend.measure(&style);
        let rectangle = SVResolvedRectangle::new(
            (central.right() - measurement.width() - 8.0).max(central.x),
            central.y + 8.0,
            measurement.width(),
            measurement.height(),
        );
        after_marks.extend_from_slice(legend.place(&measurement, rectangle, &style).primitives());
    }

    Ok((before_marks, after_marks))
}

fn sv_relative_scale(
    scale: &SVScale<'static>,
    start: f64,
    end: f64,
) -> Result<SVScale<'static>, SVError> {
    let domain = scale.domain();
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower(), domain.upper())
        .range(start, end)
        .build()
}

fn sv_place_axis(
    axis: &SVAxis,
    scale: &SVScale<'static>,
    extent: f64,
    plot: SVResolvedRectangle,
    style: &SVDecorationStyle,
) -> Option<crate::sv_decoration::SVAxisPlacement> {
    let space = SVAxisSpace::new(extent).ok()?;
    let measurement = axis.measure(scale, space, style).ok()?;
    Some(axis.place(&measurement, plot, style))
}
