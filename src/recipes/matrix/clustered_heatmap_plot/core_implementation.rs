//! `SVClusteredHeatmapPlot`: the built figure and its one operation span.

use super::SVClusterTree;
use super::builder::SVClusteredHeatmapPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVClusteredHeatmapConfiguration;
#[cfg(feature = "render-native")]
use super::render_output::SVClusteredHeatmapFrame;
use super::render_output::SVClusteredHeatmapRenderOutput;
use super::scene::SVClusteredHeatmapScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_observability::sv_run_operation;

/// A clustered heatmap ready to render.
///
/// Built only by [`SVClusteredHeatmapPlotBuilder`](super::SVClusteredHeatmapPlotBuilder):
/// the direct constructor is private, so the explicit backend rule cannot be
/// bypassed. The recipe renders a supplied clustering and never computes one.
#[derive(Clone)]
pub struct SVClusteredHeatmapPlot<'data> {
    pub(super) configuration: SVClusteredHeatmapConfiguration<'data>,
}

impl std::fmt::Debug for SVClusteredHeatmapPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVClusteredHeatmapPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("cells", &self.configuration.number_of_cells)
            .field("rows", &self.configuration.row_order.len())
            .field("columns", &self.configuration.column_order.len())
            .finish_non_exhaustive()
    }
}

impl<'data> SVClusteredHeatmapPlot<'data> {
    /// Starts building a clustered heatmap.
    pub fn builder() -> SVClusteredHeatmapPlotBuilder<'data> {
        SVClusteredHeatmapPlotBuilder::new()
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

    /// Returns the supplied row order.
    pub fn row_order(&self) -> &'data [usize] {
        self.configuration.row_order
    }

    /// Returns the supplied column order.
    pub fn column_order(&self) -> &'data [usize] {
        self.configuration.column_order
    }

    /// Returns the supplied row tree, when one was given.
    pub fn row_linkage(&self) -> Option<SVClusterTree<'data>> {
        self.configuration.row_tree
    }

    /// Returns the supplied column tree, when one was given.
    pub fn column_linkage(&self) -> Option<SVClusterTree<'data>> {
        self.configuration.column_tree
    }

    /// Composes the backend-agnostic scene.
    ///
    /// # Errors
    ///
    /// Returns the data, pivot, tree, scale or mark error of the composition,
    /// after recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVClusteredHeatmapScene, SVError> {
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
    pub fn render(&self) -> Result<SVClusteredHeatmapRenderOutput, SVError> {
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

fn sv_record_resolved_cells(scene: &SVClusteredHeatmapScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}

fn sv_dispatch_render(
    scene: &SVClusteredHeatmapScene,
) -> Result<SVClusteredHeatmapRenderOutput, SVError> {
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
fn sv_render_svg(
    scene: &SVClusteredHeatmapScene,
) -> Result<SVClusteredHeatmapRenderOutput, SVError> {
    Ok(SVClusteredHeatmapRenderOutput::Svg(
        scene.to_svg()?.into_bytes(),
    ))
}

#[cfg(not(feature = "render-svg"))]
fn sv_render_svg(
    _scene: &SVClusteredHeatmapScene,
) -> Result<SVClusteredHeatmapRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Svg,
    })
}

#[cfg(feature = "render-native")]
fn sv_render_native(
    scene: &SVClusteredHeatmapScene,
) -> Result<SVClusteredHeatmapRenderOutput, SVError> {
    let frame = scene.to_native_frame()?;
    Ok(SVClusteredHeatmapRenderOutput::Native(
        SVClusteredHeatmapFrame::new(frame.width(), frame.height(), frame.pixels().to_vec()),
    ))
}

#[cfg(not(feature = "render-native"))]
fn sv_render_native(
    _scene: &SVClusteredHeatmapScene,
) -> Result<SVClusteredHeatmapRenderOutput, SVError> {
    Err(SVError::BackendUnavailable {
        backend_kind: SVBackendKind::Native,
    })
}
