//! The public operations and their observability spans.

use super::super::composition::SVStepPlotComposition;
use super::super::extraction::sv_combined_representation;
use super::definition::SVStepPlot;
use scienceview_core::sv_backend::SVRenderOutput;
use scienceview_core::sv_data::SVRepresentation;
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
        let (representation, points) = self.sv_measure()?;
        sv_run_operation("render", self.backend_kind, representation, points, || {
            self.sv_serialize(&self.compose()?)
        })
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
        let (representation, points) = self.sv_measure()?;
        sv_run_operation("update", self.backend_kind, representation, points, || {
            self.sv_serialize(&self.compose()?)
        })
    }

    /// Writes the rendered chart to `destination`.
    ///
    /// # Errors
    ///
    /// Same as [`SVStepPlot::render`], plus [`SVError::BackendFailure`] when
    /// the destination cannot be written.
    pub fn save(&self, destination: &Path) -> Result<(), SVError> {
        let (representation, points) = self.sv_measure()?;
        sv_run_operation("save", self.backend_kind, representation, points, || {
            let composition = self.compose()?;
            let output = self.sv_serialize(&composition)?;
            std::fs::write(destination, output.bytes).map_err(|error| SVError::BackendFailure {
                backend_kind: self.backend_kind,
                operation: "save",
                message: error.to_string(),
            })
        })
    }

    fn sv_measure(&self) -> Result<(SVRepresentation, usize), SVError> {
        let mut x_snapshot = self.data_source.column(&self.x)?;
        let mut y_snapshot = self.data_source.column(&self.y)?;
        let mut series_snapshot = match self.series.as_deref() {
            Some(name) => Some(self.data_source.column(name)?),
            None => None,
        };
        let x_view = x_snapshot.column_view();
        let y_view = y_snapshot.column_view();
        let series_view = series_snapshot
            .as_mut()
            .map(|snapshot| snapshot.column_view());
        if y_view.length() != x_view.length() {
            return Err(SVError::MismatchedLengths {
                name: self.y.clone(),
                expected: x_view.length(),
                actual: y_view.length(),
            });
        }
        Ok((
            sv_combined_representation(x_view, y_view, series_view),
            x_view.length(),
        ))
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
