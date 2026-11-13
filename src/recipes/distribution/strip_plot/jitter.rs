//! Category-axis jitter through `SVMove::Jitter`.
//!
//! The move stores a displacement on its horizontal axis. A strip plot's
//! categorical axis is the horizontal axis of the move whatever the figure
//! orientation is: the recipe maps the resulting offset onto the axis the
//! categories occupy. The offset comes from the move kernel, which keys it by
//! the observation's row and category — never by a frame index — so a point
//! keeps its displacement while its value changes.

use super::configuration::SVStripConfiguration;
use super::data_contract::{SVStripData, SVStripObservation};
use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

/// Returns the displaced category position of every drawn observation.
///
/// The synthetic slot column keeps the representation of the value column: a
/// sparse value column produces a sparse slot column over the same stored
/// indices, so a huge sparse input is never expanded. The slot values are the
/// category indices, the keys are the same indices, and the offsets are read
/// back only at the stored observations.
pub(super) fn sv_jittered_categories(
    value: SVColumnView<'_>,
    data: &SVStripData,
    configuration: &SVStripConfiguration<'_>,
) -> Result<Vec<f64>, SVError> {
    let number_of_positions = value.length();
    let mut group_keys = vec![0_u32; number_of_positions];
    for observation in &data.observations {
        if let Some(key) = group_keys.get_mut(observation.row) {
            *key = sv_key(observation);
        }
    }

    let sequence = SVMoveSequence::single(sv_jitter_move(configuration))?;
    let offsets = match value.sparse_view() {
        Some(sparse) => {
            let stored = sv_sparse_slots(sparse, &data.observations);
            let slot_view = SVSparseView::new(sparse.indices(), &stored, number_of_positions)?;
            let request = SVMoveRequest::new(SVColumnView::Sparse(slot_view), &group_keys);
            sequence.apply(&request)?
        }
        None => {
            let slots = sv_dense_slots(number_of_positions, &data.observations);
            let request = SVMoveRequest::new(SVColumnView::Dense(&slots), &group_keys);
            sequence.apply(&request)?
        }
    };

    Ok(data
        .observations
        .iter()
        .map(|observation| {
            let offset = offsets.horizontal_offset(observation.row).unwrap_or(0.0);
            observation.category_index as f64 + offset
        })
        .collect())
}

fn sv_jitter_move(configuration: &SVStripConfiguration<'_>) -> SVMove {
    let mut options = SVMove::jitter()
        .amount(configuration.jitter)
        .mode(configuration.jitter_mode);
    if let Some(seed) = configuration.seed {
        options = options.seed(seed);
    }
    SVMove::from(options)
}

fn sv_key(observation: &SVStripObservation) -> u32 {
    u32::try_from(observation.category_index).unwrap_or(u32::MAX)
}

/// Builds the dense category-slot column of a dense or streaming value column.
fn sv_dense_slots(number_of_positions: usize, observations: &[SVStripObservation]) -> Vec<f64> {
    let mut slots = vec![0.0; number_of_positions];
    for observation in observations {
        if let Some(slot) = slots.get_mut(observation.row) {
            *slot = observation.category_index as f64;
        }
    }
    slots
}

/// Builds the stored category-slot values over the value column's stored rows.
///
/// The slot view borrows the stored indices from the value column, so the
/// kernel sees exactly the entries the data holds and no implicit zero becomes
/// a mark.
fn sv_sparse_slots(sparse: &SVSparseView<'_>, observations: &[SVStripObservation]) -> Vec<f64> {
    let mut stored = Vec::with_capacity(sparse.indices().len());
    for row in sparse.indices() {
        let category_index = observations
            .iter()
            .find(|observation| observation.row == *row)
            .map_or(0.0, |observation| observation.category_index as f64);
        stored.push(category_index);
    }
    stored
}
