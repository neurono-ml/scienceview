//! The single validation point of the parallel categories builder.

use super::super::configuration::{SVParallelCategoriesConfiguration, sv_invalid};
use super::super::core_implementation::SVParallelCategoriesPlot;
use super::SVParallelCategoriesPlotBuilder;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_data::{SVColumnRole, SVDataSource, SVRepresentation, SVSourceSchema};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkKind;

impl<'data> SVParallelCategoriesPlotBuilder<'data> {
    /// Validates the configuration and builds the plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step, an
    /// [`SVError::InvalidConfiguration`] for fewer than two variables, a
    /// duplicate variable, an out-of-range option or labels for an unknown
    /// variable, [`SVError::ColumnNotFound`] for a column the source does not
    /// expose, a role error for a column with the wrong kind,
    /// [`SVError::MismatchedLengths`] when the columns disagree, and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw ribbons.
    pub fn build(self) -> Result<SVParallelCategoriesPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        sv_validate_variables(&self.variables)?;
        sv_validate_options(self.opacity, self.band_gap, self.label_angle)?;
        sv_validate_label_variables(&self.variables, &self.category_labels)?;
        sv_verify_mark_supported(SVMarkKind::Ribbon, backend_kind)?;

        let schema = data_source.schema();
        for variable in &self.variables {
            sv_require_role(&schema, variable, "variable")?;
        }
        if let Some(colour) = self.colour.as_deref() {
            sv_require_role(&schema, colour, "colour")?;
        }

        let mut names: Vec<&str> = self.variables.iter().map(String::as_str).collect();
        names.extend(self.colour.as_deref());
        let (number_of_observations, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVParallelCategoriesPlot {
            configuration: SVParallelCategoriesConfiguration {
                data_source,
                variables: self.variables,
                variable_labels: self.variable_labels,
                category_labels: self.category_labels,
                colour: self.colour,
                colour_mode: self.colour_mode,
                opacity: self.opacity,
                band_gap: self.band_gap,
                category_order: self.category_order,
                label_angle: self.label_angle,
                missing_value_policy: self.missing_value_policy,
                title: self.title,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_observations,
                representation,
            },
        })
    }
}

fn sv_validate_variables(variables: &[String]) -> Result<(), SVError> {
    if variables.len() < 2 {
        return Err(sv_invalid(
            "SVParallelCategoriesPlot requires at least two `variables`; call `variables(&[\"<column>\", ...])`",
        ));
    }
    for (index, variable) in variables.iter().enumerate() {
        if variables.iter().take(index).any(|seen| seen == variable) {
            return Err(sv_invalid(&format!(
                "the variable `{variable}` is listed more than once"
            )));
        }
    }
    Ok(())
}

fn sv_validate_options(opacity: f32, band_gap: f32, label_angle: f32) -> Result<(), SVError> {
    if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
        return Err(sv_invalid(
            "the parallel categories opacity must be a finite number from zero to one",
        ));
    }
    if !band_gap.is_finite() || !(0.0..=1.0).contains(&band_gap) {
        return Err(sv_invalid(
            "the parallel categories band gap must be a finite fraction from zero to one",
        ));
    }
    if !label_angle.is_finite() {
        return Err(sv_invalid(
            "the parallel categories label angle must be a finite number of degrees",
        ));
    }
    Ok(())
}

fn sv_validate_label_variables(
    variables: &[String],
    labels: &[(String, Vec<String>)],
) -> Result<(), SVError> {
    for (variable, _) in labels {
        if !variables.iter().any(|candidate| candidate == variable) {
            return Err(sv_invalid(&format!(
                "category labels were supplied for the unknown variable `{variable}`"
            )));
        }
    }
    Ok(())
}

fn sv_require_role(
    schema: &SVSourceSchema<'_>,
    name: &str,
    role_name: &str,
) -> Result<(), SVError> {
    let Some(descriptor) = schema.column(name) else {
        return Err(SVError::ColumnNotFound {
            name: name.to_string(),
        });
    };
    let allowed = [
        SVColumnRole::Grouping,
        SVColumnRole::Label,
        SVColumnRole::X,
        SVColumnRole::Y,
    ];
    if allowed.contains(&descriptor.role()) {
        Ok(())
    } else {
        Err(sv_invalid(&format!(
            "the `{role_name}` column `{name}` has the wrong role for a parallel categories plot"
        )))
    }
}

/// Probes the named columns once for existence and equal length.
fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected_length: Option<usize> = None;
    let mut representation = SVRepresentation::Dense;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let representation_of_column = snapshot.representation();
        let view = snapshot.column_view();
        match expected_length {
            None => expected_length = Some(view.length()),
            Some(length) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some(_) => {}
        }
        representation = sv_combine_representation(representation, representation_of_column);
    }
    Ok((expected_length.unwrap_or(0), representation))
}

fn sv_combine_representation(left: SVRepresentation, right: SVRepresentation) -> SVRepresentation {
    match (left, right) {
        (SVRepresentation::Streaming, _) | (_, SVRepresentation::Streaming) => {
            SVRepresentation::Streaming
        }
        (SVRepresentation::Sparse, _) | (_, SVRepresentation::Sparse) => SVRepresentation::Sparse,
        _ => SVRepresentation::Dense,
    }
}
