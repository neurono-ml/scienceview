//! The composed chart: series, scales, plot geometry and decorations.

use scienceview_core::sv_decoration::{SVDecorationPrimitive, SVDecorationStyle};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind};

use super::SVLinePlot;
use super::decorations::sv_build_decorations;
use super::geometry::sv_visit_geometries;
use super::series::{SVLinePlotSeries, sv_build_series};

/// The viewport every generated line-plot figure uses.
pub const SV_LINE_PLOT_VIEWPORT_WIDTH: f64 = 640.0;
/// The height every generated line-plot figure uses.
pub const SV_LINE_PLOT_VIEWPORT_HEIGHT: f64 = 400.0;
/// The left edge of the solved plot area.
pub const SV_LINE_PLOT_PLOT_LEFT: f64 = 64.0;
/// The top edge of the solved plot area.
pub const SV_LINE_PLOT_PLOT_TOP: f64 = 56.0;
/// The width of the solved plot area.
pub const SV_LINE_PLOT_PLOT_WIDTH: f64 = 528.0;
/// The height of the solved plot area.
pub const SV_LINE_PLOT_PLOT_HEIGHT: f64 = 272.0;

/// A line plot resolved into everything a backend serializes.
#[derive(Clone, Debug)]
pub struct SVLinePlotComposition {
    viewport: SVViewport,
    plot_area: SVResolvedRectangle,
    horizontal_scale: SVScale<'static>,
    vertical_scale: SVScale<'static>,
    series: Vec<SVLinePlotSeries>,
    decorations: Vec<SVDecorationPrimitive>,
    title: Option<String>,
}

impl SVLinePlotComposition {
    /// Returns the viewport the chart occupies.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the solved plot rectangle.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the horizontal scale.
    pub fn horizontal_scale(&self) -> &SVScale<'static> {
        &self.horizontal_scale
    }

    /// Returns the vertical scale.
    pub fn vertical_scale(&self) -> &SVScale<'static> {
        &self.vertical_scale
    }

    /// Returns every series in first-appearance order.
    pub fn series(&self) -> &[SVLinePlotSeries] {
        &self.series
    }

    /// Returns the chart title, when it has one.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the axis, grid and legend primitives in draw order.
    pub fn decorations(&self) -> &[SVDecorationPrimitive] {
        &self.decorations
    }

    /// Visits every mark geometry in draw order: intervals, lines, then markers.
    ///
    /// The geometry is borrowed for the duration of the callback, so a backend
    /// serializes it without the recipe owning a document type.
    ///
    /// # Errors
    ///
    /// Returns the mark kernel's [`SVError`] when a geometry cannot be built.
    pub fn visit_mark_geometries(
        &self,
        visitor: impl FnMut(&scienceview_core::sv_mark::SVMarkGeometry<'_>),
    ) -> Result<(), SVError> {
        sv_visit_geometries(self, visitor)
    }

    /// Returns how many primitives the chart draws.
    ///
    /// # Errors
    ///
    /// Returns the mark kernel's [`SVError`] when a geometry cannot be built.
    pub fn number_of_primitives(&self) -> Result<usize, SVError> {
        let mut count = 0;
        self.visit_mark_geometries(|geometry| count += geometry.len())?;
        Ok(count + self.decorations.len())
    }
}

pub(super) fn sv_compose_line_plot(
    plot: &SVLinePlot<'_>,
) -> Result<SVLinePlotComposition, SVError> {
    let series = sv_build_series(plot)?;
    let (horizontal_scale, vertical_scale) = sv_build_scales(&series)?;
    let plot_area = SVResolvedRectangle::new(
        SV_LINE_PLOT_PLOT_LEFT,
        SV_LINE_PLOT_PLOT_TOP,
        SV_LINE_PLOT_PLOT_WIDTH,
        SV_LINE_PLOT_PLOT_HEIGHT,
    );
    let style = SVDecorationStyle::built_in();
    let decorations =
        sv_build_decorations(plot, &series, &horizontal_scale, &vertical_scale, &style)?;

    Ok(SVLinePlotComposition {
        viewport: SVViewport::new(SV_LINE_PLOT_VIEWPORT_WIDTH, SV_LINE_PLOT_VIEWPORT_HEIGHT),
        plot_area,
        horizontal_scale,
        vertical_scale,
        series,
        decorations,
        title: plot.title.clone(),
    })
}

fn sv_build_scales(
    series: &[SVLinePlotSeries],
) -> Result<(SVScale<'static>, SVScale<'static>), SVError> {
    let mut x_values = Vec::new();
    let mut y_values = Vec::new();
    for series in series {
        for segment in &series.segments {
            for (x, y) in &segment.line {
                if x.is_finite() && y.is_finite() {
                    x_values.push(*x);
                    y_values.push(*y);
                }
            }
            if let Some(interval) = &segment.interval {
                y_values.extend(interval.lower.iter().copied().filter(|y| y.is_finite()));
                y_values.extend(interval.upper.iter().copied().filter(|y| y.is_finite()));
            }
        }
    }

    let (x_lower, x_upper) = sv_padded_domain(&x_values);
    let (y_lower, y_upper) = sv_padded_domain(&y_values);
    let horizontal = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(x_lower, x_upper)
        .range(
            SV_LINE_PLOT_PLOT_LEFT,
            SV_LINE_PLOT_PLOT_LEFT + SV_LINE_PLOT_PLOT_WIDTH,
        )
        .build()?;
    let vertical = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(y_lower, y_upper)
        .range(
            SV_LINE_PLOT_PLOT_TOP + SV_LINE_PLOT_PLOT_HEIGHT,
            SV_LINE_PLOT_PLOT_TOP,
        )
        .build()?;
    Ok((horizontal, vertical))
}

fn sv_padded_domain(values: &[f64]) -> (f64, f64) {
    let (mut lower, mut upper) = values
        .iter()
        .copied()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        });
    if !lower.is_finite() || !upper.is_finite() {
        return (0.0, 1.0);
    }
    if (upper - lower).abs() <= f64::EPSILON {
        let padding = if lower.abs() <= f64::EPSILON {
            1.0
        } else {
            lower.abs() * 0.1
        };
        lower -= padding;
        upper += padding;
    }
    (lower, upper)
}
