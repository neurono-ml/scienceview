//! The backend-agnostic result of one joint render.
//!
//! The scene owns no notation: it carries the solved viewport and the three
//! panel rectangles, the decoration primitives before and after the marks, and
//! the mark geometries of the central panel plus the two marginals. An SVG,
//! native or component backend serializes the same scene without re-deriving a
//! position.

use super::{SVJointKind, SVJointMarginal};
use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVColour, SVScale};

/// Everything one joint composition produced.
///
/// The scene is backend-agnostic, but it carries the kind it was composed for:
/// the recipe-level serializers exposed on it (`to_svg`, `to_native_frame`)
/// refuse a scene whose configured kind they do not serve, so a serializer can
/// never be applied to the wrong backend by accident.
#[derive(Clone, Debug, PartialEq)]
pub struct SVJointScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) central_panel: SVResolvedRectangle,
    pub(super) horizontal_marginal_panel: SVResolvedRectangle,
    pub(super) vertical_marginal_panel: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) horizontal_scale: SVScale<'static>,
    pub(super) vertical_scale: SVScale<'static>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) legend_labels: Vec<String>,
    pub(super) number_of_series: usize,
    pub(super) number_of_marks: usize,
    pub(super) kind: SVJointKind,
    pub(super) marginal_kind: SVJointMarginal,
}

impl SVJointScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle of the central bivariate panel.
    pub fn central_panel(&self) -> SVResolvedRectangle {
        self.central_panel
    }

    /// Returns the rectangle of the top marginal panel.
    pub fn horizontal_marginal_panel(&self) -> SVResolvedRectangle {
        self.horizontal_marginal_panel
    }

    /// Returns the rectangle of the right marginal panel.
    pub fn vertical_marginal_panel(&self) -> SVResolvedRectangle {
        self.vertical_marginal_panel
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

    /// Returns the horizontal scale shared by the central panel and the top
    /// marginal.
    pub fn horizontal_scale(&self) -> &SVScale<'static> {
        &self.horizontal_scale
    }

    /// Returns the vertical scale shared by the central panel and the right
    /// marginal.
    pub fn vertical_scale(&self) -> &SVScale<'static> {
        &self.vertical_scale
    }

    /// Returns the decorations drawn below the marks: grid and panel frames.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the mark geometries of all three panels in draw order.
    pub fn mark_geometries(&self) -> &[SVMarkGeometry<'static>] {
        &self.mark_geometries
    }

    /// Returns the decorations drawn above the marks: axes, title and legend.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the number of logical marks the scene draws.
    pub fn number_of_marks(&self) -> usize {
        self.number_of_marks
    }

    /// Returns the number of primitives the mark kernels produced.
    pub fn number_of_mark_primitives(&self) -> usize {
        self.mark_geometries.iter().map(SVMarkGeometry::len).sum()
    }

    /// Returns the number of series the central panel is grouped into.
    pub fn number_of_series(&self) -> usize {
        self.number_of_series
    }

    /// Returns the legend entries in assignment order.
    pub fn legend_labels(&self) -> &[String] {
        &self.legend_labels
    }

    /// Returns the central form the scene drew.
    pub fn kind(&self) -> SVJointKind {
        self.kind
    }

    /// Returns the marginal form the scene drew.
    pub fn marginal_kind(&self) -> SVJointMarginal {
        self.marginal_kind
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
                    .unwrap_or_else(|| "joint plot".to_string()),
            )
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "joint plot".to_string()),
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
