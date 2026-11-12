//! Normalising: a group rescaled to a fraction or a percentage.
//!
//! The group total is computed from the *contribution* of each point, that is
//! `value - baseline`, so a normalized stack keeps its segment heights. Positive
//! contributions are divided by the positive total and negative contributions by
//! the negative total, which makes a diverging stack reach `+1` and `-1`
//! independently instead of sharing one scale.
//!
//! A sparse column contributes only through the entries it stores: an implicit
//! zero has a zero contribution, is not part of either total and stays at the
//! zero it already means, so the pass never visits a logical position the
//! column does not hold.

use crate::sv_error::SVError;
use crate::sv_move::column::SVMoveColumn;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMoveRequest, SVNormalisationTarget, SVNormalizeOptions};
use std::collections::HashMap;

/// Applies a normalize to the working state.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when the group codes and the values
/// disagree; the sequence validates that first.
pub(crate) fn sv_apply_normalize(
    state: &mut SVMoveState<'_>,
    request: &SVMoveRequest<'_>,
    options: &SVNormalizeOptions,
) -> Result<(), SVError> {
    let group_keys = request.group_keys();
    if group_keys.len() != state.length {
        return Err(SVError::MismatchedLengths {
            name: "group_keys".to_string(),
            expected: state.length,
            actual: group_keys.len(),
        });
    }

    let source = state.source;
    let values = state.values.materialise();
    let baselines = state
        .baselines
        .get_or_insert_with(|| SVMoveColumn::sv_zeroed_like(source));

    let mut positive_totals: HashMap<u32, f64> = HashMap::new();
    let mut negative_totals: HashMap<u32, f64> = HashMap::new();

    for index in values.sv_stored_positions() {
        let group_key = group_keys[index];
        let contribution =
            values.value_at(index).unwrap_or(0.0) - baselines.value_at(index).unwrap_or(0.0);
        if contribution > 0.0 {
            *positive_totals.entry(group_key).or_insert(0.0) += contribution;
        } else if contribution < 0.0 {
            *negative_totals.entry(group_key).or_insert(0.0) += -contribution;
        }
    }

    let target_scale = match options.target_value() {
        SVNormalisationTarget::Fraction => 1.0,
        SVNormalisationTarget::Percent => 100.0,
    };

    for index in values.sv_stored_positions() {
        let group_key = group_keys[index];
        let baseline = baselines.value_at(index).unwrap_or(0.0);
        let value = values.value_at(index).unwrap_or(0.0);
        let contribution = value - baseline;
        let total = if contribution >= 0.0 {
            positive_totals.get(&group_key).copied().unwrap_or(0.0)
        } else {
            negative_totals.get(&group_key).copied().unwrap_or(0.0)
        };

        if total > 0.0 {
            let factor = target_scale / total;
            baselines.sv_set(index, baseline * factor);
            values.sv_set(index, value * factor);
        } else {
            baselines.sv_set(index, 0.0);
            values.sv_set(index, 0.0);
        }
    }

    state.values_changed = true;
    state.baselines_produced = true;
    Ok(())
}
