//! The validating build operation of the point-plot builder.

use scienceview_core::sv_backend::sv_verify_mark_supported;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVMark, SVMarkKind};

use crate::sv_recipes::sv_distribution::sv_point_plot::configuration::{
    SVPointPlotConfiguration, sv_invalid,
};
use crate::sv_recipes::sv_distribution::sv_point_plot::core_implementation::SVPointPlot;
use crate::sv_recipes::sv_distribution::sv_point_plot::data_contract::sv_validate_columns;

use super::definition::SVPointPlotBuilder;

impl<'values> SVPointPlotBuilder<'values> {
    /// Validates the configuration and builds the point plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`]
    /// or a configuration error naming the missing column, the invalid cap
    /// width, the invalid confidence level, the invalid marker or line option,
    /// or the backend that cannot host a required mark.
    pub fn build(self) -> Result<SVPointPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .ok_or_else(|| sv_invalid("a point plot needs the `value` column"))?;
        let category = self
            .category
            .ok_or_else(|| sv_invalid("a point plot needs the `category` column"))?;
        if !self.cap_width.is_finite() || !(0.0..=1.0).contains(&self.cap_width) {
            return Err(sv_invalid(
                "the error-bar cap width must be finite and lie in 0.0..=1.0",
            ));
        }
        if !self.confidence_level.is_finite() || !(0.0..1.0).contains(&self.confidence_level) {
            return Err(sv_invalid(
                "the confidence level must be finite and lie strictly inside (0, 1)",
            ));
        }
        sv_validate_mark_options(&self)?;
        let mut required_marks = vec![SVMarkKind::Dot, SVMarkKind::ErrorBar];
        if self.join {
            required_marks.push(SVMarkKind::Line);
        }
        for mark_kind in required_marks {
            sv_verify_mark_supported(mark_kind, backend_kind)?;
        }

        let configuration = SVPointPlotConfiguration {
            data_source,
            category,
            value,
            series: self.series,
            estimator: self.estimator,
            error: self.error,
            confidence_level: self.confidence_level,
            ordering: self.ordering,
            missing_value_policy: self.missing_value_policy,
            orientation: self.orientation,
            marker: self.marker,
            caps: self.caps,
            cap_width: self.cap_width,
            join: self.join,
            line_width: self.line_width,
            dash: self.dash,
            colour: self.colour,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
        };
        sv_validate_columns(&configuration)?;

        Ok(SVPointPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}

/// Validates the mark options before any data is read.
fn sv_validate_mark_options(builder: &SVPointPlotBuilder<'_>) -> Result<(), SVError> {
    let dot: SVMark = SVMark::dot().shape(builder.marker).into();
    dot.validate()?;
    let mut line = SVMark::line();
    if let Some(width) = builder.line_width {
        line = line.width(width);
    }
    if let Some(dash) = &builder.dash {
        line = line.dash(dash.clone());
    }
    let line: SVMark = line.into();
    line.validate()
}
