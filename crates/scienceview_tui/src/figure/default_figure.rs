//! The default figure drawn on the terminal cell grid.
//!
//! A bare `SVFigure` carries values, not a composed recipe. Until a recipe adds
//! marks and decorations, the terminal backend still draws a legible figure:
//! the values through a vertical scale, the index through a horizontal scale,
//! two axes that share those scales and a title that names the whole thing.
//!
//! The solved geometry is the same every other backend consumes; only the
//! final raster snaps to cells.

use super::default_series::SVSeries;
use super::default_series::sv_collect_series;
use crate::cell::SVCellCanvas;
use crate::cell::SVCellFrame;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_decoration::SVAxis;
use scienceview_core::sv_decoration::SVAxisSide;
use scienceview_core::sv_decoration::SVAxisSpace;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_decoration::SVDecorationStyle;
use scienceview_core::sv_decoration::SVGrid;
use scienceview_core::sv_decoration::SVRectanglePrimitive;
use scienceview_core::sv_decoration::SVStroke;
use scienceview_core::sv_decoration::SVTitle;
use scienceview_core::sv_decoration::SVTitleRole;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_layout::SVResolvedRectangle;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::SVMark;
use scienceview_core::sv_mark::SVMarkRequest;
use scienceview_core::sv_mark::SVMarkTheme;
use scienceview_core::sv_scale::SVScale;
use scienceview_core::sv_scale::SVScaleBuilder;
use scienceview_core::sv_scale::SVScaleKind;

/// The horizontal padding between the viewport and the plot.
const PLOT_HORIZONTAL_PADDING: f64 = 64.0;
/// The top padding between the viewport and the plot.
const PLOT_TOP_PADDING: f64 = 56.0;
/// The bottom padding reserved for the axis and the labels.
const PLOT_BOTTOM_PADDING: f64 = 72.0;
/// The minimum plot width a narrow viewport keeps.
const PLOT_MINIMUM_WIDTH: f64 = 32.0;
/// The minimum plot height a short viewport keeps.
const PLOT_MINIMUM_HEIGHT: f64 = 24.0;
/// Below this many columns a terminal keeps the plot and the mark only.
const COMPACT_MINIMUM_COLUMNS: usize = 32;
/// Below this many rows a terminal keeps the plot and the mark only.
const COMPACT_MINIMUM_ROWS: usize = 12;

/// Rasterizes a bare figure request onto the cell grid.
pub(crate) fn sv_rasterize_default_figure(
    request: &SVRenderRequest<'_>,
    viewport: &SVViewport,
) -> Result<SVCellFrame, SVError> {
    let style = SVDecorationStyle::built_in();
    let plot = sv_plot_rectangle(viewport);
    let series = sv_collect_series(&request.view);

    let mut canvas = SVCellCanvas::new(viewport);
    canvas.fill(style.background());

    if series.points.is_empty() {
        canvas.draw_decoration_primitive(&SVDecorationPrimitive::rectangle(
            SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), 1.0)),
        ));
        return Ok(canvas.to_frame());
    }

    let horizontal_values: Vec<f64> = series.points.iter().map(|(x, _)| *x).collect();
    let vertical_values: Vec<f64> = series.points.iter().map(|(_, y)| *y).collect();
    let horizontal = sv_horizontal_scale(&series, plot)?;
    let vertical = sv_vertical_scale(&vertical_values, plot)?;

    let bottom = SVAxis::builder(SVAxisSide::Bottom).build();
    let left = SVAxis::builder(SVAxisSide::Left).build();
    let bottom_placement = bottom.place(
        &bottom.measure(&horizontal, SVAxisSpace::new(plot.width)?, &style)?,
        plot,
        &style,
    );
    let left_placement = left.place(
        &left.measure(&vertical, SVAxisSpace::new(plot.height)?, &style)?,
        plot,
        &style,
    );

    if !sv_is_compact(viewport) {
        let grid =
            SVGrid::new().place(Some(&bottom_placement), Some(&left_placement), plot, &style);
        canvas.draw_decoration_primitives(grid.primitives());
    }
    canvas.draw_decoration_primitive(&SVDecorationPrimitive::rectangle(
        SVRectanglePrimitive::outlined(plot, SVStroke::new(style.axis(), 1.0)),
    ));

    let theme = SVMarkTheme::default();
    let mark: SVMark = SVMark::line()
        .width(SVLength::absolute(2.0))
        .colour(theme.mark_colour())
        .into();
    let mark_request = SVMarkRequest::new(
        SVColumnView::Dense(&horizontal_values),
        SVColumnView::Dense(&vertical_values),
        &horizontal,
        &vertical,
        plot,
    );
    canvas.draw_mark_geometry(&mark.geometry(&mark_request)?);

    if !sv_is_compact(viewport) {
        canvas.draw_decoration_primitives(bottom_placement.primitives());
        canvas.draw_decoration_primitives(left_placement.primitives());

        if let Some(title) = request.title {
            let title = SVTitle::new(SVTitleRole::Title, title);
            let measurement = title.measure(&style);
            let rectangle =
                SVResolvedRectangle::new(0.0, 8.0, viewport.width(), measurement.height());
            canvas.draw_decoration_primitives(
                title.place(&measurement, rectangle, &style).primitives(),
            );
        }
    }

    Ok(canvas.to_frame())
}

/// Returns `true` when the terminal is too small for labels and a title.
///
/// The plot frame and the mark are always drawn; a terminal below the threshold
/// drops the grid, the tick labels and the title instead of letting them
/// overlap the plot area. This is the terminal's responsiveness policy for the
/// documented "very small terminals cannot show every decoration" risk.
fn sv_is_compact(viewport: &SVViewport) -> bool {
    viewport.cells_wide() < COMPACT_MINIMUM_COLUMNS || viewport.cells_high() < COMPACT_MINIMUM_ROWS
}

/// Resolves the plot rectangle from a logical viewport.
fn sv_plot_rectangle(viewport: &SVViewport) -> SVResolvedRectangle {
    let width = (viewport.width() - 2.0 * PLOT_HORIZONTAL_PADDING).max(PLOT_MINIMUM_WIDTH);
    let height =
        (viewport.height() - PLOT_TOP_PADDING - PLOT_BOTTOM_PADDING).max(PLOT_MINIMUM_HEIGHT);
    SVResolvedRectangle::new(PLOT_HORIZONTAL_PADDING, PLOT_TOP_PADDING, width, height)
}

fn sv_horizontal_scale(
    series: &SVSeries,
    plot: SVResolvedRectangle,
) -> Result<SVScale<'static>, SVError> {
    let maximum = (series.logical_length.saturating_sub(1)).max(1) as f64;
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, maximum)
        .range(plot.x, plot.right())
        .build()
}

fn sv_vertical_scale(
    values: &[f64],
    plot: SVResolvedRectangle,
) -> Result<SVScale<'static>, SVError> {
    let (mut minimum, mut maximum) = values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(*value), high.max(*value))
        });
    if (maximum - minimum).abs() <= f64::EPSILON {
        let padding = if minimum.abs() <= f64::EPSILON {
            1.0
        } else {
            minimum.abs() * 0.1
        };
        minimum -= padding;
        maximum += padding;
    }

    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(minimum, maximum)
        .range(plot.bottom(), plot.y)
        .build()
}
