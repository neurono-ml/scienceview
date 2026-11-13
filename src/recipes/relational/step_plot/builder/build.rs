//! The single validation point of the builder.

use super::super::core_implementation::SVStepPlot;
use super::definition::SVStepPlotBuilder;
use scienceview_core::sv_error::SVError;

impl<'source> SVStepPlotBuilder<'source> {
    /// Validates the configuration and builds the chart.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] when a required builder step is
    /// missing, [`SVError::ColumnNotFound`] for an unknown column,
    /// [`SVError::MismatchedLengths`] for columns of different lengths, and
    /// [`SVError::InvalidConfiguration`] for a backend this change cannot
    /// serialize, a non-aggregate stat, or a mistyped column.
    pub fn build(self) -> Result<SVStepPlot<'source>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = self.x.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVStepPlot requires an `x` column; call `x(\"<column>\")`".to_string(),
        })?;
        let y = self.y.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVStepPlot requires a `y` column; call `y(\"<column>\")`".to_string(),
        })?;

        let plot = SVStepPlot {
            data_source,
            x,
            y,
            series: self.series,
            step_position: self.step_position,
            duplicate_policy: self.duplicate_policy,
            line_width: self.line_width,
            dash: self.dash,
            markers: self.markers,
            aggregate: self.aggregate,
            viewport: self.viewport,
            title: self.title,
            backend_kind,
            execution_mode: self.execution_mode,
        };
        plot.validate()?;
        Ok(plot)
    }
}
