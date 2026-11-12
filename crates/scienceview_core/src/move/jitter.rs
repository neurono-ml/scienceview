//! Jitter: a deterministic displacement within a category.
//!
//! The displacement is a function of the mark's key — its group code and its
//! index — never of a frame index, so a point keeps its offset while the values
//! around it change (design D4). The amount is a fraction of the category slot,
//! which is [`SV_JITTER_SLOT_WIDTH`] data units wide, matching the default dodge
//! width so a jittered and a dodged mark share the same slot.
//!
//! A jitter is purely positional: it shares the borrowed value column and never
//! copies it.

use crate::sv_error::SVError;
use crate::sv_move::column::SVMoveColumn;
use crate::sv_move::random::{
    sv_mix_mark_key, sv_signed_unit_interval, sv_split_mix_64, sv_unit_interval,
};
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVJitterMode, SVJitterOptions, SVMoveRequest};

/// The category slot a jitter amount is a fraction of, in data units.
pub(crate) const SV_JITTER_SLOT_WIDTH: f64 = 0.8;

/// How many uniforms the bounded bell shape sums.
///
/// Summing twelve uniforms and subtracting six gives a bell with unit variance
/// and a bounded support of `-6.0..6.0`; the sum is scaled by one sixth so the
/// result spans `-1.0..1.0`. Only addition is involved, so the shape is
/// identical on every platform.
const SV_GAUSSIAN_DRAWS: usize = 12;

/// Applies a jitter to the working state.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when the group codes and the values
/// disagree; the sequence validates that first.
pub(crate) fn sv_apply_jitter(
    state: &mut SVMoveState<'_>,
    request: &SVMoveRequest<'_>,
    options: &SVJitterOptions,
) -> Result<(), SVError> {
    let group_keys = request.group_keys();
    if group_keys.len() != state.length {
        return Err(SVError::MismatchedLengths {
            name: "group_keys".to_string(),
            expected: state.length,
            actual: group_keys.len(),
        });
    }

    let band = f64::from(options.amount_value()) * SV_JITTER_SLOT_WIDTH;
    let seed = options.seed_value();
    let mode = options.mode_value();
    let source = state.source;
    let horizontal_offsets = state
        .horizontal_offsets
        .get_or_insert_with(|| SVMoveColumn::sv_zeroed_like(source));

    for (index, group_key) in group_keys.iter().copied().enumerate() {
        let mut generator_state = sv_mix_mark_key(seed, group_key, index);
        let unit = match mode {
            SVJitterMode::Uniform => sv_signed_unit_interval(sv_split_mix_64(&mut generator_state)),
            SVJitterMode::Gaussian => sv_gaussian_unit(&mut generator_state),
        };
        horizontal_offsets.sv_add(index, unit * band);
    }

    Ok(())
}

/// Returns one bounded bell-shaped draw in `-1.0..1.0`.
fn sv_gaussian_unit(generator_state: &mut u64) -> f64 {
    let mut total = 0.0;
    for _ in 0..SV_GAUSSIAN_DRAWS {
        total += sv_unit_interval(sv_split_mix_64(generator_state));
    }
    (total - 6.0) / 6.0
}
