//! `SVPairPlot`: validation, the built figure and its operation spans.

use super::builder::SVPairPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVPairConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVPairFrame;
use super::render_output::SVPairRenderOutput;
use super::scene::SVPairScene;
use super::{SVPairDiagonalKind, SVPairOffDiagonalKind};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A pair grid ready to render.
///
/// Built only by [`SVPairPlotBuilder`](super::SVPairPlotBuilder): the direct
/// constructor is private, so the explicit backend rule cannot be bypassed.
/// The backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVPairPlot<'data> {
    pub(super) configuration: SVPairConfiguration<'data>,
}

impl std::fmt::Debug for SVPairPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVPairPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("points", &self.configuration.number_of_points)
            .field("x_variables", &self.configuration.x_variables)
            .field("y_variables", &self.configuration.y_variables)
            .finish_non_exhaustive()
    }
}

impl<'data> SVPairPlot<'data> {
    /// Starts building a pair grid.
    pub fn builder() -> SVPairPlotBuilder<'data> {
        SVPairPlotBuilder::new()
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
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of observations `build` probed.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_points
    }

    /// Returns the variables placed along the horizontal axes.
    pub fn x_variables(&self) -> &[String] {
        &self.configuration.x_variables
    }

    /// Returns the variables placed along the vertical axes.
    pub fn y_variables(&self) -> &[String] {
        &self.configuration.y_variables
    }

    /// Returns the hue column, when one was configured.
    pub fn hue(&self) -> Option<&str> {
        self.configuration.hue.as_deref()
    }

    /// Returns the diagonal kind as configured.
    pub fn diagonal_kind(&self) -> SVPairDiagonalKind {
        self.configuration.diagonal_kind
    }

    /// Returns the off-diagonal kind as configured.
    pub fn off_diagonal_kind(&self) -> SVPairOffDiagonalKind {
        self.configuration.off_diagonal_kind
    }

    /// Composes the backend-agnostic scene.
    ///
    /// # Errors
    ///
    /// Returns the column, stat, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVPairScene, SVError> {
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
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVPairRenderOutput, SVError> {
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

    /// Re-renders the current window after `appended_points` new samples.
    ///
    /// The recipe owns no incremental cell buffer: every cell is recomposed
    /// from the window the source now carries.
    ///
    /// # Errors
    ///
    /// Same as [`SVPairPlot::render`].
    pub fn update(&self, appended_points: usize) -> Result<SVPairRenderOutput, SVError> {
        let _ = appended_points;
        self.render()
    }
}

fn sv_record_resolved_points(scene: &SVPairScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(scene: &SVPairScene) -> Result<SVPairRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVPairScene) -> Result<SVPairRenderOutput, SVError> {
    Ok(SVPairRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVPairScene) -> Result<SVPairRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVPairScene) -> Result<SVPairRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVPairRenderOutput::Native(SVPairFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVPairScene) -> Result<SVPairRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}
