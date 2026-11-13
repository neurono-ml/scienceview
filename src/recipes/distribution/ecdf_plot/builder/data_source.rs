//! Binding a data source to the builder.

use super::definition::SVECdfPlotBuilder;
use scienceview_core::sv_data::SVDataSource;

impl SVECdfPlotBuilder<'_> {
    /// Attaches the data source the chart reads its columns from.
    #[must_use]
    pub fn data_source(self, data_source: &dyn SVDataSource) -> SVECdfPlotBuilder<'_> {
        SVECdfPlotBuilder {
            data_source: Some(data_source),
            value: self.value,
            series: self.series,
            normalise: self.normalise,
            orient: self.orient,
            complementary: self.complementary,
            line_width: self.line_width,
            viewport: self.viewport,
            title: self.title,
            backend_kind: self.backend_kind,
            execution_mode: self.execution_mode,
        }
    }
}
