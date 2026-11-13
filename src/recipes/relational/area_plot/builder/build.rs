//! Build-time validation and the construction of the built plot.

use scienceview_core::sv_backend::sv_verify_mark_supported;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVWithBackend;
use scienceview_core::sv_mark::{SVBaselineMode, SVMarkKind};

use super::super::configuration::SVAreaPlotConfiguration;
use super::super::core_implementation::SVAreaPlot;
use super::builder_definition::SVAreaPlotBuilder;
use super::validation::{sv_invalid, sv_validate_recipe_backend, sv_validate_scale_kind};

impl<'values> SVAreaPlotBuilder<'values, SVWithBackend> {
    /// Validates the configuration and builds the area plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] when no source was attached,
    /// [`SVError::ColumnNotFound`] for a column the source does not declare,
    /// [`SVError::InvalidConfiguration`] for an incomplete or out-of-range
    /// option, [`SVError::UnsupportedMark`] when the selected backend cannot
    /// draw the composed area, and [`SVError::UnsupportedBackendCapability`]
    /// when the recipe has no serializer for the selected kind.
    pub fn build(self) -> Result<SVAreaPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let x = self
            .x
            .ok_or_else(|| sv_invalid("an area plot needs the `x` column"))?;
        let y = self
            .y
            .ok_or_else(|| sv_invalid("an area plot needs the `y` column"))?;

        sv_validate_scale_kind(self.x_scale_kind)?;
        if let Some(opacity) = self.opacity {
            if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
                return Err(sv_invalid(
                    "the area opacity must be finite and lie in 0.0..=1.0",
                ));
            }
        }
        if let SVBaselineMode::Value(value) = self.baseline {
            if !value.is_finite() {
                return Err(sv_invalid("the area baseline must be finite"));
            }
        }

        let schema = data_source.schema();
        for name in [Some(x), Some(y), self.series].into_iter().flatten() {
            if schema.column(name).is_none() {
                return Err(SVError::ColumnNotFound {
                    name: name.to_string(),
                });
            }
        }

        sv_verify_mark_supported(SVMarkKind::Area, backend_kind)?;
        if self.stack {
            sv_verify_mark_supported(SVMarkKind::Ribbon, backend_kind)?;
        }
        if self.line {
            sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;
        }
        sv_validate_recipe_backend(backend_kind)?;

        let configuration = SVAreaPlotConfiguration {
            data_source,
            x,
            y,
            series: self.series,
            stack: self.stack,
            baseline: self.baseline,
            opacity: self.opacity,
            line: self.line,
            interpolation: self.interpolation,
            missing_value_policy: self.missing_value_policy,
            theme: self.theme,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            x_scale_kind: self.x_scale_kind,
            temporal_unit: self.temporal_unit,
            viewport: self.viewport,
        };

        Ok(SVAreaPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}
