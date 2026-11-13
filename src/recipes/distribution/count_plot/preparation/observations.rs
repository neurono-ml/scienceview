//! Turning the category column into one group per series.
//!
//! Each group keeps the representation it was given. A dense or streaming
//! column is filtered into an owned buffer only when the missing-value policy
//! has to drop something; a sparse column keeps sparse storage, so an implicit
//! zero stays an implicit zero of its group and is still counted as category
//! zero by the kernel. Nothing here counts observations.

use std::collections::HashMap;

use super::super::SVCountMissingValuePolicy;
use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_scale::{SVColour, sv_default_scale_palette};

/// The category observations of one series, in the shape they arrived in.
pub(super) enum SVCategoryStorage<'a> {
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

impl SVCategoryStorage<'_> {
    /// Returns the view the counting kernel consumes.
    pub(super) fn view(&self) -> Result<SVColumnView<'_>, SVError> {
        match self {
            SVCategoryStorage::Borrowed(view) => Ok(*view),
            SVCategoryStorage::Dense(values) => Ok(SVColumnView::Dense(values)),
            SVCategoryStorage::Sparse {
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
            SVCategoryStorage::Borrowed(view) => view.length(),
            SVCategoryStorage::Dense(values) => values.len(),
            SVCategoryStorage::Sparse { length, .. } => *length,
        }
    }
}

/// One series of category observations, with its resolved label and colour.
pub(super) struct SVCategoryGroup<'a> {
    pub(super) label: String,
    pub(super) colour: SVColour,
    pub(super) storage: SVCategoryStorage<'a>,
}

/// Builds one category group per series.
pub(super) fn sv_category_groups<'a>(
    category: SVColumnView<'a>,
    series: Option<SVColumnView<'a>>,
    policy: SVCountMissingValuePolicy,
    labels: &[String],
    requested_colour: Option<SVColour>,
) -> Result<Vec<SVCategoryGroup<'a>>, SVError> {
    let palette = sv_default_scale_palette();
    match series {
        None => Ok(vec![SVCategoryGroup {
            label: String::new(),
            colour: requested_colour.unwrap_or_else(|| palette.colour_at_index(0)),
            storage: sv_whole_storage(category, policy)?,
        }]),
        Some(series) => {
            let memberships = sv_series_memberships(series, category.length(), policy)?;
            memberships
                .into_iter()
                .enumerate()
                .map(|(index, membership)| {
                    let label = labels.get(index).cloned().unwrap_or(membership.label);
                    Ok(SVCategoryGroup {
                        label,
                        colour: requested_colour.unwrap_or_else(|| palette.colour_at_index(index)),
                        storage: sv_group_storage(category, &membership.positions, policy)?,
                    })
                })
                .collect()
        }
    }
}

struct SVSeriesMembership {
    label: String,
    positions: Vec<usize>,
}

fn sv_series_memberships(
    series: SVColumnView<'_>,
    number_of_points: usize,
    policy: SVCountMissingValuePolicy,
) -> Result<Vec<SVSeriesMembership>, SVError> {
    let mut groups: Vec<(f64, SVSeriesMembership)> = Vec::new();
    for index in 0..number_of_points {
        let Some(value) = series.value_at(index) else {
            continue;
        };
        if !value.is_finite() {
            policy.report("count_series", index)?;
            continue;
        }
        let position = match groups
            .iter()
            .position(|(key, _)| sv_category_key(*key) == sv_category_key(value))
        {
            Some(position) => position,
            None => {
                groups.push((
                    value,
                    SVSeriesMembership {
                        label: sv_series_label(value),
                        positions: Vec::new(),
                    },
                ));
                groups.len() - 1
            }
        };
        if let Some((_, group)) = groups.get_mut(position) {
            group.positions.push(index);
        }
    }
    Ok(groups.into_iter().map(|(_, group)| group).collect())
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
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'_>, SVError> {
    match view {
        SVColumnView::Dense(values) => sv_dense_storage(values, policy),
        SVColumnView::Streaming(streaming) => sv_streaming_storage(streaming, policy),
        SVColumnView::Sparse(sparse) => sv_sparse_storage(sparse, policy),
    }
}

fn sv_group_storage<'a>(
    view: SVColumnView<'a>,
    positions: &[usize],
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'a>, SVError> {
    match view {
        SVColumnView::Dense(values) => {
            let selected: Vec<f64> = positions
                .iter()
                .filter_map(|index| values.get(*index).copied())
                .collect();
            sv_owned_dense(selected, positions, policy)
        }
        SVColumnView::Streaming(streaming) => {
            let values = streaming.values();
            let selected: Vec<f64> = positions
                .iter()
                .filter_map(|index| values.get(*index).copied())
                .collect();
            sv_owned_dense(selected, positions, policy)
        }
        SVColumnView::Sparse(sparse) => sv_group_sparse(sparse, positions, policy),
    }
}

fn sv_owned_dense<'a>(
    values: Vec<f64>,
    original_positions: &[usize],
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'a>, SVError> {
    let mut kept = Vec::with_capacity(values.len());
    for (position, value) in values.into_iter().enumerate() {
        if !value.is_finite() {
            let original = original_positions
                .get(position)
                .copied()
                .unwrap_or(position);
            policy.report("count_category", original)?;
            if policy.omits() {
                continue;
            }
        }
        kept.push(value);
    }
    Ok(SVCategoryStorage::Dense(kept))
}

fn sv_dense_storage(
    values: &[f64],
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'_>, SVError> {
    if let Some(position) = values.iter().position(|value| !value.is_finite()) {
        policy.report("count_category", position)?;
    }
    let has_missing = values.iter().any(|value| !value.is_finite());
    if !has_missing {
        return Ok(SVCategoryStorage::Borrowed(SVColumnView::Dense(values)));
    }
    let kept = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    Ok(SVCategoryStorage::Dense(kept))
}

fn sv_streaming_storage(
    streaming: SVStreamingView<'_>,
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'_>, SVError> {
    let values = streaming.values();
    if let Some(position) = values.iter().position(|value| !value.is_finite()) {
        policy.report("count_category", position)?;
    }
    let has_missing = values.iter().any(|value| !value.is_finite());
    if !has_missing {
        return Ok(SVCategoryStorage::Borrowed(SVColumnView::Streaming(
            streaming,
        )));
    }
    let kept = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    Ok(SVCategoryStorage::Dense(kept))
}

fn sv_sparse_storage(
    sparse: SVSparseView<'_>,
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'_>, SVError> {
    if let Some(position) = sparse.values().iter().position(|value| !value.is_finite()) {
        policy.report("count_category", position)?;
    }
    let has_missing = sparse.values().iter().any(|value| !value.is_finite());
    if !has_missing {
        return Ok(SVCategoryStorage::Borrowed(SVColumnView::Sparse(sparse)));
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
    Ok(SVCategoryStorage::Sparse {
        indices,
        values,
        length: sparse.length().saturating_sub(dropped),
    })
}

fn sv_group_sparse<'a>(
    sparse: SVSparseView<'a>,
    positions: &[usize],
    policy: SVCountMissingValuePolicy,
) -> Result<SVCategoryStorage<'a>, SVError> {
    let mut stored_by_position: HashMap<usize, f64> = HashMap::with_capacity(sparse.values().len());
    for (index, value) in sparse.stored_entries() {
        stored_by_position.insert(index, value);
    }

    let mut stored_entries: Vec<(usize, f64)> = Vec::new();
    for (local, original) in positions.iter().copied().enumerate() {
        if let Some(value) = stored_by_position.get(&original).copied() {
            stored_entries.push((local, value));
        }
    }

    let mut indices = Vec::new();
    let mut values = Vec::new();
    let mut dropped = 0usize;
    for (local, value) in stored_entries {
        if !value.is_finite() {
            let original = positions.get(local).copied().unwrap_or(local);
            policy.report("count_category", original)?;
            if policy.omits() {
                dropped += 1;
                continue;
            }
        }
        indices.push(local - dropped);
        values.push(value);
    }
    // A dropped observation leaves the logical group one position shorter;
    // the group's implicit zeros keep their count instead of absorbing it.
    Ok(SVCategoryStorage::Sparse {
        indices,
        values,
        length: positions.len().saturating_sub(dropped),
    })
}

/// Maps a category value onto its exact storage key.
///
/// `-0.0` and `0.0` are the same category, so both map onto zero. This is the
/// same key the counting kernel uses; the recipe only needs it to align the
/// categories the kernel returns.
pub(super) fn sv_category_key(value: f64) -> u64 {
    if value == 0.0 { 0 } else { value.to_bits() }
}
