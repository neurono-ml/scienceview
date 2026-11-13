//! Resolving the joint columns once, preserving the representation.
//!
//! A dense or streaming column is copied into a finite-only buffer; a sparse
//! column keeps its stored entries and is never expanded into a dense value
//! buffer.

use super::super::SVJointMissingValuePolicy;
use super::super::channels::{SVJointSeriesGroup, sv_series_groups};
use crate::sv_data::{SVColumnView, SVRepresentation, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkTheme;

/// The two columns and the series group of every observation.
pub(super) struct SVJointColumns {
    pub(super) horizontal: SVOwnedColumn,
    pub(super) vertical: SVOwnedColumn,
    pub(super) group_indices: Vec<usize>,
    pub(super) groups: Vec<SVJointSeriesGroup>,
    pub(super) series_labels: Vec<String>,
    pub(super) number_of_series: usize,
}

/// One resolved column, either a finite-only dense buffer or a sparse view.
pub(super) enum SVOwnedColumn {
    /// A contiguous buffer of finite values.
    Dense(Vec<f64>),
    /// Stored entries with their logical length, never expanded.
    Sparse {
        /// The logical length of the column.
        length: usize,
        /// The stored indices.
        indices: Vec<usize>,
        /// The stored values.
        values: Vec<f64>,
    },
}

impl SVOwnedColumn {
    /// Borrows the column as the data boundary's view type.
    ///
    /// # Errors
    ///
    /// The stored sparse entries were validated when the column was resolved,
    /// so the call only reports a malformed view if the boundary rejects it.
    pub(super) fn as_view(&self) -> Result<SVColumnView<'_>, SVError> {
        match self {
            Self::Dense(values) => Ok(SVColumnView::Dense(values)),
            Self::Sparse {
                length,
                indices,
                values,
            } => Ok(SVColumnView::Sparse(SVSparseView::new(
                indices, values, *length,
            )?)),
        }
    }
}

/// Resolves the required columns and drops the observations the policy omits.
pub(super) fn sv_resolve_columns(
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
    series: Option<SVColumnView<'_>>,
    policy: SVJointMissingValuePolicy,
    horizontal_name: &str,
    vertical_name: &str,
) -> Result<SVJointColumns, SVError> {
    let number_of_points = horizontal.length();
    if vertical.length() != number_of_points {
        return Err(SVError::MismatchedLengths {
            name: format!("{vertical_name} (against {horizontal_name})"),
            expected: number_of_points,
            actual: vertical.length(),
        });
    }
    if let Some(series) = series {
        if series.length() != number_of_points {
            return Err(SVError::MismatchedLengths {
                name: "series (against x)".to_string(),
                expected: number_of_points,
                actual: series.length(),
            });
        }
    }

    let horizontal_view = horizontal;
    let vertical_view = vertical;
    sv_check_finite(
        horizontal_view,
        vertical_view,
        policy,
        horizontal_name,
        vertical_name,
    )?;
    sv_check_finite(
        vertical_view,
        horizontal_view,
        policy,
        vertical_name,
        horizontal_name,
    )?;

    let (groups, group_indices) = sv_series_groups(series, number_of_points, default_colour())?;
    let series_labels = groups
        .iter()
        .map(|group| group.label.clone())
        .filter(|label| !label.is_empty())
        .collect();
    let number_of_series = if series.is_some() { groups.len() } else { 0 };
    let sparse_input = horizontal_view.representation() == SVRepresentation::Sparse
        || vertical_view.representation() == SVRepresentation::Sparse;
    let group_indices = if series.is_some() && !sparse_input {
        (0..number_of_points)
            .filter(|index| sv_pair_is_finite(horizontal_view, vertical_view, *index))
            .map(|index| group_indices.get(index).copied().unwrap_or(0))
            .collect()
    } else {
        group_indices
    };

    let horizontal = sv_finite_column(horizontal_view, vertical_view);
    let vertical = sv_finite_column(vertical_view, horizontal_view);

    Ok(SVJointColumns {
        horizontal,
        vertical,
        group_indices,
        groups,
        series_labels,
        number_of_series,
    })
}

fn default_colour() -> crate::sv_scale::SVColour {
    SVMarkTheme::default().mark_colour()
}

fn sv_check_finite(
    primary: SVColumnView<'_>,
    secondary: SVColumnView<'_>,
    policy: SVJointMissingValuePolicy,
    primary_name: &str,
    _secondary_name: &str,
) -> Result<(), SVError> {
    for index in 0..primary.length() {
        let value = primary.value_at(index).unwrap_or(f64::NAN);
        let partner = secondary.value_at(index).unwrap_or(f64::NAN);
        if value.is_finite() && partner.is_finite() {
            continue;
        }
        if primary.representation() == SVRepresentation::Sparse
            || secondary.representation() == SVRepresentation::Sparse
        {
            if policy == SVJointMissingValuePolicy::Error {
                policy.report(primary_name, index)?;
            }
            return Err(SVError::UnsupportedRepresentation {
                operation: "joint_plot",
                representation: SVRepresentation::Sparse,
                alternative: "drop the non-finite pair in the caller or pass a dense/windowed column",
            });
        }
        policy.report(primary_name, index)?;
    }
    Ok(())
}

fn sv_finite_column(primary: SVColumnView<'_>, secondary: SVColumnView<'_>) -> SVOwnedColumn {
    if let Some(sparse) = primary.sparse_view() {
        return SVOwnedColumn::Sparse {
            length: sparse.length(),
            indices: sparse.indices().to_vec(),
            values: sparse.values().to_vec(),
        };
    }

    let mut values = Vec::with_capacity(primary.length());
    for index in 0..primary.length() {
        let value = primary.value_at(index).unwrap_or(f64::NAN);
        let partner = secondary.value_at(index).unwrap_or(f64::NAN);
        if value.is_finite() && partner.is_finite() {
            values.push(value);
        }
    }
    SVOwnedColumn::Dense(values)
}

fn sv_pair_is_finite(
    horizontal: SVColumnView<'_>,
    vertical: SVColumnView<'_>,
    index: usize,
) -> bool {
    horizontal.value_at(index).is_some_and(f64::is_finite)
        && vertical.value_at(index).is_some_and(f64::is_finite)
}
