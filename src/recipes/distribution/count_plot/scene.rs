//! The backend-agnostic result of one count-plot composition.
//!
//! The scene owns no notation: it carries the solved viewport and plot
//! rectangle, the decoration primitives before and after the bars, the mark
//! geometries and a data-space summary of every series. An SVG, native or
//! component serializer turns the same scene into its own artifact.

use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::SVColour;

/// The counted values of one series, in data space.
#[derive(Clone, Debug, PartialEq)]
pub struct SVCountSeriesSummary {
    label: String,
    colour: SVColour,
    counts: Vec<f64>,
}

impl SVCountSeriesSummary {
    pub(super) fn new(label: String, colour: SVColour, counts: Vec<f64>) -> Self {
        Self {
            label,
            colour,
            counts,
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

    /// Returns the drawn value of every category, in axis order.
    pub fn counts(&self) -> &[f64] {
        &self.counts
    }
}

/// Everything one count-plot composition produced.
#[derive(Clone, Debug, PartialEq)]
pub struct SVCountScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) categories: Vec<f64>,
    pub(super) category_labels: Vec<String>,
    pub(super) totals: Vec<f64>,
    pub(super) series: Vec<SVCountSeriesSummary>,
    pub(super) number_of_bars: usize,
}

impl SVCountScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the bars occupy.
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

    /// Returns the decorations drawn below the bars: title, grid and border.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the mark geometries in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the bars: axes and legend.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the category codes, in axis order.
    pub fn categories(&self) -> &[f64] {
        &self.categories
    }

    /// Returns the display label of every category, in axis order.
    pub fn category_labels(&self) -> &[String] {
        &self.category_labels
    }

    /// Returns the counted value of every series, in axis order.
    pub fn series(&self) -> &[SVCountSeriesSummary] {
        &self.series
    }

    /// Returns the number of categories the axis carries.
    pub fn number_of_categories(&self) -> usize {
        self.categories.len()
    }

    /// Returns the number of series the categories are split into.
    pub fn number_of_series(&self) -> usize {
        self.series.len()
    }

    /// Returns the number of bars the scene draws.
    pub fn number_of_bars(&self) -> usize {
        self.number_of_bars
    }

    /// Returns the counted value of every category, summed over the series.
    pub fn counts(&self) -> &[f64] {
        &self.totals
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
                    .unwrap_or_else(|| "count plot".to_string()),
            )
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "count plot".to_string()),
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
