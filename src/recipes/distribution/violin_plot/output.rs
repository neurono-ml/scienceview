//! What a violin render produces: display-space geometry plus decorations.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVMarkGeometry;
use scienceview_core::sv_scale::SVColour;

use super::statistics::SVViolinSummary;

/// The backend-agnostic result of rendering one violin plot.
#[derive(Clone, Debug, PartialEq)]
pub struct SVViolinRenderOutput {
    backend_kind: SVBackendKind,
    representation: SVRepresentation,
    number_of_points: usize,
    number_of_categories: usize,
    is_approximate: bool,
    viewport: SVViewport,
    plot_area: SVResolvedRectangle,
    title: Option<String>,
    summaries: Vec<SVViolinSummary>,
    series: Vec<(String, SVColour)>,
    geometries: Vec<SVMarkGeometry<'static>>,
    decorations: Vec<SVDecorationPrimitive>,
}

impl SVViolinRenderOutput {
    /// Assembles one render result.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        backend_kind: SVBackendKind,
        representation: SVRepresentation,
        number_of_points: usize,
        number_of_categories: usize,
        is_approximate: bool,
        viewport: SVViewport,
        plot_area: SVResolvedRectangle,
        title: Option<String>,
        summaries: Vec<SVViolinSummary>,
        series: Vec<(String, SVColour)>,
        geometries: Vec<SVMarkGeometry<'static>>,
        decorations: Vec<SVDecorationPrimitive>,
    ) -> Self {
        Self {
            backend_kind,
            representation,
            number_of_points,
            number_of_categories,
            is_approximate,
            viewport,
            plot_area,
            title,
            summaries,
            series,
            geometries,
            decorations,
        }
    }

    /// The backend the figure was configured for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// The representation the operation resolved.
    pub fn representation(&self) -> SVRepresentation {
        self.representation
    }

    /// The number of finite observations the estimates consumed.
    pub fn number_of_points(&self) -> usize {
        self.number_of_points
    }

    /// The number of categories the figure draws.
    pub fn number_of_categories(&self) -> usize {
        self.number_of_categories
    }

    /// Whether the estimate came from a streaming window and is approximate.
    pub fn is_approximate(&self) -> bool {
        self.is_approximate
    }

    /// The logical viewport the figure was laid out for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// The solved plot rectangle.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// The optional figure title.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// The density summaries, one per violin.
    pub fn summaries(&self) -> &[SVViolinSummary] {
        &self.summaries
    }

    /// The legend entries: one per series, in palette order.
    pub fn series(&self) -> &[(String, SVColour)] {
        &self.series
    }

    /// The violin geometry, one or more primitives per drawn violin.
    pub fn geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.geometries
    }

    /// The decoration primitives in draw order.
    pub fn decorations(&self) -> &[SVDecorationPrimitive] {
        &self.decorations
    }

    /// Serializes the result as one self-contained SVG document.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the figure was configured
    /// for a backend other than `Svg`.
    #[cfg(feature = "render-svg")]
    pub fn to_svg(&self) -> Result<String, SVError> {
        if self.backend_kind != SVBackendKind::Svg {
            return Err(sv_wrong_backend(
                "SVG",
                self.backend_kind,
                SVBackendKind::Svg,
            ));
        }
        let style = scienceview_core::sv_decoration::SVDecorationStyle::built_in();
        let mut document =
            scienceview_svg::SVSvgDocument::new(self.viewport).with_background(style.background());
        if let Some(title) = &self.title {
            document = document.with_title(title.clone());
        }
        for geometry in &self.geometries {
            document.push_mark_geometry(geometry);
        }
        for primitive in &self.decorations {
            document.push_decoration_primitives(std::slice::from_ref(primitive));
        }
        Ok(document.to_svg())
    }

    /// Rasterizes the result into one native frame.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the figure was configured
    /// for a backend other than `Native`.
    #[cfg(feature = "render-native")]
    pub fn to_native_frame(&self) -> Result<scienceview_native::SVNativeFrame, SVError> {
        if self.backend_kind != SVBackendKind::Native {
            return Err(sv_wrong_backend(
                "native",
                self.backend_kind,
                SVBackendKind::Native,
            ));
        }
        let style = scienceview_core::sv_decoration::SVDecorationStyle::built_in();
        let mut canvas = scienceview_native::SVRasterCanvas::new(&self.viewport);
        canvas.fill(style.background());
        for geometry in &self.geometries {
            canvas.draw_mark_geometry(geometry);
        }
        canvas.draw_decoration_primitives(&self.decorations);
        Ok(canvas.to_frame())
    }
}

/// Builds the error of asking a serializer for the wrong configured backend.
#[cfg(any(feature = "render-svg", feature = "render-native"))]
fn sv_wrong_backend(
    serializer: &str,
    configured: SVBackendKind,
    required: SVBackendKind,
) -> SVError {
    SVError::InvalidConfiguration {
        message: format!(
            "the `{configured}` backend has no {serializer} serializer; this figure was configured for `{configured}`, and the {serializer} serializer requires `{required}`"
        ),
    }
}
