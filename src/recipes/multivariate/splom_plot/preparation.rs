//! Column resolution, the missing-observation policy and the axis domains.
//!
//! The observations are shared across the whole matrix, so one observation is
//! either present in every cell or in none. The policy is applied once, here,
//! before any scale is derived: a sparse column is read through
//! [`SVColumnView::value_at`] and is never expanded unless the policy actually
//! drops an observation, in which case the kept values form a shorter filtered
//! copy — fewer values, never a densification of the original storage.

use super::configuration::SVSplomConfiguration;
use super::{SVSplomMissingValuePolicy, SVSplomScaleMode};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// One variable column after the missing-observation policy was applied.
pub(super) enum SVSplomPreparedColumn<'a> {
    /// The original dense, sparse or streaming view, untouched.
    Borrowed(SVColumnView<'a>),
    /// A shorter copy holding only the kept observations.
    Filtered(Vec<f64>),
}

impl SVSplomPreparedColumn<'_> {
    /// Returns the column as one of the three representations.
    pub(super) fn view(&self) -> SVColumnView<'_> {
        match self {
            Self::Borrowed(view) => *view,
            Self::Filtered(values) => SVColumnView::Dense(values),
        }
    }

    /// Returns the value at one prepared index.
    pub(super) fn value_at(&self, index: usize) -> Option<f64> {
        match self {
            Self::Borrowed(view) => view.value_at(index),
            Self::Filtered(values) => values.get(index).copied(),
        }
    }

    /// Returns the number of prepared observations.
    pub(super) fn length(&self) -> usize {
        match self {
            Self::Borrowed(view) => view.length(),
            Self::Filtered(values) => values.len(),
        }
    }
}

/// One colour group of the matrix.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SVSplomColourGroup {
    pub(super) value: f64,
    pub(super) label: String,
    pub(super) colour: SVColour,
}

/// The resolved variables, domains and colour assignment of one render.
pub(super) struct SVSplomPreparedData<'a> {
    pub(super) columns: Vec<SVSplomPreparedColumn<'a>>,
    pub(super) group_indices: Vec<usize>,
    pub(super) groups: Vec<SVSplomColourGroup>,
    pub(super) domains: Vec<(f64, f64)>,
    pub(super) number_of_observations: usize,
}

/// Applies the missing-observation policy and derives the axis domains.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when a variable column disagrees with
/// the first one, [`SVError::NonFiniteValue`] when the policy is `Error` and an
/// observation is not finite, and [`SVError::InvalidConfiguration`] when the
/// source ends before a prepared index.
pub(super) fn sv_prepare_data<'a>(
    configuration: &SVSplomConfiguration<'_>,
    views: &[SVColumnView<'a>],
    colour: Option<SVColumnView<'a>>,
) -> Result<SVSplomPreparedData<'a>, SVError> {
    let number_of_observations = views.first().map_or(0, SVColumnView::length);
    for (index, view) in views.iter().enumerate() {
        if view.length() != number_of_observations {
            return Err(SVError::MismatchedLengths {
                name: configuration
                    .variables
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| format!("variable-{index}")),
                expected: number_of_observations,
                actual: view.length(),
            });
        }
    }
    if let Some(colour) = colour {
        if colour.length() != number_of_observations {
            return Err(SVError::MismatchedLengths {
                name: configuration
                    .colour
                    .clone()
                    .unwrap_or_else(|| "colour".to_string()),
                expected: number_of_observations,
                actual: colour.length(),
            });
        }
    }

    let kept = sv_kept_observations(configuration, views, colour)?;
    let columns = if kept.len() == number_of_observations {
        views
            .iter()
            .map(|view| SVSplomPreparedColumn::Borrowed(*view))
            .collect()
    } else {
        sv_filtered_columns(views, &kept)?
    };

    let (groups, group_indices) = sv_colour_groups(colour, &kept)?;
    let domains = sv_domains(configuration.scale_mode, &columns)?;

    Ok(SVSplomPreparedData {
        columns,
        group_indices,
        groups,
        domains,
        number_of_observations: kept.len(),
    })
}

fn sv_kept_observations(
    configuration: &SVSplomConfiguration<'_>,
    views: &[SVColumnView<'_>],
    colour: Option<SVColumnView<'_>>,
) -> Result<Vec<usize>, SVError> {
    let number_of_observations = views.first().map_or(0, SVColumnView::length);
    let mut kept = Vec::with_capacity(number_of_observations);
    for index in 0..number_of_observations {
        let finite = views
            .iter()
            .all(|view| view.value_at(index).is_some_and(|value| value.is_finite()))
            && colour.is_none_or(|colour| {
                colour
                    .value_at(index)
                    .is_some_and(|value| value.is_finite())
            });
        if finite {
            kept.push(index);
            continue;
        }
        if configuration.missing_value_policy == SVSplomMissingValuePolicy::Error {
            let name = sv_non_finite_column(views, colour, index, &configuration.variables);
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the column `{name}` has a non-finite value at observation {index}; the missing-value policy is Error"
                ),
            });
        }
    }
    Ok(kept)
}

fn sv_non_finite_column(
    views: &[SVColumnView<'_>],
    colour: Option<SVColumnView<'_>>,
    index: usize,
    variables: &[String],
) -> String {
    for (position, view) in views.iter().enumerate() {
        if view.value_at(index).is_none_or(|value| !value.is_finite()) {
            return variables
                .get(position)
                .cloned()
                .unwrap_or_else(|| format!("variable-{position}"));
        }
    }
    if colour.is_some_and(|colour| {
        colour
            .value_at(index)
            .is_none_or(|value| !value.is_finite())
    }) {
        return "colour".to_string();
    }
    "observation".to_string()
}

fn sv_filtered_columns<'a>(
    views: &[SVColumnView<'a>],
    kept: &[usize],
) -> Result<Vec<SVSplomPreparedColumn<'a>>, SVError> {
    let mut columns = Vec::with_capacity(views.len());
    for view in views {
        let mut values = Vec::with_capacity(kept.len());
        for index in kept {
            let value = view
                .value_at(*index)
                .ok_or_else(|| SVError::InvalidConfiguration {
                    message: format!("the variable column ended before observation {index}"),
                })?;
            values.push(value);
        }
        columns.push(SVSplomPreparedColumn::Filtered(values));
    }
    Ok(columns)
}

fn sv_colour_groups(
    colour: Option<SVColumnView<'_>>,
    kept: &[usize],
) -> Result<(Vec<SVSplomColourGroup>, Vec<usize>), SVError> {
    let Some(colour) = colour else {
        return Ok((
            vec![SVSplomColourGroup {
                value: 0.0,
                label: String::new(),
                colour: SVColour::new(9, 105, 218, 255),
            }],
            Vec::new(),
        ));
    };
    let palette = sv_default_scale_palette();
    let mut groups: Vec<SVSplomColourGroup> = Vec::new();
    let mut indices = Vec::with_capacity(kept.len());
    for index in kept {
        let value = colour
            .value_at(*index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the colour column ended before observation {index}"),
            })?;
        let group_index = match groups.iter().position(|group| group.value == value) {
            Some(group_index) => group_index,
            None => {
                groups.push(SVSplomColourGroup {
                    value,
                    label: sv_group_label(value),
                    colour: palette.colour_at_index(groups.len()),
                });
                groups.len() - 1
            }
        };
        indices.push(group_index);
    }
    Ok((groups, indices))
}

fn sv_group_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

fn sv_domains(
    scale_mode: SVSplomScaleMode,
    columns: &[SVSplomPreparedColumn<'_>],
) -> Result<Vec<(f64, f64)>, SVError> {
    let own = columns
        .iter()
        .map(sv_column_domain)
        .collect::<Result<Vec<_>, _>>()?;
    if scale_mode == SVSplomScaleMode::Independent {
        return Ok(own);
    }
    let shared = own
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |accumulated, domain| {
            (accumulated.0.min(domain.0), accumulated.1.max(domain.1))
        });
    let shared = sv_pad_domain(shared.0, shared.1);
    Ok(vec![shared; columns.len()])
}

fn sv_column_domain(column: &SVSplomPreparedColumn<'_>) -> Result<(f64, f64), SVError> {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for index in 0..column.length() {
        let value = column
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the variable column ended before observation {index}"),
            })?;
        low = low.min(value);
        high = high.max(value);
    }
    Ok(sv_pad_domain(low, high))
}

fn sv_pad_domain(low: f64, high: f64) -> (f64, f64) {
    if !low.is_finite() || !high.is_finite() {
        return (0.0, 1.0);
    }
    if high - low > f64::EPSILON {
        return (low, high);
    }
    let padding = if low.abs() <= f64::EPSILON {
        1.0
    } else {
        low.abs() * 0.1
    };
    (low - padding, high + padding)
}
