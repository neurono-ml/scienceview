//! The public operations and their observability spans.

use super::super::composition::SVStepPlotComposition;
use super::super::extraction::sv_combined_representation;
use super::definition::SVStepPlot;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_observability::sv_run_operation;
use std::path::Path;
use tracing::Span;

impl SVStepPlot<'_> {
    /// Renders the chart with the configured backend.
    ///
    /// # Errors
    ///
    /// Returns the composition error, or [`SVError::BackendUnavailable`] when
    /// the configured kind has no serializer in this build.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        self.sv_resolve(|x_view, y_view, series_view| {
            let representation = sv_combined_representation(x_view, y_view, series_view);
            sv_run_operation("render", self.backend_kind, representation, 0, || {
                let composition = self.sv_compose_views(x_view, y_view, series_view)?;
                sv_record_drawn_points(&composition);
                self.sv_serialize(&composition)
            })
        })
    }

    /// Re-renders the current window after `appended_points` new samples.
    ///
    /// The recipe owns no incremental buffer: `appended_points` documents how
    /// many samples the source appended since the previous render, and the
    /// operation resolves the source again so the step extends to the newest
    /// sample. A backend that can append without recomposing the history
    /// consumes the count; this serializer recomposes the scene.
    ///
    /// # Errors
    ///
    /// Same as [`SVStepPlot::render`].
    pub fn update(&self, appended_points: usize) -> Result<SVRenderOutput, SVError> {
        self.sv_resolve(|x_view, y_view, series_view| {
            let representation = sv_combined_representation(x_view, y_view, series_view);
            sv_run_operation("update", self.backend_kind, representation, 0, || {
                let _ = appended_points;
                let composition = self.sv_compose_views(x_view, y_view, series_view)?;
                sv_record_drawn_points(&composition);
                self.sv_serialize(&composition)
            })
        })
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// # Errors
    ///
    /// Same as [`SVStepPlot::render`], plus [`SVError::BackendFailure`] when
    /// the destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        self.sv_resolve(|x_view, y_view, series_view| {
            let representation = sv_combined_representation(x_view, y_view, series_view);
            sv_run_operation("save", self.backend_kind, representation, 0, || {
                let composition = self.sv_compose_views(x_view, y_view, series_view)?;
                sv_record_drawn_points(&composition);
                let output = self.sv_serialize(&composition)?;
                std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
                    backend_kind: self.backend_kind,
                    operation: "save",
                    message: error.to_string(),
                })
            })
        })
    }

    fn sv_serialize(&self, composition: &SVStepPlotComposition) -> Result<SVRenderOutput, SVError> {
        #[cfg(feature = "render-svg")]
        if self.backend_kind == scienceview_core::sv_backend::SVBackendKind::Svg {
            return Ok(SVRenderOutput {
                backend_kind: self.backend_kind,
                bytes: super::super::render_svg::sv_render_step_plot_svg(composition)?,
            });
        }
        #[cfg(not(feature = "render-svg"))]
        let _ = composition;
        Err(SVError::BackendUnavailable {
            backend_kind: self.backend_kind,
        })
    }
}

/// Records the number of observations the scene actually draws.
///
/// The span starts with the input shape unknown (zero) because only the
/// composition knows how many samples survive ordering, duplicate collapsing,
/// aggregation and missing-value omission. Recording here keeps `points` equal
/// to the drawn marks even for a sparse column whose logical length is large.
fn sv_record_drawn_points(composition: &SVStepPlotComposition) {
    Span::current().record("points", composition.point_count());
}
