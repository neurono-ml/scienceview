//! `SVSplomPlot`: validation, the built figure and the operation spans.

use super::builder::SVSplomPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVSplomConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVSplomFrame;
use super::render_output::SVSplomRenderOutput;
use super::scene::SVSplomScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A scatterplot matrix ready to render.
///
/// Built only by [`SVSplomPlotBuilder`](super::SVSplomPlotBuilder): the direct
/// constructor is private, so the explicit backend rule cannot be bypassed. The
/// backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVSplomPlot<'data> {
    pub(super) configuration: SVSplomConfiguration<'data>,
}

impl std::fmt::Debug for SVSplomPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVSplomPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("variables", &self.configuration.variables.len())
            .field("observations", &self.configuration.number_of_observations)
            .finish_non_exhaustive()
    }
}

impl<'data> SVSplomPlot<'data> {
    /// Starts building a scatterplot matrix.
    pub fn builder() -> SVSplomPlotBuilder<'data> {
        SVSplomPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the first variable column.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of observations the matrix was built over.
    pub fn observation_count(&self) -> usize {
        self.configuration.number_of_observations
    }

    /// Returns the number of variables, the width and height of the grid.
    pub fn variable_count(&self) -> usize {
        self.configuration.variables.len()
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves every variable column once and never blocks on a
    /// streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, layout, scale or mark error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVSplomScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
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
    /// and every other kind reports [`SVError::UnsupportedBackendCapability`].
    /// A compiled-out backend reports [`SVError::BackendUnavailable`].
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no recipe-level
    /// serializer.
    pub fn render(&self) -> Result<SVSplomRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_points(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }
}

fn sv_record_resolved_points(scene: &SVSplomScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(scene: &SVSplomScene) -> Result<SVSplomRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVSplomScene) -> Result<SVSplomRenderOutput, SVError> {
    Ok(SVSplomRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVSplomScene) -> Result<SVSplomRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVSplomScene) -> Result<SVSplomRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVSplomRenderOutput::Native(SVSplomFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVSplomScene) -> Result<SVSplomRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

/// Probes the named columns once for existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<(usize, SVRepresentation)> = None;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        match expected {
            None => expected = Some((view.length(), view.representation())),
            Some((length, _)) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some(_) => {}
        }
    }
    Ok(expected.unwrap_or((0, SVRepresentation::Dense)))
}
