//! Serializing one scene to the configured backend.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind, SVRenderOutput};
use scienceview_core::sv_error::SVError;

use super::super::rendering::SVAreaRendering;

/// Dispatches the composed scene to the serializer of the configured kind.
///
/// The scene contract is backend-agnostic on purpose: [`SVAreaRendering`]
/// carries the viewport, the background, and the decoration and mark
/// primitives in draw order. A serializer only replays those lists.
pub(crate) fn sv_serialize(
    backend_kind: SVBackendKind,
    rendering: &SVAreaRendering,
) -> Result<SVRenderOutput, SVError> {
    let bytes = match backend_kind {
        SVBackendKind::Svg => sv_svg_bytes(rendering)?,
        SVBackendKind::Native => sv_native_bytes(rendering)?,
        other => {
            return Err(SVError::UnsupportedBackendCapability {
                backend_kind: other,
                capability: SVBackendCapability::Export,
            });
        }
    };
    Ok(SVRenderOutput {
        backend_kind,
        bytes,
    })
}

/// Returns the bytes an export writes for the configured kind.
pub(crate) fn sv_export(
    backend_kind: SVBackendKind,
    rendering: &SVAreaRendering,
) -> Result<Vec<u8>, SVError> {
    match backend_kind {
        SVBackendKind::Svg => sv_svg_bytes(rendering),
        other => Err(SVError::UnsupportedBackendCapability {
            backend_kind: other,
            capability: SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_svg_bytes(rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    Ok(sv_svg_document(rendering))
}

#[cfg(not(feature = "render-svg"))]
fn sv_svg_bytes(_rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Svg,
    })
}

#[cfg(feature = "render-native")]
fn sv_native_bytes(rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    let mut canvas = scienceview_native::SVRasterCanvas::new(&rendering.viewport());
    canvas.fill(rendering.background());
    canvas.draw_decoration_primitives(rendering.grid_and_frame());
    for primitive in rendering.primitives() {
        canvas.draw_mark_primitive(primitive);
    }
    canvas.draw_decoration_primitives(rendering.axes_and_title());
    Ok(canvas.to_frame().into_pixels())
}

#[cfg(not(feature = "render-native"))]
fn sv_native_bytes(_rendering: &SVAreaRendering) -> Result<Vec<u8>, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Native,
    })
}

#[cfg(feature = "render-svg")]
fn sv_svg_document(rendering: &SVAreaRendering) -> Vec<u8> {
    let mut document = scienceview_svg::SVSvgDocument::new(rendering.viewport())
        .with_background(rendering.background());
    if let Some(title) = rendering.title() {
        document = document.with_title(title);
    }
    document.push_decoration_primitives(rendering.grid_and_frame());
    for primitive in rendering.primitives() {
        document.push_mark_primitive(primitive);
    }
    document.push_decoration_primitives(rendering.axes_and_title());
    document.to_bytes()
}
