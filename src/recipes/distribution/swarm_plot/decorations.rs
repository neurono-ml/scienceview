//! The axes, grid, title and legend a swarm draws around its dots.

use super::channels::SVSwarmData;
use super::configuration::SVSwarmConfiguration;
use super::scales::SVSwarmScales;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVTitle, SVTitleRole,
};
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

/// Builds the decorations below and above the dot layer.
pub(super) fn sv_swarm_decorations(
    configuration: &SVSwarmConfiguration<'_>,
    data: &SVSwarmData,
    scales: &SVSwarmScales<'_>,
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

    let relative_value = sv_relative_value_scale(&scales.value, plot)?;
    let (bottom_scale, left_scale, bottom_title, left_title) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            &scales.nominal,
            &relative_value,
            configuration
                .x_label
                .as_deref()
                .unwrap_or(&configuration.category),
            configuration
                .y_label
                .as_deref()
                .unwrap_or(&configuration.value),
        ),
        SVBarOrientation::Horizontal => (
            &relative_value,
            &scales.nominal,
            configuration
                .x_label
                .as_deref()
                .unwrap_or(&configuration.value),
            configuration
                .y_label
                .as_deref()
                .unwrap_or(&configuration.category),
        ),
    };

    let bottom_axis = sv_axis(SVAxisSide::Bottom, bottom_title);
    let left_axis = sv_axis(SVAxisSide::Left, left_title);
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
    after_marks.extend_from_slice(bottom.primitives());
    after_marks.extend_from_slice(left.primitives());

    if configuration.series.is_some() && data.series.len() > 1 {
        let mut legend = SVLegend::new();
        for (index, group) in data.series.iter().enumerate() {
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

fn sv_axis(side: SVAxisSide, title: &str) -> SVAxis {
    SVAxis::builder(side).title(title).build()
}

/// Builds the plot-relative value scale the decoration reads.
///
/// Axis placement adds the plot origin back, so the axis scale answers in
/// plot-relative coordinates while the marks read the absolute scale.
fn sv_relative_value_scale(
    scale: &SVScale<'_>,
    plot: SVResolvedRectangle,
) -> Result<SVScale<'static>, SVError> {
    let domain = scale.domain();
    let reversed = scale.range().start() > scale.range().end();
    let (start, end) = if reversed {
        (plot.height, 0.0)
    } else {
        (0.0, plot.width)
    };
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(domain.lower(), domain.upper())
        .range(start, end)
        .build()
}
