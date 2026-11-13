//! Validation and assembly of the built parallel-coordinates plot.

use super::super::configuration::SVParallelCoordinatesConfiguration;
use super::super::core_implementation::SVParallelCoordinatesPlot;
use super::super::options::SVDimension;
use super::builder_state::SVParallelCoordinatesPlotBuilder;
use super::column_probe::sv_probe_columns;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_error::SVError;
use crate::sv_mark::{SVMark, SVMarkKind};

impl<'values> SVParallelCoordinatesPlotBuilder<'values> {
    /// Validates the configuration and builds the chart.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingBackendSelection`] without a backend,
    /// [`SVError::MissingDataSource`] without a source,
    /// [`SVError::ColumnNotFound`] for a mistyped column,
    /// [`SVError::MismatchedLengths`] for columns that do not share one length,
    /// [`SVError::InvalidConfiguration`] for an invalid option and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw a line.
    pub fn build(self) -> Result<SVParallelCoordinatesPlot<'values>, SVError> {
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;

        sv_validate_variables(&self.variables)?;
        let dimensions = sv_resolve_dimensions(&self.variables, &self.dimensions)?;
        let visible = sv_visible_dimensions(&dimensions);
        if visible.is_empty() {
            return Err(SVError::InvalidConfiguration {
                message: "a parallel-coordinates plot needs at least one visible axis".to_string(),
            });
        }
        if let Some(range) = &self.range {
            sv_validate_range(range, visible.len())?;
        }
        sv_validate_line(self.line_width, self.opacity)?;
        if !self.label_angle.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: "the variable-label angle must be finite".to_string(),
            });
        }

        let mut names = self.variables.clone();
        names.extend(self.colour);
        let (number_of_observations, representation) = sv_probe_columns(data_source, &names)?;
        sv_verify_mark_supported(SVMarkKind::Line, backend_kind)?;

        Ok(SVParallelCoordinatesPlot::from_configuration(
            SVParallelCoordinatesConfiguration {
                data_source,
                variables: self.variables,
                colour: self.colour,
                line_width: self.line_width,
                opacity: self.opacity,
                dimensions,
                range: self.range,
                label_angle: self.label_angle,
                axis_label: self.axis_label,
                title: self.title,
                backend_kind,
                execution_mode: self.execution_mode,
                representation,
                number_of_observations,
                normalization: self.normalization,
                missing: self.missing,
            },
        ))
    }
}

fn sv_validate_variables(variables: &[&str]) -> Result<(), SVError> {
    if variables.len() < 2 {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "a parallel-coordinates plot needs at least two variables, received {}",
                variables.len()
            ),
        });
    }
    for (position, variable) in variables.iter().enumerate() {
        if variables[..position].contains(variable) {
            return Err(SVError::InvalidConfiguration {
                message: format!("the variable `{variable}` is configured more than once"),
            });
        }
    }
    Ok(())
}

fn sv_resolve_dimensions<'values>(
    variables: &[&'values str],
    dimensions: &[SVDimension<'values>],
) -> Result<Vec<SVDimension<'values>>, SVError> {
    if dimensions.is_empty() {
        return Ok(variables
            .iter()
            .map(|name| SVDimension::new(name))
            .collect());
    }
    for (position, dimension) in dimensions.iter().enumerate() {
        if !variables.contains(&dimension.variable()) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the dimension `{}` is not one of the configured variables",
                    dimension.variable()
                ),
            });
        }
        if dimensions[..position]
            .iter()
            .any(|earlier| earlier.variable() == dimension.variable())
        {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the dimension `{}` is configured more than once",
                    dimension.variable()
                ),
            });
        }
        if let Some((lower, upper)) = dimension.range_override() {
            sv_validate_one_range(lower, upper, dimension.variable())?;
        }
    }
    Ok(dimensions.to_vec())
}

fn sv_visible_dimensions<'values>(
    dimensions: &'values [SVDimension<'values>],
) -> Vec<&'values SVDimension<'values>> {
    dimensions
        .iter()
        .filter(|dimension| dimension.is_visible())
        .collect()
}

fn sv_validate_range(range: &[(f64, f64)], expected: usize) -> Result<(), SVError> {
    if range.len() != expected {
        return Err(SVError::MismatchedLengths {
            name: "range overrides".to_string(),
            expected,
            actual: range.len(),
        });
    }
    for (lower, upper) in range {
        sv_validate_one_range(*lower, *upper, "range override")?;
    }
    Ok(())
}

fn sv_validate_one_range(lower: f64, upper: f64, name: &str) -> Result<(), SVError> {
    if !lower.is_finite() || !upper.is_finite() || lower >= upper {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the range of `{name}` must be finite with a lower bound below its upper bound, received ({lower}, {upper})"
            ),
        });
    }
    Ok(())
}

fn sv_validate_line(line_width: crate::sv_layout::SVLength, opacity: f32) -> Result<(), SVError> {
    let line: SVMark = SVMark::line().width(line_width).opacity(opacity).into();
    line.validate()
}
