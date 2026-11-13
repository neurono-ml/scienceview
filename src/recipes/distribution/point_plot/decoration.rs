//! The decorations that make a point plot legible: axes, grid, title and
//! legend.

use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationPrimitive, SVDecorationStyle, SVGrid, SVLegend,
    SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

use super::configuration::SVPointPlotConfiguration;
use super::data_contract::{sv_category_label, sv_series_label};
use super::statistics::SVPointSummary;

/// Builds the decoration primitives in draw order.
pub(crate) fn sv_compose_decorations(
    configuration: &SVPointPlotConfiguration<'_>,
    summary: &SVPointSummary,
    nominal_scale: &SVScale<'_>,
    value_scale: &SVScale<'_>,
    plot: SVResolvedRectangle,
    viewport: SVViewport,
) -> Result<Vec<SVDecorationPrimitive>, SVError> {
    let style = SVDecorationStyle::built_in();
    let mut primitives = Vec::new();

    if let Some(title_text) = configuration.title {
        let title = SVTitle::new(SVTitleRole::Title, title_text);
        let measurement = title.measure(&style);
        let rectangle = SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
        primitives.extend_from_slice(title.place(&measurement, rectangle, &style).primitives());
    }

    let relative_vertical = sv_relative_value_scale(value_scale, plot.height, 0.0)?;
    let relative_horizontal = sv_relative_value_scale(value_scale, 0.0, plot.width)?;
    let (bottom_scale, left_scale, bottom_title, left_title) = match configuration.orientation {
        SVBarOrientation::Vertical => (
            nominal_scale,
            &relative_vertical,
            configuration.x_title,
            configuration.y_title,
        ),
        SVBarOrientation::Horizontal => (
            &relative_horizontal,
            nominal_scale,
            configuration.x_title,
            configuration.y_title,
        ),
    };
    let bottom_axis = sv_axis(SVAxisSide::Bottom, bottom_title);
    let left_axis = sv_axis(SVAxisSide::Left, left_title);

    let bottom = bottom_axis.place(
        &bottom_axis.measure(bottom_scale, SVAxisSpace::new(plot.width)?, &style)?,
        plot,
        &style,
    );
    let left = left_axis.place(
        &left_axis.measure(left_scale, SVAxisSpace::new(plot.height)?, &style)?,
        plot,
        &style,
    );
    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot, &style);

    primitives.extend_from_slice(grid.primitives());
    primitives.extend_from_slice(bottom.primitives());
    primitives.extend_from_slice(left.primitives());

    if summary.series().len() > 1 {
        let palette = scienceview_core::sv_scale::sv_default_scale_palette();
        let mut legend = SVLegend::new();
        for (index, series) in summary.series().iter().enumerate() {
            if let Some(series) = series {
                legend.add_series(
                    format!("series-{index}"),
                    sv_series_label(*series),
                    palette.colour_at_index(index),
                );
            }
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

fn sv_axis(side: SVAxisSide, title: Option<&str>) -> SVAxis {
    let builder = SVAxis::builder(side);
    match title {
        Some(title) => builder.title(title).build(),
        None => builder.build(),
    }
}

/// Builds the relative axis scale the decoration reads.
///
/// Axis placement adds the plot origin back, so the axis scale answers in
/// plot-relative coordinates while the marks read the absolute scale.
fn sv_relative_value_scale(
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

/// The categorical labels the nominal axis shows, in draw order.
pub(crate) fn sv_category_labels(summary: &SVPointSummary) -> Vec<String> {
    summary
        .categories()
        .iter()
        .map(|category| sv_category_label(*category))
        .collect()
}
