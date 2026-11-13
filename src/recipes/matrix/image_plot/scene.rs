//! The backend-agnostic result of one image render.
//!
//! The scene owns the mapped RGBA buffer. The image mark borrows its pixels, so
//! the single mark primitive is rebuilt from that buffer when a serializer asks
//! for it: a self-referential scene would need an owning pointer and unsafe
//! code, which the recipe never uses.

use super::{SVImageExtent, SVImageInterpolation, SVRasterDimensions};
use crate::sv_backend::SVBackendKind;
use crate::sv_decoration::SVDecorationPrimitive;
use crate::sv_error::SVError;
use crate::sv_layout::{SVResolvedRectangle, SVViewport};
use crate::sv_mark::{SVImageRaster, SVMark, SVMarkGeometry, SVMarkRequest};
use crate::sv_scale::{SVColour, SVScale, SVScaleBuilder, SVScaleKind};

const SV_EMPTY_CHANNEL: &[f64] = &[];

/// Everything one image composition produced.
///
/// The scene is backend-agnostic, but it carries the kind it was composed for:
/// the recipe-level serializers exposed on it (`to_svg`, `to_native_frame`)
/// refuse a scene whose configured kind they do not serve.
#[derive(Clone, Debug, PartialEq)]
pub struct SVImageScene {
    pub(super) backend_kind: SVBackendKind,
    pub(super) viewport: SVViewport,
    pub(super) plot_area: SVResolvedRectangle,
    pub(super) image_rectangle: SVResolvedRectangle,
    pub(super) background: SVColour,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
    pub(super) decoration_primitives_before_marks: Vec<SVDecorationPrimitive>,
    pub(super) decoration_primitives_after_marks: Vec<SVDecorationPrimitive>,
    pub(super) raster: Vec<u8>,
    pub(super) dimensions: SVRasterDimensions,
    pub(super) interpolation: SVImageInterpolation,
    pub(super) opacity: f32,
    pub(super) extent: SVImageExtent,
}

impl SVImageScene {
    /// Returns the backend kind the scene was composed for.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.backend_kind
    }

    /// Returns the viewport the scene was solved for.
    pub fn viewport(&self) -> SVViewport {
        self.viewport
    }

    /// Returns the rectangle the axis frame occupies.
    pub fn plot_area(&self) -> SVResolvedRectangle {
        self.plot_area
    }

    /// Returns the rectangle the raster occupies after the aspect policy.
    pub fn image_rectangle(&self) -> SVResolvedRectangle {
        self.image_rectangle
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

    /// Returns the decorations drawn below the raster.
    pub fn decoration_primitives_before_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_before_marks
    }

    /// Returns the decorations drawn above the raster.
    pub fn decoration_primitives_after_marks(&self) -> &[SVDecorationPrimitive] {
        &self.decoration_primitives_after_marks
    }

    /// Returns the mapped RGBA pixels, row-major.
    pub fn raster(&self) -> &[u8] {
        &self.raster
    }

    /// Returns the raster dimensions.
    pub fn dimensions(&self) -> SVRasterDimensions {
        self.dimensions
    }

    /// Returns the sampling rule the raster is drawn with.
    pub fn interpolation(&self) -> SVImageInterpolation {
        self.interpolation
    }

    /// Returns the configured opacity.
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Returns the data extent the raster covers.
    pub fn extent(&self) -> SVImageExtent {
        self.extent
    }

    /// Returns the number of pixels one frame holds.
    pub fn number_of_pixels(&self) -> usize {
        self.dimensions.number_of_pixels()
    }

    /// Returns the colour of one stored pixel, or `None` past the raster.
    pub fn pixel_colour(&self, row: usize, column: usize) -> Option<SVColour> {
        if row >= self.dimensions.rows() || column >= self.dimensions.columns() {
            return None;
        }
        let offset = row
            .saturating_mul(self.dimensions.columns())
            .saturating_add(column)
            .saturating_mul(4);
        let pixel = self.raster.get(offset..offset.saturating_add(4))?;
        Some(SVColour::new(pixel[0], pixel[1], pixel[2], pixel[3]))
    }

    /// Returns the single image mark primitive, borrowing the owned buffer.
    ///
    /// # Errors
    ///
    /// Returns the mark kernel's configuration error when the raster cannot be
    /// drawn.
    pub fn mark_geometry(&self) -> Result<SVMarkGeometry<'_>, SVError> {
        let horizontal = sv_unit_scale()?;
        let vertical = sv_unit_scale()?;
        let raster = SVImageRaster::new(
            self.dimensions.columns(),
            self.dimensions.rows(),
            &self.raster,
        );
        let request = SVMarkRequest::new(
            crate::sv_data::SVColumnView::Dense(SV_EMPTY_CHANNEL),
            crate::sv_data::SVColumnView::Dense(SV_EMPTY_CHANNEL),
            &horizontal,
            &vertical,
            self.image_rectangle,
        )
        .with_image(raster);
        let mark: SVMark = SVMark::image()
            .interpolation(self.interpolation)
            .opacity(self.opacity)
            .into();
        mark.geometry(&request)
    }

    /// Returns the number of primitives the mark kernel produced.
    pub fn number_of_mark_primitives(&self) -> Result<usize, SVError> {
        Ok(self.mark_geometry()?.len())
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

        let geometry = self.mark_geometry()?;
        let mut document = scienceview_svg::SVSvgDocument::new(self.viewport)
            .with_title(
                self.title
                    .clone()
                    .unwrap_or_else(|| "image plot".to_string()),
            )
            .with_description(
                self.description
                    .clone()
                    .unwrap_or_else(|| "image plot".to_string()),
            )
            .with_background(self.background);
        document.push_decoration_primitives(&self.decoration_primitives_before_marks);
        document.push_mark_geometry(&geometry);
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

        let geometry = self.mark_geometry()?;
        let mut canvas = scienceview_native::SVRasterCanvas::new(&self.viewport);
        canvas.fill(self.background);
        canvas.draw_decoration_primitives(&self.decoration_primitives_before_marks);
        canvas.draw_mark_geometry(&geometry);
        canvas.draw_decoration_primitives(&self.decoration_primitives_after_marks);
        Ok(canvas.to_frame())
    }
}

fn sv_unit_scale() -> Result<SVScale<'static>, SVError> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 1.0)
        .range(0.0, 1.0)
        .build()
}
