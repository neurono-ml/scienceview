//! The public composed scene of one contour render.

use super::super::field::SVContourField;
use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::SVMarkGeometry;
use crate::sv_scale::{SVColour, SVContinuousColourScale, SVScale};

/// Everything one contour composition produced.
#[derive(Clone)]
pub struct SVContourComposition {
    pub(super) backend_kind: SVBackendKind,
    pub(super) representation: crate::sv_data::SVRepresentation,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) horizontal_scale: SVScale<'static>,
    pub(super) vertical_scale: SVScale<'static>,
    pub(super) colour_scale: SVContinuousColourScale,
    pub(super) field: SVContourField,
    pub(super) level_thresholds: Vec<f64>,
    pub(super) decorations_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) decorations_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) mark_geometries: Vec<SVMarkGeometry<'static>>,
    pub(super) number_of_drawn_marks: usize,
}

impl std::fmt::Debug for SVContourComposition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVContourComposition")
            .field("backend_kind", &self.backend_kind)
            .field("field", &self.field)
            .field("levels", &self.level_thresholds.len())
            .field("number_of_drawn_marks", &self.number_of_drawn_marks)
            .finish_non_exhaustive()
    }
}

impl SVContourComposition {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the dominant representation of the bound columns.
    pub fn representation(&self) -> crate::sv_data::SVRepresentation {
        self.representation
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the contours occupy.
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

    /// Returns the horizontal scale.
    pub fn horizontal_scale(&self) -> &SVScale<'static> {
        &self.horizontal_scale
    }

    /// Returns the vertical scale.
    pub fn vertical_scale(&self) -> &SVScale<'static> {
        &self.vertical_scale
    }

    /// Returns the continuous colour scale the levels use.
    pub fn colour_scale(&self) -> &SVContinuousColourScale {
        &self.colour_scale
    }

    /// Returns the sampled field the contours were extracted from.
    pub fn field(&self) -> &SVContourField {
        &self.field
    }

    /// Returns the contour thresholds, in increasing order.
    pub fn level_thresholds(&self) -> &[f64] {
        &self.level_thresholds
    }

    /// Returns the number of contour levels.
    pub fn number_of_levels(&self) -> usize {
        self.level_thresholds.len()
    }

    /// Returns the decorations drawn below the contours.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_before_marks
    }

    /// Returns the decorations drawn above the contours.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decorations_after_marks
    }

    /// Visits every contour geometry in draw order.
    ///
    /// # Errors
    ///
    /// Contour geometry is prebuilt, so the callback never fails and the result
    /// is always `Ok`; the signature matches the other recipes so a backend can
    /// treat every composition alike.
    pub fn visit_mark_geometries(
        &self,
        mut visitor: impl FnMut(&SVMarkGeometry<'static>),
    ) -> Result<(), SVError> {
        for geometry in &self.mark_geometries {
            visitor(geometry);
        }
        Ok(())
    }

    /// Returns the number of contour primitives the scene draws.
    pub fn number_of_drawn_marks(&self) -> usize {
        self.number_of_drawn_marks
    }

    /// Serializes the composition as a self-contained SVG document.
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
            .with_title(self.title.as_deref().unwrap_or("contour plot"))
            .with_description("Contour plot generated by scienceview")
            .with_background(self.background);
        document.push_decoration_primitives(&self.decorations_before_marks);
        for geometry in &self.mark_geometries {
            document.push_mark_geometry(geometry);
        }
        document.push_decoration_primitives(&self.decorations_after_marks);
        Ok(document.to_svg())
    }

    /// Rasterizes the composition with the native renderer.
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
        canvas.draw_decoration_primitives(&self.decorations_before_marks);
        for geometry in &self.mark_geometries {
            canvas.draw_mark_geometry(geometry);
        }
        canvas.draw_decoration_primitives(&self.decorations_after_marks);
        Ok(canvas.to_frame())
    }
}
