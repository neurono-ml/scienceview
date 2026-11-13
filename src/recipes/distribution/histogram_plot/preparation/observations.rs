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
    /// The weights aligned one-to-one with `storage`, when weighting is on.
    pub(super) weight: Option<SVObservationStorage<'a>>,
}

impl SVObservationGroup<'_> {
    /// Returns the weight column aligned with this group's observations.
    pub(super) fn weight_view(&self) -> Result<Option<SVColumnView<'_>>, SVError> {
        self.weight
            .as_ref()
            .map(SVObservationStorage::view)
            .transpose()
    }
}

/// Builds one observation group per series.
///
/// The weight column is filtered exactly like the value column, so a dropped
/// observation drops its weight too and the two stay aligned position by
/// position.
pub(super) fn sv_observation_groups<'a>(
    value: SVColumnView<'a>,
    weight: Option<SVColumnView<'a>>,
    series: Option<SVColumnView<'a>>,
    policy: SVHistogramMissingValuePolicy,
    labels: &[String],
) -> Result<Vec<SVObservationGroup<'a>>, SVError> {
    let palette = sv_default_scale_palette();
    match series {
        None => {
            let (storage, dropped) = sv_whole_storage(value, policy)?;
            Ok(vec![SVObservationGroup {
                label: String::new(),
                colour: palette.colour_at_index(0),
                weight: sv_whole_weights(weight, &dropped, storage.length()),
                storage,
            }])
        }
        Some(series) => {
            let groups = sv_series_membership(series, value.length())?;
            groups
                .into_iter()
                .enumerate()
                .map(|(index, membership)| {
                    let label = labels.get(index).cloned().unwrap_or(membership.label);
                    let (storage, dropped) = sv_group_storage(value, &membership.indices, policy)?;
                    Ok(SVObservationGroup {
                        label,
                        colour: palette.colour_at_index(index),
                        weight: sv_group_weights(
                            weight,
                            &membership.indices,
                            &dropped,
                            storage.length(),
                        ),
                        storage,
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
) -> Result<(SVObservationStorage<'_>, Vec<usize>), SVError> {
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
) -> Result<(SVObservationStorage<'a>, Vec<usize>), SVError> {
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
) -> Result<(SVObservationStorage<'a>, Vec<usize>), SVError> {
    let mut kept = Vec::with_capacity(values.len());
    let mut dropped = Vec::new();
    for (position, value) in values.into_iter().enumerate() {
        if !value.is_finite() {
            let original = original_indices.get(position).copied().unwrap_or(position);
            policy.report("histogram_values", original)?;
            if policy.omits() {
                dropped.push(position);
                continue;
            }
        }
        kept.push(value);
    }
    Ok((SVObservationStorage::Dense(kept), dropped))
}

fn sv_dense_storage(
    values: &[f64],
    policy: SVHistogramMissingValuePolicy,
) -> Result<(SVObservationStorage<'_>, Vec<usize>), SVError> {
    let dropped: Vec<usize> = values
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_finite())
        .map(|(position, _)| position)
        .collect();
    if let Some(position) = dropped.first() {
        policy.report("histogram_values", *position)?;
    }
    if dropped.is_empty() {
        return Ok((
            SVObservationStorage::Borrowed(SVColumnView::Dense(values)),
            dropped,
        ));
    }
    let kept = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    Ok((SVObservationStorage::Dense(kept), dropped))
}

fn sv_streaming_storage(
    streaming: SVStreamingView<'_>,
    policy: SVHistogramMissingValuePolicy,
) -> Result<(SVObservationStorage<'_>, Vec<usize>), SVError> {
    let values = streaming.values();
    let dropped: Vec<usize> = values
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_finite())
        .map(|(position, _)| position)
        .collect();
    if let Some(position) = dropped.first() {
        policy.report("histogram_values", *position)?;
    }
    if dropped.is_empty() {
        return Ok((
            SVObservationStorage::Borrowed(SVColumnView::Streaming(streaming)),
            dropped,
        ));
    }
    let kept = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    Ok((SVObservationStorage::Dense(kept), dropped))
}

fn sv_sparse_storage(
    sparse: SVSparseView<'_>,
    policy: SVHistogramMissingValuePolicy,
) -> Result<(SVObservationStorage<'_>, Vec<usize>), SVError> {
    let dropped_positions: Vec<usize> = sparse
        .stored_entries()
        .filter(|(_, value)| !value.is_finite())
        .map(|(index, _)| index)
        .collect();
    if let Some(position) = dropped_positions.first() {
        policy.report("histogram_values", *position)?;
    }
    if dropped_positions.is_empty() {
        return Ok((
            SVObservationStorage::Borrowed(SVColumnView::Sparse(sparse)),
            dropped_positions,
        ));
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
    Ok((
        SVObservationStorage::Sparse {
            indices,
            values,
            length: sparse.length().saturating_sub(dropped),
        },
        dropped_positions,
    ))
}

fn sv_group_sparse<'a>(
    sparse: SVSparseView<'a>,
    indices: &[usize],
    policy: SVHistogramMissingValuePolicy,
) -> Result<(SVObservationStorage<'a>, Vec<usize>), SVError> {
    let mut stored_indices = Vec::new();
    let mut stored_values = Vec::new();
    let mut dropped = 0usize;
    let mut dropped_positions = Vec::new();
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
                dropped_positions.push(position);
                continue;
            }
        }
        stored_indices.push(position - dropped);
        stored_values.push(value);
    }
    // A dropped observation leaves the logical column one position shorter;
    // the group's implicit zeros keep their count instead of absorbing it.
    Ok((
        SVObservationStorage::Sparse {
            indices: stored_indices,
            values: stored_values,
            length: indices.len().saturating_sub(dropped),
        },
        dropped_positions,
    ))
}

/// Aligns the weight column of the whole value column.
///
/// When no observation is dropped the weights are borrowed as they arrived, so
/// a sparse weight column reaches the kernel without being materialised.
fn sv_whole_weights<'a>(
    weight: Option<SVColumnView<'a>>,
    dropped: &[usize],
    new_length: usize,
) -> Option<SVObservationStorage<'a>> {
    weight.map(|weight| {
        if dropped.is_empty() {
            SVObservationStorage::Borrowed(weight)
        } else {
            sv_compact_weights(SVObservationStorage::Borrowed(weight), dropped, new_length)
        }
    })
}

/// Aligns the weight column with one series group.
fn sv_group_weights<'a>(
    weight: Option<SVColumnView<'a>>,
    indices: &[usize],
    dropped: &[usize],
    new_length: usize,
) -> Option<SVObservationStorage<'a>> {
    weight.map(|weight| {
        let selected = sv_selected_weights(weight, indices);
        sv_compact_weights(selected, dropped, new_length)
    })
}

/// Reads the weights at the given positions, preserving sparse storage.
fn sv_selected_weights<'a>(
    weight: SVColumnView<'a>,
    positions: &[usize],
) -> SVObservationStorage<'a> {
    match weight {
        SVColumnView::Dense(values) => SVObservationStorage::Dense(
            positions
                .iter()
                .map(|position| values.get(*position).copied().unwrap_or(0.0))
                .collect(),
        ),
        SVColumnView::Streaming(streaming) => {
            let values = streaming.values();
            SVObservationStorage::Dense(
                positions
                    .iter()
                    .map(|position| values.get(*position).copied().unwrap_or(0.0))
                    .collect(),
            )
        }
        SVColumnView::Sparse(sparse) => {
            let mut indices = Vec::new();
            let mut values = Vec::new();
            for (stored_index, stored_value) in sparse.stored_entries() {
                if let Ok(position) = positions.binary_search(&stored_index) {
                    indices.push(position);
                    values.push(stored_value);
                }
            }
            SVObservationStorage::Sparse {
                indices,
                values,
                length: positions.len(),
            }
        }
    }
}

/// Removes the dropped logical positions from a weight column.
fn sv_compact_weights<'a>(
    weights: SVObservationStorage<'a>,
    dropped: &[usize],
    new_length: usize,
) -> SVObservationStorage<'a> {
    if dropped.is_empty() {
        return weights;
    }
    match weights {
        SVObservationStorage::Borrowed(SVColumnView::Dense(values)) => {
            SVObservationStorage::Dense(sv_skip_dropped(values.iter().copied(), dropped))
        }
        SVObservationStorage::Borrowed(SVColumnView::Streaming(streaming)) => {
            SVObservationStorage::Dense(sv_skip_dropped(
                streaming.values().iter().copied(),
                dropped,
            ))
        }
        SVObservationStorage::Borrowed(SVColumnView::Sparse(sparse)) => {
            sv_compact_sparse_weights(sparse.indices(), sparse.values(), dropped, new_length)
        }
        SVObservationStorage::Dense(values) => {
            SVObservationStorage::Dense(sv_skip_dropped(values.into_iter(), dropped))
        }
        SVObservationStorage::Sparse {
            indices, values, ..
        } => sv_compact_sparse_weights(&indices, &values, dropped, new_length),
    }
}

fn sv_skip_dropped(values: impl Iterator<Item = f64>, dropped: &[usize]) -> Vec<f64> {
    let mut next_dropped = dropped.iter().copied().peekable();
    let mut kept = Vec::new();
    for (position, value) in values.enumerate() {
        while next_dropped.peek().is_some_and(|next| *next < position) {
            next_dropped.next();
        }
        if next_dropped.peek().is_some_and(|next| *next == position) {
            continue;
        }
        kept.push(value);
    }
    kept
}

fn sv_compact_sparse_weights<'a>(
    indices: &[usize],
    values: &[f64],
    dropped: &[usize],
    new_length: usize,
) -> SVObservationStorage<'a> {
    let mut mapped_indices = Vec::new();
    let mut mapped_values = Vec::new();
    for (index, value) in indices.iter().copied().zip(values.iter().copied()) {
        if dropped.binary_search(&index).is_ok() {
            continue;
        }
        let removed = dropped.partition_point(|position| *position < index);
        mapped_indices.push(index - removed);
        mapped_values.push(value);
    }
    SVObservationStorage::Sparse {
        indices: mapped_indices,
        values: mapped_values,
        length: new_length,
    }
}
