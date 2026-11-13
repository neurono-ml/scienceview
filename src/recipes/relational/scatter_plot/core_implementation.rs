//! `SVScatterPlot`: validation, the built figure and the one operation span.

use super::builder::SVScatterPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVScatterConfiguration;
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

    /// Composes the scene with the configured backend.
    ///
    /// The scene is backend-agnostic: the configured kind is recorded in the
    /// operation span and in the built figure, and a backend serializes the
    /// geometry later. The call resolves each column once and never blocks on a
    /// streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn render(&self) -> Result<SVScatterScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "render",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_points,
            || sv_compose_scene(configuration),
        )
    }
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
