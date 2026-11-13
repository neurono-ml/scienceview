//! `SVJointPlot`: validation, the built figure and the one operation span.

use super::builder::SVJointPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVJointConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVJointFrame;
use super::render_output::SVJointRenderOutput;
use super::scene::SVJointScene;
use super::{SVJointKind, SVJointMarginal};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A joint plot ready to render.
///
/// Built only by [`SVJointPlotBuilder`](super::SVJointPlotBuilder): the direct
/// constructor is private, so the explicit backend rule cannot be bypassed. The
/// backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVJointPlot<'data> {
    pub(super) configuration: SVJointConfiguration<'data>,
}

impl std::fmt::Debug for SVJointPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVJointPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("kind", &self.configuration.kind)
            .field("marginal_kind", &self.configuration.marginal_kind)
            .field("points", &self.configuration.number_of_points)
            .finish_non_exhaustive()
    }
}

impl<'data> SVJointPlot<'data> {
    /// Starts building a joint plot.
    pub fn builder() -> SVJointPlotBuilder<'data> {
        SVJointPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the horizontal column.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of logical observations the joint plot draws.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_points
    }

    /// Returns the configured central form.
    pub fn kind(&self) -> SVJointKind {
        self.configuration.kind
    }

    /// Returns the configured marginal form, with `Auto` still unresolved.
    pub fn marginal_kind(&self) -> SVJointMarginal {
        self.configuration.marginal_kind
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The call resolves each column once, solves the three panels through the
    /// layout kernel and composes the configured central form plus the two
    /// marginals. It never densifies a sparse column and never blocks on a
    /// streaming one.
    ///
    /// # Errors
    ///
    /// Returns the column, presence, scale or mark error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVJointScene, SVError> {
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
    /// and every other kind reports
    /// [`SVError::UnsupportedBackendCapability`] because the recipe has no
    /// serializer for it yet.
    ///
    /// # Errors
    ///
    /// Returns the composition error, [`SVError::BackendUnavailable`] when the
    /// configured backend's feature is disabled, or
    /// [`SVError::UnsupportedBackendCapability`] for a kind with no
    /// recipe-level serializer.
    pub fn render(&self) -> Result<SVJointRenderOutput, SVError> {
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

fn sv_record_resolved_points(scene: &SVJointScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(scene: &SVJointScene) -> Result<SVJointRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVJointScene) -> Result<SVJointRenderOutput, SVError> {
    Ok(SVJointRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVJointScene) -> Result<SVJointRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVJointScene) -> Result<SVJointRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVJointRenderOutput::Native(SVJointFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVJointScene) -> Result<SVJointRenderOutput, SVError> {
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
