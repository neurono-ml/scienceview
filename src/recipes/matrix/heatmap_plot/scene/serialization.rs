//! Serializing the scene through the backend that produced it.

use super::SVHeatmapScene;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_backend::SVBackendKind;
#[cfg(any(feature = "render-svg", feature = "render-native"))]
use crate::sv_error::SVError;

impl SVHeatmapScene {
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
            .with_title(self.title.clone().unwrap_or_else(|| "heatmap".to_string()))
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "heatmap".to_string()),
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
