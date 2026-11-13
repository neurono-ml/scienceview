//! `SVImagePlot`: the built figure and the one operation span.

use super::builder::SVImagePlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVImageConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVImageFrame;
use super::render_output::SVImageRenderOutput;
use super::scene::SVImageScene;
use super::{SVImageExtent, SVImageSource, SVRasterDimensions};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// An image plot ready to render.
///
/// Built only by [`SVImagePlotBuilder`]: the direct constructor is private, so
/// the explicit backend rule cannot be bypassed. The backend kind is carried,
/// never dispatched in `build`.
#[derive(Clone)]
pub struct SVImagePlot<'data> {
    pub(super) configuration: SVImageConfiguration<'data>,
}

impl std::fmt::Debug for SVImagePlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVImagePlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("pixels", &self.configuration.number_of_pixels)
            .finish_non_exhaustive()
    }
}

impl<'data> SVImagePlot<'data> {
    /// Starts building an image plot.
    pub fn builder() -> SVImagePlotBuilder<'data> {
        SVImagePlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the intensity column.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of pixels one frame holds.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_pixels
    }

    /// Returns the raster source the plot draws.
    pub fn source(&self) -> &SVImageSource<'data> {
        &self.configuration.source
    }

    /// Returns the raster dimensions.
    pub fn dimensions(&self) -> SVRasterDimensions {
        self.configuration.source.dimensions()
    }

    /// Returns the configured data extent, when one was set.
    pub fn extent(&self) -> Option<SVImageExtent> {
        self.configuration.extent
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves the raster once, maps its values through the colour
    /// scale and never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVImageScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_pixels,
            || sv_compose_scene(configuration),
        )
    }

    /// Renders through the path the configured kind actually has.
    ///
    /// The configured kind is never ignored: `Svg` serializes the SVG document,
    /// `Native` rasterizes the native frame, and every other kind reports
    /// [`SVError::UnsupportedBackendCapability`] because the recipe has no
    /// serializer for it yet. A compiled-out backend reports
    /// [`SVError::BackendUnavailable`] instead of degrading silently.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVImageRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_pixels,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_dispatch_render(&scene)
            },
        )
    }
}

fn sv_dispatch_render(scene: &SVImageScene) -> Result<SVImageRenderOutput, SVError> {
    match scene.backend_kind() {
        SVBackendKind::Svg => sv_render_svg(scene),
        SVBackendKind::Native => sv_render_native(scene),
        backend_kind => Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: crate::sv_backend::SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_render_svg(scene: &SVImageScene) -> Result<SVImageRenderOutput, SVError> {
    Ok(SVImageRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVImageScene) -> Result<SVImageRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVImageScene) -> Result<SVImageRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVImageRenderOutput::Native(SVImageFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVImageScene) -> Result<SVImageRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}
