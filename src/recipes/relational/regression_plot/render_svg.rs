//! SVG serialization of a composed regression chart.
//!
//! The document owns notation only: scales place the values, the marks turn
//! them into backend-agnostic primitives, the decorations are measured and
//! placed, and the SVG backend serializes the result. This is the same path the
//! documentation example takes, so the checked-in figure cannot drift from the
//! tool.

use super::composition::SVRegressionComposition;
use super::configuration::SVRegressionConfiguration;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_decoration::{
    SVAxis, SVAxisSide, SVAxisSpace, SVDecorationStyle, SVGrid, SVLegend, SVTitle, SVTitleRole,
};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVLength, SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::{SVMark, SVMarkRequest};
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};
use scienceview_svg::SVSvgDocument;

const IMAGE_WIDTH: f64 = 640.0;
const IMAGE_HEIGHT: f64 = 400.0;
const PLOT_LEFT: f64 = 72.0;
const PLOT_TOP: f64 = 56.0;
const PLOT_WIDTH: f64 = 492.0;
const PLOT_HEIGHT: f64 = 272.0;

/// Serializes a composition into a self-contained SVG document.
pub(crate) fn sv_render_svg(
    composition: &SVRegressionComposition,
    configuration: &SVRegressionConfiguration,
    title: Option<&str>,
) -> Result<String, SVError> {
    let style = SVDecorationStyle::built_in();
    let plot = SVResolvedRectangle::new(PLOT_LEFT, PLOT_TOP, PLOT_WIDTH, PLOT_HEIGHT);
    let (x_low, x_high) = composition.x_domain();
    let (y_low, y_high) = composition.y_domain();

    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(x_low, x_high)
        .range(plot.x, plot.right())
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(y_low, y_high)
        .range(plot.bottom(), plot.y)
        .build()?;

    let bottom_axis = SVAxis::builder(SVAxisSide::Bottom)
        .title(configuration.x().unwrap_or("x"))
        .build();
    let left_axis = SVAxis::builder(SVAxisSide::Left)
        .title(configuration.y().unwrap_or("y"))
        .build();
    let bottom = bottom_axis.place(
        &bottom_axis.measure(&horizontal, SVAxisSpace::new(plot.width)?, &style)?,
        plot,
        &style,
    );
    let left = left_axis.place(
        &left_axis.measure(&vertical, SVAxisSpace::new(plot.height)?, &style)?,
        plot,
        &style,
    );
    let grid = SVGrid::new().place(Some(&bottom), Some(&left), plot, &style);

    let chart_title = title.unwrap_or("Regression");
    let mut document = SVSvgDocument::new(SVViewport::new(IMAGE_WIDTH, IMAGE_HEIGHT))
        .with_title(chart_title)
        .with_description("A scatter with its fitted regression line")
        .with_background(style.background());

    let title_decoration = SVTitle::new(SVTitleRole::Title, chart_title);
    let measurement = title_decoration.measure(&style);
    document.push_decoration_primitives(
        title_decoration
            .place(
                &measurement,
                SVResolvedRectangle::new(0.0, 8.0, IMAGE_WIDTH, measurement.height()),
                &style,
            )
            .primitives(),
    );
    document.push_decoration_primitives(grid.primitives());

    for series in composition.series() {
        if let (Some(lower), Some(upper)) = (series.lower(), series.upper()) {
            let mark: SVMark = SVMark::ribbon()
                .opacity(0.25)
                .colour(series.colour())
                .into();
            let request = SVMarkRequest::new(
                SVColumnView::Dense(series.grid()),
                SVColumnView::Dense(series.predicted()),
                &horizontal,
                &vertical,
                plot,
            )
            .with_lower(SVColumnView::Dense(lower))
            .with_upper(SVColumnView::Dense(upper));
            document.push_mark_geometry(&mark.geometry(&request)?);
        }
        if configuration.show_fit() {
            let width = configuration.line_width();
            if !sv_is_zero_length(width) {
                let mark: SVMark = SVMark::line().width(width).colour(series.colour()).into();
                document.push_mark_geometry(&sv_series_geometry(
                    mark,
                    series.grid(),
                    series.predicted(),
                    &horizontal,
                    &vertical,
                    plot,
                )?);
            }
        }
        if configuration.show_points() {
            let mark: SVMark = SVMark::dot()
                .shape(configuration.marker())
                .size(SVLength::absolute(7.0))
                .opacity(0.8)
                .colour(series.colour())
                .into();
            document.push_mark_geometry(&sv_series_geometry(
                mark,
                series.points_x(),
                series.points_y(),
                &horizontal,
                &vertical,
                plot,
            )?);
        }
    }

    document.push_decoration_primitives(bottom.primitives());
    document.push_decoration_primitives(left.primitives());

    if configuration.series().is_some() {
        let mut legend = SVLegend::new();
        for series in composition.series() {
            let key = series.label().to_string();
            legend.add_series(&key, series.label(), series.colour());
        }
        let legend_measurement = legend.measure(&style);
        let legend_rectangle = SVResolvedRectangle::new(
            plot.right() + 16.0,
            plot.y,
            legend_measurement.width(),
            legend_measurement.height(),
        );
        document.push_decoration_primitives(
            legend
                .place(&legend_measurement, legend_rectangle, &style)
                .primitives(),
        );
    }

    Ok(document.to_svg())
}

/// Draws one mark from two borrowed channels.
fn sv_series_geometry<'scales>(
    mark: SVMark,
    primary: &'scales [f64],
    secondary: &'scales [f64],
    horizontal: &'scales SVScale<'scales>,
    vertical: &'scales SVScale<'scales>,
    plot: SVResolvedRectangle,
) -> Result<scienceview_core::sv_mark::SVMarkGeometry<'scales>, SVError> {
    let request = SVMarkRequest::new(
        SVColumnView::Dense(primary),
        SVColumnView::Dense(secondary),
        horizontal,
        vertical,
        plot,
    );
    mark.geometry(&request)
}

/// Returns whether an absolute length resolves to zero.
fn sv_is_zero_length(length: SVLength) -> bool {
    matches!(length, SVLength::Absolute { pixels } if pixels <= 0.0)
}
