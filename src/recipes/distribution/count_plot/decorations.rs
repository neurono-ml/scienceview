//! The axes, grid, title and legend a count plot draws around its bars.

use super::configuration::SVCountConfiguration;
use super::preparation::SVPreparedCount;
use crate::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVTitle, SVTitleRole,
};
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVBarOrientation;
use crate::sv_scale::SVScale;
use crate::sv_stat::SVNormalisation;

/// Builds the decorations below and above the bars.
pub(super) fn sv_count_decorations(
    configuration: &SVCountConfiguration<'_>,
    prepared: &SVPreparedCount,
    category: &SVScale<'_>,
    value: &SVScale<'_>,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
    labels: &[String],
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

    let (category_title, value_title) = sv_axis_titles(configuration);
    let (bottom_scale, left_scale, bottom_title, left_title) = match configuration.orientation {
        SVBarOrientation::Vertical => (category, value, category_title, value_title),
        SVBarOrientation::Horizontal => (value, category, value_title, category_title),
    };
    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(bottom_title)
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left).title(left_title).build();

    let bottom = sv_place_axis(&bottom_axis, bottom_scale, plot.width, plot, &style);
    let left = sv_place_axis(&left_axis, left_scale, plot.height, plot, &style);
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

    if configuration.series.is_some() && prepared.series.len() > 1 {
        let mut legend = SVLegend::new();
        for (index, series) in prepared.series.iter().enumerate() {
            let label = if series.label.is_empty() {
                labels
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| format!("series {index}"))
            } else {
                series.label.clone()
            };
            legend.add_series(format!("series-{index}"), label, series.colour);
        }
        let measurement = legend.measure(&style);
        let x = (plot.right() + 12.0).min((viewport.width() - measurement.width() - 8.0).max(8.0));
        let rectangle =
            SVResolvedRectangle::new(x, plot.y, measurement.width(), measurement.height());
        after_marks.extend_from_slice(legend.place(&measurement, rectangle, &style).primitives());
    }

    (before_marks, after_marks)
}

fn sv_axis_titles(configuration: &SVCountConfiguration<'_>) -> (String, String) {
    let category_title = configuration
        .x_label
        .clone()
        .unwrap_or_else(|| configuration.category.clone());
    let value_title = configuration
        .y_label
        .clone()
        .unwrap_or_else(|| sv_measure_title(configuration.normalisation));
    match configuration.orientation {
        SVBarOrientation::Vertical => (category_title, value_title),
        SVBarOrientation::Horizontal => (value_title, category_title),
    }
}

fn sv_measure_title(normalisation: SVNormalisation) -> String {
    match normalisation {
        SVNormalisation::Count => "count",
        SVNormalisation::Probability => "probability",
        SVNormalisation::Density => "density",
    }
    .to_string()
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
