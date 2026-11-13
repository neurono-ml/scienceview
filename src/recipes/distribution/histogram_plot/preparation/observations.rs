//! Resolving the value column into one observation group per series.
//!
//! A group keeps the representation it was given. A dense or streaming column
//! is filtered into an owned buffer only when the missing-value policy has to
//! drop something; a sparse column keeps sparse storage, with the implicit
//! zeros of the group still counted as observations.

use super::super::SVHistogramMissingValuePolicy;
use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// The observations of one series, in the shape they arrived in.
pub(super) enum SVObservationStorage<'a> {
    /// The original dense, sparse or streaming view, untouched.
    Borrowed(SVColumnView<'a>),
    /// A dense buffer that dropped non-finite values.
    Dense(Vec<f64>),
    /// Sparse storage that dropped non-finite stored values.
    Sparse {
        indices: Vec<usize>,
        values: Vec<f64>,
        length: usize,
    },
}

impl SVObservationStorage<'_> {
    /// Returns the view the bin kernel consumes.
    pub(super) fn view(&self) -> Result<SVColumnView<'_>, SVError> {
        match self {
            SVObservationStorage::Borrowed(view) => Ok(*view),
            SVObservationStorage::Dense(values) => Ok(SVColumnView::Dense(values)),
            SVObservationStorage::Sparse {
                indices,
                values,
                length,
            } => Ok(SVColumnView::Sparse(SVSparseView::new(
                indices, values, *length,
            )?)),
        }
    }

    /// Returns the number of logical observations.
    pub(super) fn length(&self) -> usize {
        match self {
            SVObservationStorage::Borrowed(view) => view.length(),
            SVObservationStorage::Dense(values) => values.len(),
            SVObservationStorage::Sparse { length, .. } => *length,
        }
    }
}

/// One series of observations, with its resolved label and colour.
pub(super) struct SVObservationGroup<'a> {
    pub(super) label: String,
    pub(super) colour: SVColour,
    pub(super) storage: SVObservationStorage<'a>,
}

/// Builds one observation group per series.
pub(super) fn sv_observation_groups<'a>(
    value: SVColumnView<'a>,
    series: Option<SVColumnView<'a>>,
    policy: SVHistogramMissingValuePolicy,
    labels: &[String],
) -> Result<Vec<SVObservationGroup<'a>>, SVError> {
    let palette = sv_default_scale_palette();
    match series {
        None => Ok(vec![SVObservationGroup {
            label: String::new(),
            colour: palette.colour_at_index(0),
            storage: sv_whole_storage(value, policy)?,
        }]),
        Some(series) => {
            let groups = sv_series_membership(series, value.length())?;
            groups
                .into_iter()
                .enumerate()
                .map(|(index, membership)| {
                    let label = labels.get(index).cloned().unwrap_or(membership.label);
                    Ok(SVObservationGroup {
                        label,
                        colour: palette.colour_at_index(index),
                        storage: sv_group_storage(value, &membership.indices, policy)?,
                    })
                })
                .collect()
        }
    }
}

struct SVSeriesMembership {
    label: String,
    value: f64,
    indices: Vec<usize>,
}

fn sv_series_membership(
    series: SVColumnView<'_>,
    number_of_points: usize,
) -> Result<Vec<SVSeriesMembership>, SVError> {
    let mut groups: Vec<SVSeriesMembership> = Vec::new();
    for index in 0..number_of_points {
        let value = series
            .value_at(index)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!("the series column ended before point {index}"),
            })?;
        if !value.is_finite() {
            return Err(SVError::NonFiniteValue {
                operation: "histogram_series",
                position: index,
            });
        }
        let position = match groups.iter().position(|group| group.value == value) {
            Some(position) => position,
            None => {
                groups.push(SVSeriesMembership {
                    label: sv_series_label(value),
                    value,
                    indices: Vec::new(),
                });
                groups.len() - 1
            }
        };
        if let Some(group) = groups.get_mut(position) {
            group.indices.push(index);
        }
    }
    Ok(groups)
}

fn sv_series_label(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

fn sv_whole_storage(
    view: SVColumnView<'_>,
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'_>, SVError> {
    match view {
        SVColumnView::Dense(values) => sv_dense_storage(values, policy),
        SVColumnView::Streaming(streaming) => sv_streaming_storage(streaming, policy),
        SVColumnView::Sparse(sparse) => sv_sparse_storage(sparse, policy),
    }
}

fn sv_group_storage<'a>(
    view: SVColumnView<'a>,
    indices: &[usize],
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'a>, SVError> {
    match view {
        SVColumnView::Dense(values) => {
            let selected = indices
                .iter()
                .filter_map(|index| values.get(*index).copied())
                .collect();
            sv_owned_dense(selected, indices, policy)
        }
        SVColumnView::Streaming(streaming) => {
            let values = streaming.values();
            let selected = indices
                .iter()
                .filter_map(|index| values.get(*index).copied())
                .collect();
            sv_owned_dense(selected, indices, policy)
        }
        SVColumnView::Sparse(sparse) => sv_group_sparse(sparse, indices, policy),
    }
}

fn sv_owned_dense<'a>(
    values: Vec<f64>,
    original_indices: &[usize],
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'a>, SVError> {
    let mut kept = Vec::with_capacity(values.len());
    for (position, value) in values.into_iter().enumerate() {
        if !value.is_finite() {
            let original = original_indices.get(position).copied().unwrap_or(position);
            policy.report("histogram_values", original)?;
            if policy.omits() {
                continue;
            }
        }
        kept.push(value);
    }
    Ok(SVObservationStorage::Dense(kept))
}

fn sv_dense_storage(
    values: &[f64],
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'_>, SVError> {
    if let Some(position) = values.iter().position(|value| !value.is_finite()) {
        policy.report("histogram_values", position)?;
    }
    let has_missing = values.iter().any(|value| !value.is_finite());
    if !has_missing {
        return Ok(SVObservationStorage::Borrowed(SVColumnView::Dense(values)));
    }
    let kept = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    Ok(SVObservationStorage::Dense(kept))
}

fn sv_streaming_storage(
    streaming: SVStreamingView<'_>,
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'_>, SVError> {
    let values = streaming.values();
    if let Some(position) = values.iter().position(|value| !value.is_finite()) {
        policy.report("histogram_values", position)?;
    }
    let has_missing = values.iter().any(|value| !value.is_finite());
    if !has_missing {
        return Ok(SVObservationStorage::Borrowed(SVColumnView::Streaming(
            streaming,
        )));
    }
    let kept = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    Ok(SVObservationStorage::Dense(kept))
}

fn sv_sparse_storage(
    sparse: SVSparseView<'_>,
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'_>, SVError> {
    if let Some(position) = sparse.values().iter().position(|value| !value.is_finite()) {
        policy.report("histogram_values", position)?;
    }
    let has_missing = sparse.values().iter().any(|value| !value.is_finite());
    if !has_missing {
        return Ok(SVObservationStorage::Borrowed(SVColumnView::Sparse(sparse)));
    }
    let mut indices = Vec::new();
    let mut values = Vec::new();
    let mut dropped = 0usize;
    for (index, value) in sparse.stored_entries() {
        if value.is_finite() {
            indices.push(index - dropped);
            values.push(value);
        } else {
            dropped += 1;
        }
    }
    // Dropping an observation shortens the logical column by one, so the
    // removed stored entry never turns into an implicit zero. The implicit
    // zeros that remain keep their count.
    Ok(SVObservationStorage::Sparse {
        indices,
        values,
        length: sparse.length().saturating_sub(dropped),
    })
}

fn sv_group_sparse<'a>(
    sparse: SVSparseView<'a>,
    indices: &[usize],
    policy: SVHistogramMissingValuePolicy,
) -> Result<SVObservationStorage<'a>, SVError> {
    let mut stored_indices = Vec::new();
    let mut stored_values = Vec::new();
    let mut dropped = 0usize;
    for (position, original) in indices.iter().copied().enumerate() {
        let stored = match sparse.indices().binary_search(&original) {
            Ok(stored_position) => sparse.values().get(stored_position).copied(),
            Err(_) => None,
        };
        let Some(value) = stored else {
            continue;
        };
        if !value.is_finite() {
            policy.report("histogram_values", original)?;
            if policy.omits() {
                dropped += 1;
                continue;
            }
        }
        stored_indices.push(position - dropped);
        stored_values.push(value);
    }
    // A dropped observation leaves the logical column one position shorter;
    // the group's implicit zeros keep their count instead of absorbing it.
    Ok(SVObservationStorage::Sparse {
        indices: stored_indices,
        values: stored_values,
        length: indices.len().saturating_sub(dropped),
    })
}
