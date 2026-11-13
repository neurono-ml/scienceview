//! `SVRugPlot`: the built figure, its read-only surface and `compose`.

use super::builder::SVRugPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVRugConfiguration;
use super::scene::SVRugScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVBarOrientation;
use crate::sv_observability::sv_run_operation;

/// A rug plot ready to render.
///
/// Built only by [`SVRugPlotBuilder`]: the direct constructor is private, so
/// the explicit backend rule cannot be bypassed. The backend kind is carried,
/// never dispatched in `build`.
#[derive(Clone)]
pub struct SVRugPlot<'data> {
    pub(super) configuration: SVRugConfiguration<'data>,
}

impl std::fmt::Debug for SVRugPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVRugPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("orientation", &self.configuration.orientation)
            .field("observations", &self.configuration.number_of_observations)
            .finish_non_exhaustive()
    }
}

impl<'data> SVRugPlot<'data> {
    /// Starts building a rug plot.
    pub fn builder() -> SVRugPlotBuilder<'data> {
        SVRugPlotBuilder::new()
    }

    /// Returns the configured backend kind.
    pub fn backend_kind(&self) -> SVBackendKind {
        self.configuration.backend_kind
    }

    /// Returns the configured execution mode.
    pub fn execution_mode(&self) -> SVExecutionMode {
        self.configuration.execution_mode
    }

    /// Returns the representation of the value column.
    pub fn representation(&self) -> SVRepresentation {
        self.configuration.representation
    }

    /// Returns the logical number of observations the value column holds.
    pub fn observation_count(&self) -> usize {
        self.configuration.number_of_observations
    }

    /// Returns the axis the values occupy.
    pub fn orientation(&self) -> SVBarOrientation {
        self.configuration.orientation
    }

    /// Returns the tick length as a fraction of the plot's perpendicular extent.
    pub fn height(&self) -> f32 {
        self.configuration.height
    }

    /// Returns the constant opacity of the ticks.
    pub fn alpha(&self) -> f32 {
        self.configuration.alpha
    }

    /// Returns the explicit stroke width, or `None` when the token applies.
    pub fn line_width(&self) -> Option<SVLength> {
        self.configuration.line_width
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The scene carries the configured kind but no notation: an SVG, native
    /// or component serializer turns the same viewport, decorations and tick
    /// segments into its own artifact. The call resolves each column once and
    /// never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVRugScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_ticks(&scene);
                Ok(scene)
            },
        )
    }
}

/// Records the tick count the current composition actually drew.
///
/// `build` probed the column once to validate a figure that can render; a
/// streaming source may have advanced since then. The span therefore starts
/// with the build-time count and is corrected here, after the columns were
/// resolved for this operation.
pub(super) fn sv_record_resolved_ticks(scene: &SVRugScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}
