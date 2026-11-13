//! The public operations and their backend dispatch.

use super::composition::sv_compose_scene;
use super::core_implementation::{SVRugPlot, sv_record_resolved_ticks};
#[cfg(feature = "render-native")]
use super::render_output::SVRugFrame;
use super::render_output::SVRugRenderOutput;
use super::scene::SVRugScene;
use crate::sv_backend::{SVBackendCapability, SVBackendKind};
use crate::sv_error::SVError;
use crate::sv_observability::sv_run_operation;
use std::path::Path;

impl SVRugPlot<'_> {
    /// Renders through the path the configured kind actually has.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVRugRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_ticks(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }

    /// Re-renders the current window after `appended_observations` new samples.
    ///
    /// The recipe owns no incremental buffer: `appended_observations` documents
    /// how many samples the source appended since the previous render, and the
    /// operation resolves the source again so the newest ticks appear. A
    /// backend that can append without recomposing the history consumes the
    /// count; this serializer recomposes the scene.
    ///
    /// # Errors
    ///
    /// Same as [`SVRugPlot::render`].
    pub fn update(&self, appended_observations: usize) -> Result<SVRugRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "update",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let _ = appended_observations;
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_ticks(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// # Errors
    ///
    /// Same as [`SVRugPlot::render`], plus [`SVError::BackendFailure`] when the
    /// destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let output = self.render()?;
        std::fs::write(destination, output.bytes()).map_err(|error| SVError::BackendFailure {
            backend_kind: self.configuration.backend_kind,
            operation: "save",
            message: error.to_string(),
        })
    }
}

fn sv_dispatch_render(scene: &SVRugScene) -> Result<SVRugRenderOutput, SVError> {
    match scene.backend_kind() {
        SVBackendKind::Svg => sv_render_svg(scene),
        SVBackendKind::Native => sv_render_native(scene),
        backend_kind => Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        }),
    }
}

#[cfg(feature = "render-svg")]
fn sv_render_svg(scene: &SVRugScene) -> Result<SVRugRenderOutput, SVError> {
    Ok(SVRugRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVRugScene) -> Result<SVRugRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVRugScene) -> Result<SVRugRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVRugRenderOutput::Native(SVRugFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVRugScene) -> Result<SVRugRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}
