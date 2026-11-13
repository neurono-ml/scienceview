//! Resolving the documented columns into the observations the recipe draws.
//!
//! The value column decides which logical positions are observations: a dense
//! column contributes every row, a sparse column only its stored entries — the
//! implicit zeros are never expanded — and a streaming column its current
//! window. The category and series columns are read at those same positions, so
//! a sparse input is never densified (patterns P6).

use super::configuration::SVStripConfiguration;
use crate::sv_data::{SVColumnRole, SVColumnView, SVDataSource, SVSourceSchema};
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// One drawn observation: its value, category slot and series group.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SVStripObservation {
    pub(super) row: usize,
    pub(super) value: f64,
    pub(super) category_index: usize,
    pub(super) group_index: usize,
}

/// One category slot, in first-appearance order.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVStripCategory {
    pub(super) code: f64,
    pub(super) label: String,
}

/// One coloured series, in first-appearance order.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVStripSeriesGroup {
    pub(super) code: f64,
    pub(super) label: String,
    pub(super) colour: SVColour,
}

/// Everything one resolution produced.
pub(super) struct SVStripData {
    pub(super) observations: Vec<SVStripObservation>,
    pub(super) categories: Vec<SVStripCategory>,
    pub(super) groups: Vec<SVStripSeriesGroup>,
}

/// Validates that the configured columns exist with the documented roles.
pub(super) fn sv_validate_columns(
    data_source: &dyn SVDataSource,
    value: &str,
    category: &str,
    series: Option<&str>,
) -> Result<(), SVError> {
    let schema = data_source.schema();
    sv_require_role(&schema, value, &[SVColumnRole::Y, SVColumnRole::X], "value")?;
    sv_require_role(
        &schema,
        category,
        &[SVColumnRole::Grouping, SVColumnRole::X, SVColumnRole::Label],
        "category",
    )?;
    if let Some(series) = series {
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
        return Ok(());
    }
    Err(SVError::InvalidConfiguration {
        message: format!("the `{role_name}` column `{name}` has the wrong role for a strip plot"),
    })
}

/// Resolves the configuration into observations.
pub(super) fn sv_resolve_strip_data(
    configuration: &SVStripConfiguration<'_>,
) -> Result<SVStripData, SVError> {
    let mut value_snapshot = configuration.data_source.column(&configuration.value)?;
    let mut category_snapshot = configuration.data_source.column(&configuration.category)?;
    let mut series_snapshot = configuration
        .series
        .as_deref()
        .map(|name| configuration.data_source.column(name))
        .transpose()?;

    let value_view = value_snapshot.column_view();
    let category_view = category_snapshot.column_view();
    let series_view = series_snapshot
        .as_mut()
        .map(|snapshot| snapshot.column_view());
    sv_require_same_length("value", value_view, "category", category_view)?;
    if let Some(series_view) = series_view {
        sv_require_same_length("value", value_view, "series", series_view)?;
    }

    let positions = sv_positions(value_view);
    let policy = configuration.missing_value_policy;
    let mut categories: Vec<SVStripCategory> = Vec::new();
    let mut groups: Vec<SVStripSeriesGroup> = Vec::new();
    let mut observations = Vec::with_capacity(positions.len());

    for row in positions {
        let Some(value) = value_view.value_at(row) else {
            continue;
        };
        if !value.is_finite() {
            policy.report(row)?;
            continue;
        }
        let Some(category) = category_view.value_at(row) else {
            continue;
        };
        if !category.is_finite() {
            policy.report(row)?;
            continue;
        }
        let series = match series_view {
            None => None,
            Some(series_view) => {
                let Some(series) = series_view.value_at(row) else {
                    continue;
                };
                if !series.is_finite() {
                    policy.report(row)?;
                    continue;
                }
                Some(series)
            }
        };

        let category_index = sv_category_index(&mut categories, category);
        let group_index = sv_group_index(&mut groups, series, &configuration.series_labels);
        observations.push(SVStripObservation {
            row,
            value,
            category_index,
            group_index,
        });
    }

    Ok(SVStripData {
        observations,
        categories,
        groups,
    })
}

fn sv_positions(view: SVColumnView<'_>) -> Vec<usize> {
    view.dispatch(
        |dense| (0..dense.len()).collect(),
        |sparse| sparse.indices().to_vec(),
        |streaming| (0..streaming.length()).collect(),
    )
}

fn sv_require_same_length(
    first_name: &str,
    first: SVColumnView<'_>,
    second_name: &str,
    second: SVColumnView<'_>,
) -> Result<(), SVError> {
    if first.length() == second.length() {
        return Ok(());
    }
    Err(SVError::MismatchedLengths {
        name: format!("{second_name} disagrees with {first_name}"),
        expected: first.length(),
        actual: second.length(),
    })
}

fn sv_category_index(categories: &mut Vec<SVStripCategory>, code: f64) -> usize {
    if let Some(index) = categories
        .iter()
        .position(|category| category.code.to_bits() == code.to_bits())
    {
        return index;
    }
    categories.push(SVStripCategory {
        code,
        label: sv_category_label(code),
    });
    categories.len() - 1
}

fn sv_group_index(
    groups: &mut Vec<SVStripSeriesGroup>,
    code: Option<f64>,
    labels: &[String],
) -> usize {
    let code = code.unwrap_or(0.0);
    if let Some(index) = groups
        .iter()
        .position(|group| group.code.to_bits() == code.to_bits())
    {
        return index;
    }
    let colour = if groups.is_empty() && labels.is_empty() {
        crate::sv_mark::SVMarkTheme::default().mark_colour()
    } else {
        sv_default_scale_palette().colour_at_index(groups.len())
    };
    let label = labels
        .get(groups.len())
        .cloned()
        .unwrap_or_else(|| sv_series_label(code));
    groups.push(SVStripSeriesGroup {
        code,
        label,
        colour,
    });
    groups.len() - 1
}

fn sv_category_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("category {}", code as i64)
    } else {
        format!("category {code}")
    }
}

fn sv_series_label(code: f64) -> String {
    if code.fract() == 0.0 {
        format!("{code:.0}")
    } else {
        format!("{code}")
    }
}
