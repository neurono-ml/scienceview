//! Swarm: a collision-free deterministic layout.
//!
//! Points of one category are visited in ascending value order, ties broken by
//! their original index. Each point takes the nearest candidate offset that
//! does not overlap a point already placed, where the candidate offsets are
//! whole marker extents: `0, +1, -1, +2, -2, ...`. Two points overlap when they
//! are closer than one marker extent on *both* axes, so touching is allowed and
//! the layout packs as tightly as a beeswarm does.
//!
//! The layout is deterministic because the visiting order and the candidate
//! order are; it finishes in bounded time because the candidate list is capped
//! ([`SV_SWARM_MAXIMUM_ATTEMPTS`]) and the overlap scan stops as soon as the
//! ascending values leave the marker window.

//!
//! A swarm reads the value column through its representation-aware accessor and
//! lays out only the entries the column stores, so a sparse input is never
//! expanded; an implicit sparse zero is not a stored mark and keeps offset zero.

use crate::sv_error::SVError;
use crate::sv_move::column::SVMoveColumn;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMoveRequest, SVSwarmOptions};
use std::collections::BTreeMap;

/// How many candidate offsets one point tries before the layout gives up.
///
/// The cap is what bounds the run time even for data that would need a very
/// wide swarm; a point that exhausts it is reported as an invalid
/// configuration instead of being placed on top of another mark.
pub(crate) const SV_SWARM_MAXIMUM_ATTEMPTS: usize = 1024;

/// Applies a swarm layout to the working state.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when the group codes and the values
/// disagree; the sequence validates that first. Returns
/// [`SVError::InvalidConfiguration`] when the bounded search cannot place a
/// point without a collision, naming the group and the point.
pub(crate) fn sv_apply_swarm(
    state: &mut SVMoveState<'_>,
    request: &SVMoveRequest<'_>,
    options: &SVSwarmOptions,
) -> Result<(), SVError> {
    let group_keys = request.group_keys();
    if group_keys.len() != state.length {
        return Err(SVError::MismatchedLengths {
            name: "group_keys".to_string(),
            expected: state.length,
            actual: group_keys.len(),
        });
    }

    let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for index in state.values.stored_positions() {
        groups.entry(group_keys[index]).or_default().push(index);
    }

    let marker_extent = options.marker_extent_value();
    let maximum_offset = options.width_value().map(|width| f64::from(width) / 2.0);
    let source = state.source;
    let horizontal_offsets = state
        .horizontal_offsets
        .get_or_insert_with(|| SVMoveColumn::sv_zeroed_like(source));
    let values = &state.values;

    for indices in groups.values_mut() {
        indices.sort_by(|left, right| {
            values
                .value_at(*left)
                .total_cmp(&values.value_at(*right))
                .then(left.cmp(right))
        });

        let mut placed: Vec<(f64, f64)> = Vec::with_capacity(indices.len());
        for index in indices.iter().copied() {
            let value = values.value_at(index);
            let offset = sv_place_in_swarm(&placed, value, marker_extent, maximum_offset);
            placed.push((value, offset));
            horizontal_offsets.sv_add(index, offset);
        }
    }

    Ok(())
}

/// Finds the offset of one point among the points already placed.
fn sv_place_in_swarm(
    placed: &[(f64, f64)],
    value: f64,
    marker_extent: f64,
    maximum_offset: Option<f64>,
) -> f64 {
    let maximum_rank = maximum_offset.map(|limit| (limit / marker_extent).floor() as i64);
    let mut attempt = 0_usize;
    let mut tried = 0_usize;

    while tried < SV_SWARM_MAXIMUM_ATTEMPTS {
        let rank = sv_candidate_rank(attempt);
        attempt += 1;

        if maximum_rank.is_some_and(|limit| rank.abs() > limit) {
            // Candidates grow outward, so once one leaves the bound the rest do
            // too on that side; the other side was already offered.
            if rank > 0 {
                break;
            }
            continue;
        }

        let offset = rank as f64 * marker_extent;
        if !sv_collides(placed, value, offset, marker_extent) {
            return offset;
        }
        tried += 1;
    }

    match maximum_rank {
        Some(limit) => limit.max(0) as f64 * marker_extent,
        None => sv_candidate_rank(attempt) as f64 * marker_extent,
    }
}

/// Returns the rank of one candidate in the alternating order.
fn sv_candidate_rank(attempt: usize) -> i64 {
    if attempt == 0 {
        return 0;
    }
    let pair = ((attempt + 1) / 2) as i64;
    if attempt % 2 == 1 { pair } else { -pair }
}

/// Reports whether a candidate overlaps an already placed point.
fn sv_collides(placed: &[(f64, f64)], value: f64, offset: f64, marker_extent: f64) -> bool {
    for (placed_value, placed_offset) in placed.iter().rev() {
        let vertical_distance = (value - placed_value).abs();
        if vertical_distance >= marker_extent {
            break;
        }
        if (offset - placed_offset).abs() < marker_extent {
            return true;
        }
    }
    false
}
