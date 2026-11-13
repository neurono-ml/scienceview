//! The single validation point of the `SVRugPlot` builder.

use super::builder::SVRugPlotBuilder;
use super::configuration::SVRugConfiguration;
use super::core_implementation::SVRugPlot;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_error::SVError;
use crate::sv_mark::{SVMark, SVMarkKind};

impl<'data> SVRugPlotBuilder<'data> {
    /// Validates the configuration and builds the rug plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step,
    /// [`SVError::InvalidConfiguration`] for a missing value column or an
    /// impossible height, opacity or stroke width, [`SVError::ColumnNotFound`]
    /// for a column the source does not expose, [`SVError::MismatchedLengths`]
    /// when the series column disagrees with the values, and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw lines.
    pub fn build(self) -> Result<SVRugPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self.value.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVRugPlot requires a `value` column; call `value(\"<column>\")`".to_string(),
        })?;

        sv_validate_height(self.height)?;
        let mut options = SVMark::line().opacity(self.alpha);
        if let Some(width) = self.line_width {
            options = options.width(width);
        }
        let mark: SVMark = options.into();
        mark.validate()?;
        sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;

        let mut value_snapshot = data_source.column(&value)?;
        let value_view = value_snapshot.column_view();
        let number_of_observations = value_view.length();
        let representation = value_view.representation();

        if let Some(series) = self.series.as_deref() {
            let mut series_snapshot = data_source.column(series)?;
            let series_view = series_snapshot.column_view();
            if series_view.length() != number_of_observations {
                return Err(SVError::MismatchedLengths {
                    name: series.to_string(),
                    expected: number_of_observations,
                    actual: series_view.length(),
                });
            }
        }

        Ok(SVRugPlot {
            configuration: SVRugConfiguration {
                data_source,
                value,
                series: self.series,
                series_labels: self.series_labels,
                orientation: self.orientation,
                height: self.height,
                alpha: self.alpha,
                line_width: self.line_width,
                title: self.title,
                value_label: self.value_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_observations,
                representation,
            },
        })
    }
}

fn sv_validate_height(height: f32) -> Result<(), SVError> {
    let valid = height.is_finite() && height > 0.0 && height <= 1.0;
    if valid {
        return Ok(());
    }

    Err(SVError::InvalidConfiguration {
        message: format!(
            "the rug height is a fraction of the plot extent between 0 and 1, received {height}"
        ),
    })
}
