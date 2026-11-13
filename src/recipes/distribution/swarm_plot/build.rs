//! The single validation point of the `SVSwarmPlot` builder.

use super::builder::SVSwarmPlotBuilder;
use super::configuration::SVSwarmConfiguration;
use super::core_implementation::SVSwarmPlot;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_data::{SVColumnRole, SVSourceSchema};
use crate::sv_error::SVError;
use crate::sv_mark::{SVMark, SVMarkKind};

impl<'data> SVSwarmPlotBuilder<'data> {
    /// Validates the configuration and builds the swarm.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step,
    /// [`SVError::InvalidConfiguration`] for a missing required option, a wrong
    /// column role, an impossible opacity or swarm width,
    /// [`SVError::ColumnNotFound`] for a column the source does not expose,
    /// [`SVError::MismatchedLengths`] when the columns disagree and
    /// [`SVError::UnsupportedMark`] when the backend cannot draw dots.
    pub fn build(self) -> Result<SVSwarmPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .clone()
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: "SVSwarmPlot requires a `value` column; call `value(\"<column>\")`"
                    .to_string(),
            })?;
        let category = self
            .category
            .clone()
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: "SVSwarmPlot requires a `category` column; call `category(\"<column>\")`"
                    .to_string(),
            })?;

        sv_validate_options(
            &self,
            backend_kind,
            &data_source.schema(),
            &value,
            &category,
        )?;

        let mut value_snapshot = data_source.column(&value)?;
        let value_view = value_snapshot.column_view();
        let number_of_points = value_view.length();
        let representation = value_view.representation();
        let mut representations = vec![representation];
        for name in std::iter::once(&category).chain(self.series.as_ref()) {
            let mut snapshot = data_source.column(name)?;
            let view = snapshot.column_view();
            if view.length() != number_of_points {
                return Err(SVError::MismatchedLengths {
                    name: name.clone(),
                    expected: number_of_points,
                    actual: view.length(),
                });
            }
            representations.push(view.representation());
        }

        Ok(SVSwarmPlot {
            configuration: SVSwarmConfiguration {
                data_source,
                value,
                category,
                series: self.series,
                series_labels: self.series_labels,
                marker: self.marker,
                size: self.size,
                orientation: self.orientation,
                swarm_width: self.swarm_width,
                alpha: self.alpha,
                missing_value_policy: self.missing_value_policy,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_points,
                representation: sv_widest(&representations),
            },
        })
    }
}

fn sv_validate_options(
    builder: &SVSwarmPlotBuilder<'_>,
    backend_kind: crate::sv_backend::SVBackendKind,
    schema: &SVSourceSchema<'_>,
    value: &str,
    category: &str,
) -> Result<(), SVError> {
    sv_require_role(schema, value, &[SVColumnRole::Y, SVColumnRole::X], "value")?;
    sv_require_role(
        schema,
        category,
        &[SVColumnRole::Grouping, SVColumnRole::X, SVColumnRole::Label],
        "category",
    )?;
    if let Some(series) = builder.series.as_deref() {
        sv_require_role(
            schema,
            series,
            &[SVColumnRole::Grouping, SVColumnRole::Y],
            "series",
        )?;
    }

    if let Some(width) = builder.swarm_width {
        let valid = width.is_finite() && width > 0.0 && width <= 1.0;
        if !valid {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the swarm width is a fraction of the category slot between 0 and 1, received {width}"
                ),
            });
        }
    }

    let mark: SVMark = SVMark::dot()
        .shape(builder.marker)
        .opacity(builder.alpha)
        .into();
    mark.validate()?;
    sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)
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
        return Ok(());
    }
    Err(SVError::InvalidConfiguration {
        message: format!("the `{role_name}` column `{name}` has the wrong role for a swarm plot"),
    })
}

fn sv_widest(
    representations: &[crate::sv_data::SVRepresentation],
) -> crate::sv_data::SVRepresentation {
    use crate::sv_data::SVRepresentation;
    representations
        .iter()
        .copied()
        .fold(SVRepresentation::Dense, |widest, candidate| {
            match (sv_width(candidate), sv_width(widest)) {
                (left, right) if left > right => candidate,
                _ => widest,
            }
        })
}

fn sv_width(representation: crate::sv_data::SVRepresentation) -> u8 {
    use crate::sv_data::SVRepresentation;
    match representation {
        SVRepresentation::Dense => 0,
        SVRepresentation::Sparse => 1,
        SVRepresentation::Streaming => 2,
    }
}
