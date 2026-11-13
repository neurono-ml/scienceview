//! `SVScatterPlot`: validation, the built figure and the one operation span.

use super::builder::SVScatterPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVScatterConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVScatterFrame;
use super::render_output::SVScatterRenderOutput;
use super::scene::SVScatterScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A scatter ready to render.
///
/// Built only by [`SVScatterPlotBuilder`](super::SVScatterPlotBuilder): the
/// direct constructor is private, so the explicit backend rule cannot be
/// bypassed. The backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVScatterPlot<'data> {
    pub(super) configuration: SVScatterConfiguration<'data>,
}

impl std::fmt::Debug for SVScatterPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVScatterPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("points", &self.configuration.number_of_points)
            .finish_non_exhaustive()
    }
}

impl<'data> SVScatterPlot<'data> {
    /// Starts building a scatter.
    pub fn builder() -> SVScatterPlotBuilder<'data> {
        SVScatterPlotBuilder::new()
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

    /// Returns the number of logical marks the scatter draws.
    pub fn point_count(&self) -> usize {
        self.configuration.number_of_points
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The scene carries the configured kind but no notation: an SVG, native
    /// or component serializer turns the same viewport, decorations and mark
    /// geometries into its own artifact. The call resolves each column once and
    /// never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVScatterScene, SVError> {
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
    pub fn render(&self) -> Result<SVScatterRenderOutput, SVError> {
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
///
/// `build` probes the columns once to validate a figure that can render; a
/// streaming source may have advanced since then. The span therefore starts
/// with the build-time count and is corrected here, after the columns were
/// resolved for this operation.
fn sv_record_resolved_points(scene: &SVScatterScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(scene: &SVScatterScene) -> Result<SVScatterRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVScatterScene) -> Result<SVScatterRenderOutput, SVError> {
    Ok(SVScatterRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVScatterScene) -> Result<SVScatterRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVScatterScene) -> Result<SVScatterRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVScatterRenderOutput::Native(SVScatterFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVScatterScene) -> Result<SVScatterRenderOutput, SVError> {
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
