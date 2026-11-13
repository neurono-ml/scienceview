//! The swarm displacement: the recipe's one call into the move kernel.
//!
//! Nothing here lays points out. [`SVMove::Swarm`] owns the deterministic,
//! collision-free, width-bounded layout and its typed errors; the recipe only
//! converts the display-space marker geometry into the data-space extents the
//! kernel asks for and maps the returned offsets back onto the categorical axis.

use super::channels::SVSwarmObservation;
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

/// Lays every observation out and returns its category-axis position.
///
/// The positions are in category-slot units: the slot index plus the
/// displacement the kernel produced, expressed in the same units. A point the
/// kernel cannot place without a collision leaves its typed configuration error
/// untouched.
pub(super) fn sv_layout_categories(
    observations: &[SVSwarmObservation],
    marker_extent_value: f64,
    marker_extent_category: f64,
    width_value: f64,
) -> Result<Vec<f64>, SVError> {
    if observations.is_empty() {
        return Ok(Vec::new());
    }

    let values: Vec<f64> = observations
        .iter()
        .map(|observation| observation.value)
        .collect();
    let group_keys: Vec<u32> = observations
        .iter()
        .map(|observation| {
            u32::try_from(observation.category_index).map_err(|_| SVError::InvalidConfiguration {
                message: "a swarm supports at most 4294967296 categories".to_string(),
            })
        })
        .collect::<Result<_, _>>()?;

    let swarm: SVMove = SVMove::swarm()
        .marker_extent(marker_extent_value)
        .width(width_value as f32)
        .into();
    let sequence = SVMoveSequence::single(swarm)?;
    let moved = sequence.apply(&SVMoveRequest::new(
        SVColumnView::Dense(&values),
        &group_keys,
    ))?;

    observations
        .iter()
        .enumerate()
        .map(|(index, observation)| {
            let offset =
                moved
                    .horizontal_offset(index)
                    .ok_or_else(|| SVError::InvalidConfiguration {
                        message: format!(
                            "the swarm move produced no displacement for observation {index}"
                        ),
                    })?;
            let offset_in_extents = offset / marker_extent_value;
            Ok(observation.category_index as f64 + offset_in_extents * marker_extent_category)
        })
        .collect()
}
