//! Build-time validation and the construction of the built plot.

use scienceview_core::sv_backend::sv_verify_mark_supported;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVWithBackend;
use scienceview_core::sv_mark::SVMarkKind;

use super::super::configuration::SVDensityPlotConfiguration;
use super::super::core_implementation::SVDensityPlot;
use super::builder_definition::SVDensityPlotBuilder;
use super::validation::{sv_invalid, sv_validate_line_width, sv_validate_recipe_backend};

impl<'values> SVDensityPlotBuilder<'values, SVWithBackend> {
    /// Validates the configuration and builds the density plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] when no source was attached,
    /// [`SVError::ColumnNotFound`] for a column the source does not declare,
    /// [`SVError::InvalidConfiguration`] for an incomplete or out-of-range
    /// option, [`SVError::UnsupportedMark`] when the selected backend cannot
    /// draw the composed marks, and [`SVError::UnsupportedBackendCapability`]
    /// when the recipe has no serializer for the selected kind.
    pub fn build(self) -> Result<SVDensityPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .ok_or_else(|| sv_invalid("a density plot needs the `value` column"))?;

        if let Some(bandwidth) = self.bandwidth {
            if !bandwidth.is_finite() || bandwidth <= 0.0 {
                return Err(sv_invalid(
                    "the density bandwidth must be a positive finite number",
                ));
            }
        }
        if !self.bandwidth_adjustment.is_finite() || self.bandwidth_adjustment <= 0.0 {
            return Err(sv_invalid(
                "the density bandwidth adjustment must be a positive finite number",
            ));
        }
        if self.grid_count < 2 {
            return Err(sv_invalid("a density grid needs at least two samples"));
        }
        if !self.cut.is_finite() || self.cut < 0.0 {
            return Err(sv_invalid(
                "the density cut must be a non-negative finite number",
            ));
        }
        sv_validate_line_width(self.line_width)?;

        let schema = data_source.schema();
        for name in [Some(value), self.series].into_iter().flatten() {
            if schema.column(name).is_none() {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        }

        sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;
        if self.fill {
            sv_verify_mark_supported(SVMarkKind::Area, backend_kind)?;
        }
        sv_validate_recipe_backend(backend_kind)?;

        let configuration = SVDensityPlotConfiguration {
            data_source,
            value,
            series: self.series,
            bandwidth: self.bandwidth,
            bandwidth_adjustment: self.bandwidth_adjustment,
            kernel: self.kernel,
            grid_count: self.grid_count,
            cut: self.cut,
            fill: self.fill,
            line_width: self.line_width,
            normalisation: self.normalisation,
            missing_value_policy: self.missing_value_policy,
            theme: self.theme,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
        };

        Ok(SVDensityPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}
