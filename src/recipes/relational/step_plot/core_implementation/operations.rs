//! The public operations and their observability spans.

use super::super::composition::SVStepPlotComposition;
use super::definition::SVStepPlot;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_observability::sv_run_operation;
use std::path::Path;

impl SVStepPlot<'_> {
    /// Renders the chart with the configured backend.
    ///
    /// # Errors
    ///
    /// Returns the composition error, or [`SVError::BackendUnavailable`] when
    /// the `render-svg` feature is disabled.
    pub fn render(&self) -> Result<SVRenderOutput, SVError> {
        let composition = self.compose()?;
        sv_run_operation(
            "render",
            self.backend_kind,
            composition.representation(),
            composition.point_count(),
            || self.sv_serialize(&composition),
        )
    }

    /// Re-renders the current window after `appended_points` new samples.
    ///
    /// A step over a live source extends to the newest sample without
    /// recomputing the chart configuration.
    ///
    /// # Errors
    ///
    /// Same as [`SVStepPlot::render`].
    pub fn update(&self, _appended_points: usize) -> Result<SVRenderOutput, SVError> {
        let composition = self.compose()?;
        sv_run_operation(
            "update",
            self.backend_kind,
            composition.representation(),
            composition.point_count(),
            || self.sv_serialize(&composition),
        )
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// # Errors
    ///
    /// Same as [`SVStepPlot::render`], plus [`SVError::BackendFailure`] when
    /// the destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let composition = self.compose()?;
        sv_run_operation(
            "save",
            self.backend_kind,
            composition.representation(),
            composition.point_count(),
            || {
                let output = self.sv_serialize(&composition)?;
                std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
                    backend_kind: self.backend_kind,
                    operation: "save",
                    message: error.to_string(),
                })
            },
        )
    }

    #[cfg(feature = "render-svg")]
    fn sv_serialize(&self, composition: &SVStepPlotComposition) -> Result<SVRenderOutput, SVError> {
        Ok(SVRenderOutput {
            backend_kind: self.backend_kind,
            bytes: super::super::render_svg::sv_render_step_plot_svg(composition)?,
        })
    }

    #[cfg(not(feature = "render-svg"))]
    fn sv_serialize(
        &self,
        _composition: &SVStepPlotComposition,
    ) -> Result<SVRenderOutput, SVError> {
        Err(SVError::BackendUnavailable {
            backend_kind: self.backend_kind,
        })
    }
}
