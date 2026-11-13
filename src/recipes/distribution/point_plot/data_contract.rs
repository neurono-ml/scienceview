//! Resolving the documented columns into the observations the recipe
//! aggregates.
//!
//! The value column decides which logical positions are observations: a dense
//! column contributes every row, a sparse column only its stored entries — the
//! implicit zeros are never expanded — and a streaming column its current
//! window. The category and series columns are read at those same positions, so
//! a sparse input is never densified (patterns P6).

use scienceview_core::sv_data::{
    SVColumnRole, SVColumnSnapshot, SVColumnView, SVRepresentation, SVSourceSchema,
};
use scienceview_core::sv_error::SVError;

use super::configuration::{SVPointPlotConfiguration, sv_invalid};

/// One observation of the value column with its category and series.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVPointObservation {
    pub(crate) category: f64,
    pub(crate) series: Option<f64>,
    pub(crate) value: f64,
}

/// Every observation of one operation, plus the facts the span records.
pub(crate) struct SVPointData {
    pub(crate) observations: Vec<SVPointObservation>,
    pub(crate) representation: SVRepresentation,
    pub(crate) number_of_points: usize,
    pub(crate) is_approximate: bool,
}

/// Resolves the configuration into observations.
pub(crate) fn sv_resolve_point_data(
    configuration: &SVPointPlotConfiguration<'_>,
) -> Result<SVPointData, SVError> {
    sv_validate_columns(configuration)?;
    let mut value = configuration.data_source.column(configuration.value)?;
    let mut category = configuration.data_source.column(configuration.category)?;
    let mut series = configuration
        .series
        .map(|name| configuration.data_source.column(name))
        .transpose()?;

    let value_view = value.column_view();
    let category_view = category.column_view();
    let series_view = series.as_mut().map(SVColumnSnapshot::column_view);
    sv_require_same_length(value_view, category_view, "category is missing")?;
    if let Some(series_view) = series_view {
        sv_require_same_length(value_view, series_view, "series is missing")?;
    }

    let policy = configuration.missing_value_policy;
    let mut observations = Vec::with_capacity(value_view.length());
    for position in sv_positions(value_view) {
        let Some(value) = value_view
            .value_at(position)
            .filter(|value| value.is_finite())
        else {
            policy.report(position)?;
            continue;
        };
        let Some(category) = category_view
            .value_at(position)
            .filter(|category| category.is_finite())
        else {
            policy.report(position)?;
            continue;
        };
        let series = match series_view.and_then(|view| view.value_at(position)) {
            Some(series) if series.is_finite() => Some(series),
            Some(_) => {
                policy.report(position)?;
                continue;
            }
            None => None,
        };
        observations.push(SVPointObservation {
            category,
            series,
            value,
        });
    }
    if observations.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "point plot",
        });
    }

    let representation = sv_widest(
        value_view.representation(),
        category_view.representation(),
        series_view.map_or(SVRepresentation::Dense, |view| view.representation()),
    );
    Ok(SVPointData {
        number_of_points: observations.len(),
        observations,
        representation,
        is_approximate: representation == SVRepresentation::Streaming,
    })
}

/// Validates that the configured columns exist with the documented roles.
pub(crate) fn sv_validate_columns(
    configuration: &SVPointPlotConfiguration<'_>,
) -> Result<(), SVError> {
    let schema = configuration.data_source.schema();
    sv_require_role(
        &schema,
        configuration.value,
        &[SVColumnRole::Y, SVColumnRole::X],
        "value",
    )?;
    sv_require_role(
        &schema,
        configuration.category,
        &[SVColumnRole::Grouping, SVColumnRole::X, SVColumnRole::Label],
        "category",
    )?;
    if let Some(series) = configuration.series {
        sv_require_role(
            &schema,
            series,
            &[SVColumnRole::Grouping, SVColumnRole::Y],
            "series",
        )?;
    }
    Ok(())
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
            "the `{role_name}` column `{name}` has the wrong role for a point plot"
        )))
    }
}

fn sv_positions(view: SVColumnView<'_>) -> Vec<usize> {
    view.dispatch(
        |dense| (0..dense.len()).collect(),
        |sparse| sparse.indices().to_vec(),
        |streaming| (0..streaming.length()).collect(),
    )
}

fn sv_require_same_length(
    value: SVColumnView<'_>,
    column: SVColumnView<'_>,
    message: &str,
) -> Result<(), SVError> {
    if value.length() == column.length() {
        Ok(())
    } else {
        Err(SVError::MismatchedLengths {
            name: message.to_string(),
            expected: value.length(),
            actual: column.length(),
        })
    }
}

fn sv_widest(
    value: SVRepresentation,
    category: SVRepresentation,
    series: SVRepresentation,
) -> SVRepresentation {
    [value, category, series]
        .into_iter()
        .fold(SVRepresentation::Dense, |widest, candidate| {
            if sv_width(candidate) > sv_width(widest) {
                candidate
            } else {
                widest
            }
        })
}

fn sv_width(representation: SVRepresentation) -> u8 {
    match representation {
        SVRepresentation::Dense => 0,
        SVRepresentation::Sparse => 1,
        SVRepresentation::Streaming => 2,
    }
}

/// Formats a category code as the label the axis shows.
pub(crate) fn sv_category_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("category {}", code as i64)
    } else {
        format!("category {code}")
    }
}

/// Formats a series code as the label the legend shows.
pub(crate) fn sv_series_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("series {}", code as i64)
    } else {
        format!("series {code}")
    }
}
