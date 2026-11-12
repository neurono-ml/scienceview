//! Stacking: a shared baseline for a group.
//!
//! The stack is *diverging*: positive values accumulate upward from zero and
//! negative values accumulate downward from zero, independently. A zero value
//! keeps the zero baseline and never advances either accumulator, which is also
//! what an implicit sparse zero means: it contributes nothing and stays at its
//! implicit zero. A sparse column is therefore only visited at the entries it
//! stores, never at every logical position.

use crate::sv_error::SVError;
use crate::sv_move::column::SVMoveColumn;
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
    state: &mut SVMoveState<'_>,
    request: &SVMoveRequest<'_>,
    options: &SVStackOptions,
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
        let value = values.value_at(index).unwrap_or(0.0);

        if value == 0.0 {
            baselines.sv_set(index, 0.0);
            values.sv_set(index, 0.0);
            continue;
        }

        let accumulator = if value > 0.0 {
            &mut positive_totals
        } else {
            &mut negative_totals
        };
        let baseline = accumulator.entry(group_key).or_insert(0.0);
        let point_baseline = *baseline;
        *baseline += value;
        baselines.sv_set(index, point_baseline);
        values.sv_set(index, point_baseline + value);
    }

    if options.mode_value() == SVStackMode::Center {
        sv_center_groups(values, baselines, group_keys);
    }

    state.values_changed = true;
    state.baselines_produced = true;
    Ok(())
}

/// Shifts every group so that its full extent is centred on zero.
///
/// The offset is the midpoint of the lowest baseline and the highest top, so a
/// diverging stack is centred as a whole rather than one side at a time. A
/// zero-height segment — a value equal to its baseline — is invariant under a
/// translation and keeps its zero baseline; that is what lets a sparse column
/// leave its implicit zeros untouched.
fn sv_center_groups(values: &mut SVMoveColumn, baselines: &mut SVMoveColumn, group_keys: &[u32]) {
    let mut extents: HashMap<u32, (f64, f64)> = HashMap::new();
    for index in values.sv_stored_positions() {
        let group_key = group_keys[index];
        let baseline = baselines.value_at(index).unwrap_or(0.0);
        let top = values.value_at(index).unwrap_or(0.0);
        let extent = extents.entry(group_key).or_insert((baseline, top));
        extent.0 = extent.0.min(baseline);
        extent.1 = extent.1.max(top);
    }

    for index in values.sv_stored_positions() {
        let group_key = group_keys[index];
        let baseline = baselines.value_at(index).unwrap_or(0.0);
        let top = values.value_at(index).unwrap_or(0.0);
        if top == baseline {
            continue;
        }
        if let Some((lowest_baseline, highest_top)) = extents.get(&group_key) {
            let offset = -((highest_top + lowest_baseline) / 2.0);
            baselines.sv_set(index, baseline + offset);
            values.sv_set(index, top + offset);
        }
    }
}
