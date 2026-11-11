//! Stacking: a shared baseline for a group.
//!
//! The stack is *diverging*: positive values accumulate upward from zero and
//! negative values accumulate downward from zero, independently. A zero value
//! keeps the zero baseline and never advances either accumulator, which is also
//! what an implicit sparse zero means.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMoveRequest, SVStackMode, SVStackOptions};
use std::collections::HashMap;

/// Applies a stack to the working state.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when the group codes and the values
/// disagree; the sequence validates that first, so this is a defensive check.
pub(crate) fn sv_apply_stack(
    state: &mut SVMoveState,
    request: &SVMoveRequest<'_>,
    options: &SVStackOptions,
) -> Result<(), SVError> {
    let group_keys = request.group_keys();
    if group_keys.len() != state.values.len() {
        return Err(SVError::MismatchedLengths {
            name: "group_keys".to_string(),
            expected: state.values.len(),
            actual: group_keys.len(),
        });
    }

    let mut positive_totals: HashMap<u32, f64> = HashMap::new();
    let mut negative_totals: HashMap<u32, f64> = HashMap::new();

    for (index, group_key) in group_keys.iter().copied().enumerate() {
        let value = state.values[index];

        if value == 0.0 {
            state.baselines[index] = 0.0;
            state.values[index] = 0.0;
            continue;
        }

        let accumulator = if value > 0.0 {
            &mut positive_totals
        } else {
            &mut negative_totals
        };
        let baseline = accumulator.entry(group_key).or_insert(0.0);
        state.baselines[index] = *baseline;
        *baseline += value;
        state.values[index] = state.baselines[index] + value;
    }

    if options.mode_value() == SVStackMode::Center {
        sv_center_groups(state, group_keys);
    }

    state.values_changed = true;
    state.baselines_produced = true;
    Ok(())
}

/// Shifts every group so that its full extent is centred on zero.
///
/// The offset is the midpoint of the lowest baseline and the highest top, so a
/// diverging stack is centred as a whole rather than one side at a time.
fn sv_center_groups(state: &mut SVMoveState, group_keys: &[u32]) {
    let mut extents: HashMap<u32, (f64, f64)> = HashMap::new();
    for (index, group_key) in group_keys.iter().copied().enumerate() {
        let baseline = state.baselines[index];
        let top = state.values[index];
        let extent = extents.entry(group_key).or_insert((baseline, top));
        extent.0 = extent.0.min(baseline);
        extent.1 = extent.1.max(top);
    }

    for (index, group_key) in group_keys.iter().copied().enumerate() {
        if let Some((lowest_baseline, highest_top)) = extents.get(&group_key) {
            let offset = -((highest_top + lowest_baseline) / 2.0);
            state.baselines[index] += offset;
            state.values[index] += offset;
        }
    }
}
