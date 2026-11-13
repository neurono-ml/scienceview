//! Turning the configured columns into one observation per stored row.

use scienceview_core::sv_data::{SVColumnSnapshot, SVColumnView, SVRepresentation};
use scienceview_core::sv_error::SVError;

use super::super::configuration::SVViolinPlotConfiguration;
use super::validation::{sv_positions, sv_validate_columns};

/// One observation of the value column with its category and series codes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVViolinObservation {
    pub(crate) category: f64,
    pub(crate) series: Option<f64>,
    pub(crate) value: f64,
}

/// Every observation of one operation, plus the facts the span records.
pub(crate) struct SVViolinData {
    pub(crate) observations: Vec<SVViolinObservation>,
    pub(crate) representation: SVRepresentation,
    pub(crate) number_of_points: usize,
    pub(crate) is_approximate: bool,
}

/// Resolves the configuration into observations.
pub(crate) fn sv_resolve_violin_data(
    configuration: &SVViolinPlotConfiguration<'_>,
) -> Result<SVViolinData, SVError> {
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
    sv_require_same_length(value_view, category_view, "the category column length")?;
    if let Some(series_view) = series_view {
        sv_require_same_length(value_view, series_view, "the series column length")?;
    }

    let positions = sv_positions(value_view);
    let mut observations = Vec::with_capacity(positions.len());
    for row in positions {
        let Some(value) = value_view.value_at(row) else {
            continue;
        };
        if !value.is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.value, row)?;
            continue;
        }
        let Some(category) = category_view.value_at(row) else {
            continue;
        };
        if !category.is_finite() {
            configuration
                .missing_value_policy
                .report(configuration.category, row)?;
            continue;
        }
        let series = series_view.and_then(|view| view.value_at(row));
        if let Some(series) = series {
            if !series.is_finite() {
                configuration
                    .missing_value_policy
                    .report(configuration.series.unwrap_or("series"), row)?;
                continue;
            }
        }
        observations.push(SVViolinObservation {
            category,
            series,
            value,
        });
    }
    if observations.is_empty() {
        return Err(SVError::EmptyInput {
            operation: "violin plot",
        });
    }

    let representation = sv_widest(
        value_view.representation(),
        category_view.representation(),
        series_view.map(|view| view.representation()),
    );
    Ok(SVViolinData {
        number_of_points: observations.len(),
        observations,
        representation,
        is_approximate: representation == SVRepresentation::Streaming,
    })
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
    series: Option<SVRepresentation>,
) -> SVRepresentation {
    [Some(value), Some(category), series]
        .into_iter()
        .flatten()
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
