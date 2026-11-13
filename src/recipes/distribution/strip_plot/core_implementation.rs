//! `SVStripPlot`: the built figure, its read-only surface and `compose`.

use super::SVStripMissingValuePolicy;
use super::builder::SVStripPlotBuilder;
use super::composition::sv_compose_scene;
use super::configuration::SVStripConfiguration;
use super::scene::SVStripScene;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};
use crate::sv_move::SVJitterMode;
use crate::sv_observability::sv_run_operation;

/// A strip plot ready to render.
///
/// Built only by [`SVStripPlotBuilder`]: the direct constructor is private, so
/// the explicit backend rule cannot be bypassed. The backend kind is carried,
/// never dispatched in `build`.
#[derive(Clone)]
pub struct SVStripPlot<'data> {
    pub(super) configuration: SVStripConfiguration<'data>,
}

impl std::fmt::Debug for SVStripPlot<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVStripPlot")
            .field("backend_kind", &self.configuration.backend_kind)
            .field("execution_mode", &self.configuration.execution_mode)
            .field("orientation", &self.configuration.orientation)
            .field("observations", &self.configuration.number_of_observations)
            .finish_non_exhaustive()
    }
}

impl<'data> SVStripPlot<'data> {
    /// Starts building a strip plot.
    pub fn builder() -> SVStripPlotBuilder<'data> {
        SVStripPlotBuilder::new()
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

    /// Returns the axis the categories occupy.
    pub fn orientation(&self) -> SVBarOrientation {
        self.configuration.orientation
    }

    /// Returns the jitter amount as a fraction of the category slot.
    pub fn jitter(&self) -> f32 {
        self.configuration.jitter
    }

    /// Returns the jitter distribution.
    pub fn jitter_mode(&self) -> SVJitterMode {
        self.configuration.jitter_mode
    }

    /// Returns the fixed jitter seed, when one is set.
    pub fn seed(&self) -> Option<u64> {
        self.configuration.seed
    }

    /// Returns the marker shape.
    pub fn marker(&self) -> SVDotShape {
        self.configuration.marker
    }

    /// Returns the constant opacity of the dots.
    pub fn alpha(&self) -> f32 {
        self.configuration.alpha
    }

    /// Returns the explicit dot size, or `None` when the token applies.
    pub fn size(&self) -> Option<SVLength> {
        self.configuration.size
    }

    /// Returns the missing-value policy.
    pub fn missing_value_policy(&self) -> SVStripMissingValuePolicy {
        self.configuration.missing_value_policy
    }

    /// Composes the backend-agnostic scene.
    ///
    /// The scene carries the configured kind but no notation: an SVG, native
    /// or component serializer turns the same viewport, decorations and dot
    /// geometries into its own artifact. The call resolves each column once,
    /// jitters through the move kernel and never blocks on a streaming window.
    ///
    /// # Errors
    ///
    /// Returns the column, scale, move or mark error of the composition, after
    /// recording the failure in the operation span.
    pub fn compose(&self) -> Result<SVStripScene, SVError> {
        let configuration = &self.configuration;
        sv_run_operation(
            "compose",
            configuration.backend_kind,
            configuration.representation,
            configuration.number_of_observations,
            || {
                let scene = sv_compose_scene(configuration)?;
                sv_record_resolved_marks(&scene);
                Ok(scene)
            },
        )
    }
}

/// Records the number of dots the current composition actually drew.
///
/// `build` probed the columns once to validate a figure that can render; a
/// streaming source may have advanced since then. The span therefore starts
/// with the build-time count and is corrected here, after the columns were
/// resolved for this operation.
pub(super) fn sv_record_resolved_marks(scene: &SVStripScene) {
    tracing::Span::current().record("points", scene.number_of_marks());
}
