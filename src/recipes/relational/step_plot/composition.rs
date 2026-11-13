//! The backend-neutral result of composing the chart.

use scienceview_core::sv_data::{SVColumnView, SVRepresentation};
use scienceview_core::sv_decoration::SVDecorationPrimitive;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVLength, SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::{
    SVDashPattern, SVMark, SVMarkGeometry, SVMarkRequest, SVMarkTheme,
};
use scienceview_core::sv_scale::{SVColour, SVScale};

/// One step line: its identity, colour, sample positions and expanded vertices.
#[derive(Clone, Debug, PartialEq)]
pub struct SVStepSeries {
    pub(super) key: String,
    pub(super) label: String,
    pub(super) colour: SVColour,
    pub(super) sample_horizontal: Vec<f64>,
    pub(super) sample_vertical: Vec<f64>,
    pub(super) horizontal: Vec<f64>,
    pub(super) vertical: Vec<f64>,
}

impl SVStepSeries {
    /// Returns the stable key of the series.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the label the legend shows.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the series colour.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns the ordered sample positions, before the step expansion.
    pub fn sample_horizontal(&self) -> &[f64] {
        &self.sample_horizontal
    }

    /// Returns the ordered sample values, before the step expansion.
    pub fn sample_vertical(&self) -> &[f64] {
        &self.sample_vertical
    }

    /// Returns the expanded horizontal vertices, in data space.
    pub fn horizontal(&self) -> &[f64] {
        &self.horizontal
    }

    /// Returns the expanded vertical vertices, in data space.
    pub fn vertical(&self) -> &[f64] {
        &self.vertical
    }
}

/// A composed step plot, ready for any backend that serializes mark geometry.
///
/// The composition is backend-neutral on purpose: it owns the scales, the
/// expanded vertices and the decoration primitives, so the SVG backend (and a
/// later recipe-aware backend) serializes exactly the same picture without
/// recomputing the chart.
#[derive(Clone, Debug, PartialEq)]
pub struct SVStepPlotComposition {
    pub(super) horizontal_scale: SVScale<'static>,
    pub(super) vertical_scale: SVScale<'static>,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) viewport: SVViewport,
    pub(super) series: Vec<SVStepSeries>,
    pub(super) line_width: Option<SVLength>,
    pub(super) dash: Option<SVDashPattern>,
    pub(super) markers: bool,
    pub(super) decorations_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) decorations_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) representation: SVRepresentation,
    pub(super) point_count: usize,
    pub(super) title: Option<String>,
    pub(super) background: SVColour,
}

impl SVStepPlotComposition {
    /// Returns the step lines, in first-appearance series order.
    pub fn series(&self) -> &[SVStepSeries] {
        &self.series
    }

    /// Returns the number of step lines.
    pub fn number_of_series(&self) -> usize {
        self.series.len()
    }

    /// Returns the representation the data arrived in.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// Returns the number of observations the step lines draw.
    pub fn point_count(&self) -> usize {
        self.point_count
    }

    /// Returns the logical canvas size.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the title the document announces.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the fixed background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the decorations drawn beneath the marks (title and grid).
    pub fn decorations_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_before_marks
    }

    /// Returns the decorations drawn above the marks (axes and legend).
    pub fn decorations_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_after_marks
    }

    /// Produces one geometry per step line, plus one dot geometry per line
    /// when markers are enabled. Markers sit on the samples, never on the
    /// expanded corners.
    ///
    /// # Errors
    ///
    /// Returns the mark kernel's configuration error when an option or a
    /// scale cannot be honoured.
    pub fn mark_geometries(&self) -> Result<Vec<SVMarkGeometry<'_>>, SVError> {
        let theme = SVMarkTheme::default();
        let mut geometries = Vec::with_capacity(self.series.len() * 2);

        for series in &self.series {
            let mut line = SVMark::line().colour(series.colour);
            if let Some(width) = self.line_width {
                line = line.width(width);
            }
            if let Some(dash) = &self.dash {
                line = line.dash(dash.clone());
            }
            let mark: SVMark = line.into();
            let request = self.sv_request(&series.horizontal, &series.vertical, &theme);
            geometries.push(mark.geometry(&request)?);

            if self.markers {
                let dot: SVMark = SVMark::dot().colour(series.colour).into();
                let request =
                    self.sv_request(&series.sample_horizontal, &series.sample_vertical, &theme);
                geometries.push(dot.geometry(&request)?);
            }
        }

        Ok(geometries)
    }

    fn sv_request<'values, 'scale>(
        &'scale self,
        horizontal: &'values [f64],
        vertical: &'values [f64],
        theme: &SVMarkTheme,
    ) -> SVMarkRequest<'values, 'scale> {
        SVMarkRequest::new(
            SVColumnView::Dense(horizontal),
            SVColumnView::Dense(vertical),
            &self.horizontal_scale,
            &self.vertical_scale,
            self.plot_area,
        )
        .with_theme(theme.clone())
    }
}
