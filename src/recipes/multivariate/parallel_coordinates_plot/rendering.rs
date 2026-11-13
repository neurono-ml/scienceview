//! Dispatching one composed parallel-coordinates scene to its backend.

use super::output::SVParallelCoordinatesRenderOutput;
use crate::sv_backend::{SVBackendCapability, SVBackendKind, SVRenderOutput};
use crate::sv_error::SVError;

pub(super) fn sv_dispatch_render(
    backend_kind: SVBackendKind,
    output: &SVParallelCoordinatesRenderOutput,
) -> Result<SVRenderOutput, SVError> {
    match backend_kind {
        SVBackendKind::Svg => sv_render_svg(output),
        SVBackendKind::Native => sv_render_native(output),
        backend_kind => Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_render_svg(output: &SVParallelCoordinatesRenderOutput) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind: output.backend_kind(),
        bytes: output.to_svg()?.into_bytes(),
    })
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(output: &SVParallelCoordinatesRenderOutput) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: output.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(output: &SVParallelCoordinatesRenderOutput) -> Result<SVRenderOutput, SVError> {
    Ok(SVRenderOutput {
        backend_kind: output.backend_kind(),
        bytes: output.to_native_frame()?.into_pixels(),
    })
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(output: &SVParallelCoordinatesRenderOutput) -> Result<SVRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: output.backend_kind(),
    })
}
