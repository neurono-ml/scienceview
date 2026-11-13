//! The backend-agnostic result of one histogram render.
//!
//! The scene owns no notation: it carries the solved viewport and plot
//! rectangle, the decoration primitives before and after the bins, the mark
//! geometries and a data-space summary of every series. An SVG, native or
//! component serializer turns the same scene into its own artifact.

use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::SVColour;

/// The binned values of one series, in data space.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHistogramSeriesSummary {
    label: String,
    colour: SVColour,
    edges: Vec<f64>,
    heights: Vec<f64>,
}

impl SVHistogramSeriesSummary {
    pub(super) fn new(label: String, colour: SVColour, edges: Vec<f64>, heights: Vec<f64>) -> Self {
        Self {
            label,
            colour,
            edges,
            heights,
        }
    }

    /// Returns the series label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the series colour.
    pub fn colour(&self) -> SVColour {
        self.colour
    }

    /// Returns the bin edges of this series, one more than its heights.
    pub fn edges(&self) -> &[f64] {
        &self.edges
    }

    /// Returns the drawn height of every bin.
    pub fn heights(&self) -> &[f64] {
        &self.heights
    }
}

/// Everything one histogram composition produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVHistogramScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) series: Vec<SVHistogramSeriesSummary>,
    pub(super) number_of_bars: usize,
}

impl SVHistogramScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the bins occupy.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the background colour.
    pub fn background(&self) -> SVColour {
        self.background
    }

    /// Returns the scene title, when the builder set one.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the accessible description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the decorations drawn below the bins: title, grid and border.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the mark geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the bins: axes and legend.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the binned values of every series.
    pub fn series(&self) -> &[SVHistogramSeriesSummary] {
        &self.series
    }

    /// Returns the number of series the bins are grouped into.
    pub fn number_of_series(&self) -> usize {
        self.series.len()
    }

    /// Returns the number of bars the scene draws.
    pub fn number_of_bars(&self) -> usize {
        self.number_of_bars
    }

    /// Returns the number of bins of the first series, or zero.
    pub fn number_of_bins(&self) -> usize {
        self.series.first().map_or(0, |series| series.heights.len())
    }

    /// Returns the bin edges of the first series, or an empty slice.
    pub fn bin_edges(&self) -> &[f64] {
        self.series
            .first()
            .map_or(&[], SVHistogramSeriesSummary::edges)
    }

    /// Returns the heights of the first series, or an empty slice.
    pub fn bin_heights(&self) -> &[f64] {
        self.series
            .first()
            .map_or(&[], SVHistogramSeriesSummary::heights)
    }

    /// Returns the number of primitives the mark kernel produced.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Serializes the scene as a self-contained SVG document.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] when the scene was
    /// composed for a kind other than `Svg`.
    #[cfg(feature = "render-svg")]
    pub fn to_svg(&self) -> Result<String, SVError> {
        if self.backend_kind != SVBackendKind::Svg {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: crate::sv_backend::SVBackendCapability::Export,
            });
        }

        let mut document = scienceview_svg::SVSvgDocument::new(self.viewport)
            .with_title(
                self.title
                    .clone()
                    .unwrap_or_else(|| "histogram".to_string()),
            )
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "histogram".to_string()),
            )
            .with_background(self.background);
        document.push_decoration_primitives(&self.decoration_primitives_before_marks);
        for geometry in &self.mark_geometries {
            document.push_mark_geometry(geometry);
        }
        document.push_decoration_primitives(&self.decoration_primitives_after_marks);
        Ok(document.to_svg())
    }

    /// Rasterizes the scene with the from-scratch native renderer.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedBackendCapability`] when the scene was
    /// composed for a kind other than `Native`.
    #[cfg(feature = "render-native")]
    pub fn to_native_frame(&self) -> Result<scienceview_native::SVNativeFrame, SVError> {
        if self.backend_kind != SVBackendKind::Native {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: self.backend_kind,
                capability: crate::sv_backend::SVBackendCapability::Presentation,
            });
        }

        let mut canvas = scienceview_native::SVRasterCanvas::new(&self.viewport);
        canvas.fill(self.background);
        canvas.draw_decoration_primitives(&self.decoration_primitives_before_marks);
        for geometry in &self.mark_geometries {
            canvas.draw_mark_geometry(geometry);
        }
        canvas.draw_decoration_primitives(&self.decoration_primitives_after_marks);
        Ok(canvas.to_frame())
    }
}
