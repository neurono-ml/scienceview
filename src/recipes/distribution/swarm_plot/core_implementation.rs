//! `SVSwarmPlot`: the built figure and the one operation span per render.

use super::builder::SVSwarmPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVSwarmConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVSwarmFrame;
use super::render_output::SVSwarmRenderOutput;
use super::scene::SVSwarmScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A swarm ready to render.
///
/// Built only by [`SVSwarmPlotBuilder`](super::SVSwarmPlotBuilder): the direct
/// constructor is private, so the explicit backend rule cannot be bypassed. The
/// backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVSwarmPlot<'data> {
    pub(super) configuration: SVSwarmConfiguration<'data>,
}

impl std::fmt::Debug for SVSwarmPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVSwarmPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("points", &self.configuration.number_of_points)
            .finish_non_exhaustive()
    }
}

impl<'data> SVSwarmPlot<'data> {
    /// Starts building a swarm.
    pub fn builder() -> SVSwarmPlotBuilder<'data> {
        SVSwarmPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the widest representation the configured columns carry.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the logical number of observations the columns expose.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_points
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves each column once, lays every category out through the
    /// swarm move and never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, move, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVSwarmScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_points,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_points(&scene);
                Ok(scene)
            },
        )
    }

    /// Renders through the path the configured kind actually has.
    ///
    /// `Svg` serializes the SVG document, `Native` rasterizes the native frame,
    /// and every other kind reports [`SVError::UnsupportedBackendCapability`]
    /// because the recipe has no serializer for it yet. A compiled-out backend
    /// reports [`SVError::BackendUnavailable`] instead of degrading silently.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVSwarmRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_points,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_points(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }
}

/// Records the mark count the current composition actually drew.
fn sv_record_resolved_points(scene: &SVSwarmScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(scene: &SVSwarmScene) -> Result<SVSwarmRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVSwarmScene) -> Result<SVSwarmRenderOutput, SVError> {
    Ok(SVSwarmRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVSwarmScene) -> Result<SVSwarmRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVSwarmScene) -> Result<SVSwarmRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVSwarmRenderOutput::Native(SVSwarmFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVSwarmScene) -> Result<SVSwarmRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}
