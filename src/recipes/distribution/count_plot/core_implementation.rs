//! `SVCountPlot`: the built figure and its operations.

use super::builder::SVCountPlotBuilder;
use super::composition::sv_compose_scene;
#[cfg(feature = "render-native")]
use super::render_output::SVCountFrame;
use super::render_output::SVCountRenderOutput;
use super::scene::SVCountScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;
use crate::sv_stat::{SVCountOrder, SVNormalisation};

use super::configuration::SVCountConfiguration;

/// A count plot ready to render.
///
/// Built only by [`SVCountPlotBuilder`](super::SVCountPlotBuilder): the direct
/// constructor is private, so the explicit backend rule cannot be bypassed.
/// The backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVCountPlot<'data> {
    pub(super) configuration: SVCountConfiguration<'data>,
}

impl std::fmt::Debug for SVCountPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVCountPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("observations", &self.configuration.number_of_observations)
            .field("normalisation", &self.configuration.normalisation)
            .finish_non_exhaustive()
    }
}

impl<'data> SVCountPlot<'data> {
    /// Starts building a count plot.
    pub fn builder() -> SVCountPlotBuilder<'data> {
        SVCountPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the input columns.
    pub fn representation(&self) -> crate::sv_data::SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of observations `build` probed.
    pub fn number_of_observations(&self) -> usize {
        self.configuration.number_of_observations
    }

    /// Returns the configured normalisation.
    pub fn normalisation(&self) -> SVNormalisation {
        self.configuration.normalisation
    }

    /// Returns the configured category order.
    pub fn order(&self) -> SVCountOrder {
        self.configuration.order
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The scene carries the configured kind but no notation: an SVG, native
    /// or component serializer turns the same viewport, decorations and mark
    /// geometries into its own artifact. The call resolves the columns once and
    /// never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, counting, scale or mark error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVCountScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_drawn_bars(&scene);
                Ok(scene)
            },
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
    pub fn render(&self) -> Result<SVCountRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_drawn_bars(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }

    /// Re-renders the current window after `appended_points` new samples.
    ///
    /// The recipe owns no incremental count buffer: the source is resolved
    /// again and the counts are recomposed from the window it now carries. A
    /// backend-level incremental update is deferred; this operation keeps the
    /// streaming path prompt and records the current window.
    ///
    /// # Errors
    ///
    /// Same as [`SVCountPlot::render`].
    pub fn update(&self, appended_points: usize) -> Result<SVCountRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "update",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let _ = appended_points;
                let scene = sv_compose_scene(configuration)?;
                sv_record_drawn_bars(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }
}

fn sv_record_drawn_bars(scene: &SVCountScene) {
    tracing::Span::current().record("points", scene.number_of_bars());
}

fn sv_dispatch_render(scene: &SVCountScene) -> Result<SVCountRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVCountScene) -> Result<SVCountRenderOutput, SVError> {
    Ok(SVCountRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVCountScene) -> Result<SVCountRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVCountScene) -> Result<SVCountRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVCountRenderOutput::Native(SVCountFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVCountScene) -> Result<SVCountRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}
