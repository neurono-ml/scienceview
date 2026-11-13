//! The single validation point of the builder.

use super::super::core_implementation::SVECdfPlot;
use super::definition::SVECdfPlotBuilder;
use scienceview_core::sv_data::{SVColumnRole, SVDataSource};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVMark;

impl<'source> SVECdfPlotBuilder<'source> {
    /// Validates the configuration and builds the chart.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] when a required builder step is
    /// missing, [`SVError::ColumnNotFound`] for an unknown column,
    /// [`SVError::MismatchedLengths`] for columns of different lengths and
    /// [`SVError::InvalidConfiguration`] for a missing `value` column, a
    /// label-role column or an invalid line width. The configured backend kind
    /// is not rejected here: the composition is backend-neutral, and an
    /// operation whose kind has no serializer reports the missing capability.
    pub fn build(self) -> Result<SVECdfPlot<'source>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self.value.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVECdfPlot requires a `value` column; call `value(\"<column>\")`".to_string(),
        })?;

        sv_validate_column(data_source, &value, "value")?;
        if let Some(series) = &self.series {
            sv_validate_column(data_source, series, "series")?;
        }

        let expected = sv_column_length(data_source, &value)?;
        if let Some(series) = &self.series {
            let actual = sv_column_length(data_source, series)?;
            if actual != expected {
                return Err(SVError::MismatchedLengths {
                    name: series.clone(),
                    expected,
                    actual,
                });
            }
        }

        if let Some(width) = self.line_width {
            let mark: SVMark = SVMark::line().width(width).into();
            mark.validate()?;
        }

        Ok(SVECdfPlot {
            data_source,
            value,
            series: self.series,
            normalise: self.normalise,
            orient: self.orient,
            complementary: self.complementary,
            line_width: self.line_width,
            viewport: self.viewport,
            title: self.title,
            backend_kind,
            execution_mode: self.execution_mode,
        })
    }
}

fn sv_validate_column(
    data_source: &dyn SVDataSource,
    name: &str,
    channel: &str,
) -> Result<(), SVError> {
    if let Some(descriptor) = data_source.schema().column(name) {
        if descriptor.role() == SVColumnRole::Label {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the {channel} column `{name}` carries labels; supply a numeric column instead"
                ),
            });
        }
    }
    data_source.column(name)?;
    Ok(())
}

fn sv_column_length(data_source: &dyn SVDataSource, name: &str) -> Result<usize, SVError> {
    let mut snapshot = data_source.column(name)?;
    Ok(snapshot.column_view().length())
}
