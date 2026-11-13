//! Binding a data source to the builder.

use super::definition::SVStepPlotBuilder;
use scienceview_core::sv_data::SVDataSource;

impl SVStepPlotBuilder<'_> {
    /// Attaches the data source the chart reads its columns from.
    #[must_use]
    pub fn data_source(self, data_source: &dyn SVDataSource) -> SVStepPlotBuilder<'_> {
        SVStepPlotBuilder {
            data_source: Some(data_source),
            x: self.x,
            y: self.y,
            series: self.series,
            step_position: self.step_position,
            duplicate_policy: self.duplicate_policy,
            line_width: self.line_width,
            dash: self.dash,
            markers: self.markers,
            aggregate: self.aggregate,
            viewport: self.viewport,
            x_scale_kind: self.x_scale_kind,
            title: self.title,
            backend_kind: self.backend_kind,
            execution_mode: self.execution_mode,
        }
    }
}
