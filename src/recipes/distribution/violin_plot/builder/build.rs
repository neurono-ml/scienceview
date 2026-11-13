//! Build-time validation and the construction of the built plot.

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMark, SVMarkKind};

use super::super::configuration::{SVViolinPlotConfiguration, sv_invalid};
use super::super::core_implementation::SVViolinPlot;
use super::super::data_contract::sv_validate_columns;
use super::SVViolinPlotBuilder;

impl<'values> SVViolinPlotBuilder<'values> {
    /// Validates the configuration and builds the violin plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`],
    /// [`SVError::ColumnNotFound`], [`SVError::InvalidConfiguration`] for an
    /// out-of-range option, [`SVError::UnsupportedMark`] when the backend cannot
    /// draw a composed mark, and [`SVError::UnsupportedBackendCapability`] when
    /// the recipe has no serializer for the selected kind.
    pub fn build(self) -> Result<SVViolinPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .ok_or_else(|| sv_invalid("a violin plot needs the `value` column"))?;
        let category = self
            .category
            .ok_or_else(|| sv_invalid("a violin plot needs the `category` column"))?;
        sv_validate_options(&self)?;
        if self.split && self.series.is_none() {
            return Err(sv_invalid(
                "split violins need the `series` column; without it there is nothing to split",
            ));
        }
        sv_validate_recipe_backend(backend_kind)?;
        sv_verify_mark_supported(SVMarkKind::Area, backend_kind)?;
        if self.show_box {
            sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;
        }
        if self.show_points {
            sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;
        }
        SVMark::from(SVMark::area()).validate()?;

        let configuration = SVViolinPlotConfiguration {
            data_source,
            value,
            category,
            series: self.series,
            bandwidth: self.bandwidth,
            bandwidth_adjustment: self.bandwidth_adjustment,
            width: self.width,
            scale: self.scale,
            show_box: self.show_box,
            show_points: self.show_points,
            split: self.split,
            orientation: self.orientation,
            missing_value_policy: self.missing_value_policy,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
        };
        sv_validate_columns(&configuration)?;
        Ok(SVViolinPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}

fn sv_validate_options(builder: &SVViolinPlotBuilder<'_>) -> Result<(), SVError> {
    if let Some(bandwidth) = builder.bandwidth {
        if !bandwidth.is_finite() || bandwidth <= 0.0 {
            return Err(sv_invalid(
                "the violin bandwidth must be a positive finite number",
            ));
        }
    }
    if !builder.bandwidth_adjustment.is_finite() || builder.bandwidth_adjustment <= 0.0 {
        return Err(sv_invalid(
            "the violin bandwidth adjustment must be a positive finite number",
        ));
    }
    if !builder.width.is_finite() || builder.width <= 0.0 || builder.width > 1.0 {
        return Err(sv_invalid(
            "the violin width must be finite and lie in 0.0..=1.0",
        ));
    }
    Ok(())
}

fn sv_validate_recipe_backend(backend_kind: SVBackendKind) -> Result<(), SVError> {
    if matches!(backend_kind, SVBackendKind::Svg | SVBackendKind::Native) {
        Ok(())
    } else {
        Err(SVError::UnsupportedBackendCapability {
            backend_kind,
            capability: SVBackendCapability::Export,
        })
    }
}
