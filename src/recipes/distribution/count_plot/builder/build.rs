//! The single validation point of the count-plot builder.

use super::super::configuration::{SVCountConfiguration, sv_invalid};
use super::super::core_implementation::SVCountPlot;
use super::SVCountPlotBuilder;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_data::{SVColumnRole, SVDataSource, SVRepresentation, SVSourceSchema};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkKind;

impl<'data> SVCountPlotBuilder<'data> {
    /// Validates the configuration and builds the count plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step, a
    /// configuration error for a missing category column, a configured weight
    /// (the shipped counting kernel does not consume weights) or an impossible
    /// width, [`SVError::ColumnNotFound`] for a column the source does not
    /// expose, a role error for a column with the wrong kind,
    /// [`SVError::MismatchedLengths`] when the columns disagree, and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw bars.
    pub fn build(self) -> Result<SVCountPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let category = self.category.ok_or_else(|| {
            sv_invalid("SVCountPlot requires a `category` column; call `category(\"<column>\")`")
        })?;

        if let Some(weight) = self.weight.as_deref() {
            return Err(sv_invalid(&format!(
                "the `{weight}` weight column cannot be honoured: the shipped `SVStat::Count` kernel does not consume observation weights; weighted counting is a follow-up change to `statistical-transforms`"
            )));
        }
        if !self.width.is_finite() || self.width <= 0.0 || self.width > 1.0 {
            return Err(sv_invalid(
                "the count-plot width must be a finite fraction greater than zero and at most one",
            ));
        }
        sv_verify_mark_supported(SVMarkKind::Bar, backend_kind)?;

        let schema = data_source.schema();
        sv_require_role(
            &schema,
            &category,
            &[SVColumnRole::Grouping, SVColumnRole::X, SVColumnRole::Label],
            "category",
        )?;
        if let Some(series) = self.series.as_deref() {
            sv_require_role(
                &schema,
                series,
                &[SVColumnRole::Grouping, SVColumnRole::Y, SVColumnRole::X],
                "series",
            )?;
        }

        let mut names = vec![category.as_str()];
        names.extend(self.series.as_deref());
        let (number_of_observations, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVCountPlot {
            configuration: SVCountConfiguration {
                data_source,
                category,
                series: self.series,
                category_labels: self.category_labels,
                series_labels: self.series_labels,
                orientation: self.orientation,
                width: self.width,
                normalisation: self.normalisation,
                order: self.order,
                colour: self.colour,
                multiple: self.multiple,
                missing_value_policy: self.missing_value_policy,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_observations,
                representation,
            },
        })
    }
}

fn sv_require_role(
    schema: &SVSourceSchema<'_>,
    name: &str,
    allowed: &[SVColumnRole],
    role_name: &str,
) -> Result<(), SVError> {
    let Some(descriptor) = schema.column(name) else {
        return Err(SVError::ColumnNotFound {
            name: name.to_string(),
        });
    };
    if allowed.contains(&descriptor.role()) {
        Ok(())
    } else {
        Err(sv_invalid(&format!(
            "the `{role_name}` column `{name}` has the wrong role for a count plot"
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
