//! `SVHeatmapPlot`: validation, the built figure and the one operation span.

use super::builder::SVHeatmapPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVHeatmapConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVHeatmapFrame;
use super::render_output::SVHeatmapRenderOutput;
use super::scene::SVHeatmapScene;
use super::{SVHeatmapOrientation, SVMissingValue};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A heatmap ready to render.
///
/// Built only by [`SVHeatmapPlotBuilder`](super::SVHeatmapPlotBuilder): the
/// direct constructor is private, so the explicit backend rule cannot be
/// bypassed. The backend kind is carried, never dispatched in `build`.
#[derive(Clone)]
pub struct SVHeatmapPlot<'data> {
    pub(super) configuration: SVHeatmapConfiguration<'data>,
}

impl std::fmt::Debug for SVHeatmapPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVHeatmapPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("cells", &self.configuration.number_of_cells)
            .finish_non_exhaustive()
    }
}

impl<'data> SVHeatmapPlot<'data> {
    /// Starts building a heatmap.
    pub fn builder() -> SVHeatmapPlotBuilder<'data> {
        SVHeatmapPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the values.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the number of logical cells the heatmap draws.
    pub fn cell_count(&self) -> usize {
        self.configuration.number_of_cells
    }

    /// Returns the missing-cell rule.
    pub fn missing_value(&self) -> SVMissingValue {
        self.configuration.missing
    }

    /// Returns the configured axis orientation.
    pub fn orientation(&self) -> SVHeatmapOrientation {
        self.configuration.orientation
    }

    /// Returns whether the colour bar is drawn.
    pub fn has_colour_bar(&self) -> bool {
        self.configuration.colour_bar
    }

    /// Composes the backend-agnostic scene.
    ///
    /// # Errors
    ///
    /// Returns the data, pivot, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVHeatmapScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_cells,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_cells(&scene);
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
    pub fn render(&self) -> Result<SVHeatmapRenderOutput, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_cells,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_cells(&scene);
                sv_dispatch_render(&scene)
            },
        )
    }
}

fn sv_record_resolved_cells(scene: &SVHeatmapScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(scene: &SVHeatmapScene) -> Result<SVHeatmapRenderOutput, SVError> {
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
fn sv_render_svg(scene: &SVHeatmapScene) -> Result<SVHeatmapRenderOutput, SVError> {
    Ok(SVHeatmapRenderOutput::Svg(scene.to_svg()?.into_bytes()))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(scene: &SVHeatmapScene) -> Result<SVHeatmapRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(scene: &SVHeatmapScene) -> Result<SVHeatmapRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVHeatmapRenderOutput::Native(SVHeatmapFrame::new(
        frame.width(),
        frame.height(),
        frame.pixels().to_vec(),
    )))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(scene: &SVHeatmapScene) -> Result<SVHeatmapRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: scene.backend_kind(),
    })
}

/// Probes the named columns once for existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<usize> = None;
    let mut representation = SVRepresentation::Dense;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        match expected {
            None => expected = Some(view.length()),
            Some(length) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some(_) => {}
        }
        // The value column is named last, so the reported representation is
        // the one the cells are drawn from, not the axis keys' representation.
        representation = view.representation();
    }
    Ok((expected.unwrap_or(0), representation))
}
