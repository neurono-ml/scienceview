//! Composing the current data window into a backend-neutral picture.

use super::super::composition::{SVStepPlotComposition, SVStepSeries};
use super::super::decorations::{SVStepDecorationInputs, sv_compose_decorations};
use super::super::extraction::{
    sv_collect_samples, sv_expand_step, sv_group_samples, sv_order_points,
};
use super::definition::SVStepPlot;
use scienceview_core::sv_decoration::SVDecorationStyle;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, sv_default_scale_palette};

const PLOT_LEFT: f64 = 72.0;
const PLOT_TOP: f64 = 56.0;
const PLOT_RIGHT_MARGIN: f64 = 24.0;
const PLOT_BOTTOM_MARGIN: f64 = 56.0;
const MINIMUM_PLOT_EXTENT: f64 = 10.0;

impl SVStepPlot<'_> {
    /// Composes the current data window into a backend-neutral picture.
    ///
    /// The source columns are resolved once, the samples are ordered and
    /// expanded into the requested step, and the scales and decorations are
    /// solved. The result owns its vertices, so it stays valid after the
    /// source window is released.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::EmptyInput`] when no finite observation remains and
    /// the kernel's configuration error when an aggregate or a scale cannot be
    /// honoured.
    pub fn compose(&self) -> Result<SVStepPlotComposition, SVError> {
        let (samples, representation) = sv_collect_samples(self)?;
        if samples.is_empty() {
            return Err(SVError::EmptyInput {
                operation: "step_plot",
            });
        }

        let palette = sv_default_scale_palette();
        let mut series = Vec::new();
        let mut point_count = 0_usize;
        let mut horizontal_minimum = f64::INFINITY;
        let mut horizontal_maximum = f64::NEG_INFINITY;
        let mut vertical_minimum = f64::INFINITY;
        let mut vertical_maximum = f64::NEG_INFINITY;

        for (index, group) in sv_group_samples(&samples).iter().enumerate() {
            let points = sv_order_points(
                &group.points,
                self.duplicate_policy,
                self.aggregate.as_ref(),
            )?;
            if points.is_empty() {
                continue;
            }
            point_count += points.len();
            for (x, y) in &points {
                horizontal_minimum = horizontal_minimum.min(*x);
                horizontal_maximum = horizontal_maximum.max(*x);
                vertical_minimum = vertical_minimum.min(*y);
                vertical_maximum = vertical_maximum.max(*y);
            }

            let (horizontal, vertical) = sv_expand_step(&points, self.step_position);
            let (key, label) = match group.code {
                Some(code) => (format!("series-{code}"), format!("series {code}")),
                None => ("series".to_string(), "value".to_string()),
            };
            series.push(SVStepSeries {
                key,
                label,
                colour: palette.colour_at_index(index),
                sample_horizontal: points.iter().map(|point| point.0).collect(),
                sample_vertical: points.iter().map(|point| point.1).collect(),
                horizontal,
                vertical,
            });
        }

        if series.is_empty() {
            return Err(SVError::EmptyInput {
                operation: "step_plot",
            });
        }

        let (horizontal_lower, horizontal_upper) =
            sv_padded_domain(horizontal_minimum, horizontal_maximum);
        let (vertical_lower, vertical_upper) = sv_padded_domain(vertical_minimum, vertical_maximum);
        let plot_area = sv_plot_area(self.viewport);

        let horizontal_scale: SVScale<'static> = SVScaleBuilder::new(self.x_scale_kind)
            .domain(horizontal_lower, horizontal_upper)
            .range(plot_area.x, plot_area.right())
            .build()?;
        let vertical_scale: SVScale<'static> = SVScaleBuilder::new(self.x_scale_kind)
            .domain(vertical_lower, vertical_upper)
            .range(plot_area.bottom(), plot_area.y)
            .build()?;

        // The axis kernel measures a scale in plot-local coordinates and adds
        // the plot origin when it places a tick, so the decoration scales span
        // `0..extent` while the mark scales span the absolute plot rectangle.
        let axis_horizontal: SVScale<'static> = SVScaleBuilder::new(self.x_scale_kind)
            .domain(horizontal_lower, horizontal_upper)
            .range(0.0, plot_area.width)
            .build()?;
        let axis_vertical: SVScale<'static> = SVScaleBuilder::new(self.x_scale_kind)
            .domain(vertical_lower, vertical_upper)
            .range(plot_area.height, 0.0)
            .build()?;

        let style = SVDecorationStyle::built_in();
        let (decorations_before_marks, decorations_after_marks) =
            sv_compose_decorations(&SVStepDecorationInputs {
                title: self.title.as_deref(),
                horizontal_title: &self.x,
                vertical_title: &self.y,
                horizontal_scale: &axis_horizontal,
                vertical_scale: &axis_vertical,
                plot: plot_area,
                viewport: self.viewport,
                series: &series,
                style: &style,
            })?;

        Ok(SVStepPlotComposition {
            horizontal_scale,
            vertical_scale,
            plot_area,
            viewport: self.viewport,
            series,
            line_width: self.line_width,
            dash: self.dash.clone(),
            markers: self.markers,
            decorations_before_marks,
            decorations_after_marks,
            representation,
            point_count,
            title: self.title.clone(),
            background: style.background(),
        })
    }
}

fn sv_padded_domain(lower: f64, upper: f64) -> (f64, f64) {
    if !lower.is_finite() || !upper.is_finite() {
        return (0.0, 1.0);
    }
    if lower == upper {
        return (lower - 0.5, upper + 0.5);
    }
    (lower, upper)
}

fn sv_plot_area(viewport: SVViewport) -> SVResolvedRectangle {
    let width = (viewport.width() - PLOT_LEFT - PLOT_RIGHT_MARGIN).max(MINIMUM_PLOT_EXTENT);
    let height = (viewport.height() - PLOT_TOP - PLOT_BOTTOM_MARGIN).max(MINIMUM_PLOT_EXTENT);
    SVResolvedRectangle::new(PLOT_LEFT, PLOT_TOP, width, height)
}
