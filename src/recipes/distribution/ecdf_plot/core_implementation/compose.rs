//! Composing the resolved curves into a backend-neutral picture.

use super::super::composition::{SVEcdfPlotComposition, SVEcdfSeries};
use super::super::data_contract::{SVEcdfData, sv_resolve_ecdf_data};
use super::super::decorations::{SVEcdfDecorationInputs, sv_compose_decorations};
use super::definition::SVECdfPlot;
use scienceview_core::sv_decoration::SVDecorationStyle;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::{SVScale, SVScaleBuilder, SVScaleKind, sv_default_scale_palette};

const PLOT_LEFT: f64 = 72.0;
const PLOT_TOP: f64 = 56.0;
const PLOT_RIGHT_MARGIN: f64 = 24.0;
const PLOT_BOTTOM_MARGIN: f64 = 56.0;
const MINIMUM_PLOT_EXTENT: f64 = 10.0;

impl SVECdfPlot<'_> {
    /// Composes the current data window into a backend-neutral picture.
    ///
    /// The source columns are resolved once, the kernel derives the cumulative
    /// probabilities, the step vertices are expanded exactly (horizontal then
    /// vertical) and the scales and decorations are solved. The result owns its
    /// vertices, so it stays valid after the source window is released.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::EmptyInput`] when no finite observation remains and
    /// the kernel's configuration error when a scale cannot be honoured.
    pub fn compose(&self) -> Result<SVEcdfPlotComposition, SVError> {
        let data = sv_resolve_ecdf_data(self)?;
        self.sv_compose_data(&data)
    }

    pub(crate) fn sv_compose_data(
        &self,
        data: &SVEcdfData,
    ) -> Result<SVEcdfPlotComposition, SVError> {
        if data.curves.is_empty() {
            return Err(SVError::EmptyInput {
                operation: "ecdf_plot",
            });
        }

        let palette = sv_default_scale_palette();
        let mut series = Vec::with_capacity(data.curves.len());
        let mut value_minimum = f64::INFINITY;
        let mut value_maximum = f64::NEG_INFINITY;
        let mut probability_maximum: f64 = 0.0;
        let mut observation_count = 0_usize;

        for (index, curve) in data.curves.iter().enumerate() {
            observation_count = observation_count.max(curve.observation_count);
            for value in &curve.values {
                value_minimum = value_minimum.min(*value);
                value_maximum = value_maximum.max(*value);
            }
            for probability in &curve.probabilities {
                probability_maximum = probability_maximum.max(*probability);
            }
            series.push(SVEcdfSeries::from_curve(
                palette.colour_at_index(index),
                curve,
                self.orient,
                self.normalise,
                self.complementary,
            ));
        }

        let total = if self.normalise {
            1.0
        } else {
            observation_count as f64
        };
        let probability_upper = probability_maximum.max(total).max(0.0);
        let (value_lower, value_upper) = sv_padded_domain(value_minimum, value_maximum);
        let plot_area = sv_plot_area(self.viewport);

        let (horizontal_lower, horizontal_upper, vertical_lower, vertical_upper) = match self.orient
        {
            SVBarOrientation::Vertical => (value_lower, value_upper, 0.0, probability_upper),
            SVBarOrientation::Horizontal => (0.0, probability_upper, value_lower, value_upper),
        };

        let horizontal_scale: SVScale<'static> = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(horizontal_lower, horizontal_upper)
            .range(plot_area.x, plot_area.right())
            .build()?;
        let vertical_scale: SVScale<'static> = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(vertical_lower, vertical_upper)
            .range(plot_area.bottom(), plot_area.y)
            .build()?;

        // The axis kernel measures a scale in plot-local coordinates and adds
        // the plot origin when it places a tick, so the decoration scales span
        // `0..extent` while the mark scales span the absolute plot rectangle.
        let axis_horizontal: SVScale<'static> = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(horizontal_lower, horizontal_upper)
            .range(0.0, plot_area.width)
            .build()?;
        let axis_vertical: SVScale<'static> = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(vertical_lower, vertical_upper)
            .range(plot_area.height, 0.0)
            .build()?;

        let (horizontal_title, vertical_title) = match self.orient {
            SVBarOrientation::Vertical => (self.value.as_str(), "cumulative probability"),
            SVBarOrientation::Horizontal => ("cumulative probability", self.value.as_str()),
        };
        let style = SVDecorationStyle::built_in();
        let (decorations_before_marks, decorations_after_marks) =
            sv_compose_decorations(&SVEcdfDecorationInputs {
                title: self.title.as_deref(),
                horizontal_title,
                vertical_title,
                horizontal_scale: &axis_horizontal,
                vertical_scale: &axis_vertical,
                plot: plot_area,
                viewport: self.viewport,
                series: &series,
                style: &style,
            })?;

        Ok(SVEcdfPlotComposition {
            backend_kind: self.backend_kind,
            representation: data.representation,
            point_count: data.point_count,
            viewport: self.viewport,
            plot_area,
            horizontal_scale,
            vertical_scale,
            series,
            line_width: self.line_width,
            orientation: self.orient,
            decorations_before_marks,
            decorations_after_marks,
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
