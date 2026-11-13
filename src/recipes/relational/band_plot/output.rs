//! What a band render produces: display-space geometry plus decorations.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVRepresentation;
use scienceview_core::sv_decoration::SVDecorationPrimitive;
#[cfg(feature = "render-svg")]
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::{SVResolvedRectangle, SVViewport};
use scienceview_core::sv_mark::SVMarkGeometry;

/// The backend-agnostic result of rendering one band plot.
///
/// The geometry is already placed in logical pixels through the scale kernel;
/// a backend only serializes it. Keeping the output free of any backend type is
/// what lets the example program write the very same geometry with the SVG
/// document surface.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBandRenderOutput {
    backend_kind: SVBackendKind,
    representation: SVRepresentation,
    number_of_points: usize,
    is_approximate: bool,
    viewport: SVViewport,
    plot_area: SVResolvedRectangle,
    title: Option<String>,
    geometries: Vec<SVMarkGeometry<'static>>,
    decorations: Vec<SVDecorationPrimitive>,
}

impl SVBandRenderOutput {
    /// Assembles one render result.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        backend_kind: SVBackendKind,
        representation: SVRepresentation,
        number_of_points: usize,
        is_approximate: bool,
        viewport: SVViewport,
        plot_area: SVResolvedRectangle,
        title: Option<String>,
        geometries: Vec<SVMarkGeometry<'static>>,
        decorations: Vec<SVDecorationPrimitive>,
    ) -> Self {
        Self {
            backend_kind,
            representation,
            number_of_points,
            is_approximate,
            viewport,
            plot_area,
            title,
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

    /// The number of drawn band vertices.
    pub fn number_of_points(&self) -> usize {
        self.number_of_points
    }

    /// Whether the band came from a streaming window and is therefore an
    /// approximation of the full data (FDR-105 acceptance).
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

    /// The band geometry, one or more primitives per drawn band.
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
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the `{}` backend has no band-plot serializer yet; select `Svg`",
                    self.backend_kind
                ),
            });
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
}
