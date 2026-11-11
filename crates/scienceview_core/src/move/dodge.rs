//! Dodging: group members side by side inside a category slot.
//!
//! The width of a group comes from the number of distinct groups the category
//! actually has, so a category with two groups fills its slot with two bars and
//! a category with four fills it with four. The width is derived from the group
//! count; it is never assumed.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVDodgeOptions, SVMoveRequest};
use std::collections::BTreeMap;

/// Applies a dodge to the working state.
///
/// # Errors
///
/// Returns [`SVError::MismatchedLengths`] when the key columns and the values
/// disagree, and [`SVError::InvalidConfiguration`] when a point names a
/// category the request never declared.
pub(crate) fn sv_apply_dodge(
    state: &mut SVMoveState,
    request: &SVMoveRequest<'_>,
    options: &SVDodgeOptions,
) -> Result<(), SVError> {
    let group_keys = request.group_keys();
    let category_keys = request.category_keys();
    let number_of_points = state.values.len();
    if group_keys.len() != number_of_points || category_keys.len() != number_of_points {
        return Err(SVError::MismatchedLengths {
            name: "group_keys".to_string(),
            expected: number_of_points,
            actual: group_keys.len(),
        });
    }

    // Category -> (group -> ordinal), sorted by code so the layout is stable.
    let mut ordinals: BTreeMap<u32, BTreeMap<u32, usize>> = BTreeMap::new();
    for index in 0..number_of_points {
        let groups = ordinals.entry(category_keys[index]).or_default();
        let number_of_groups = groups.len();
        groups.entry(group_keys[index]).or_insert(number_of_groups);
    }

    let total_width = f64::from(options.width_value());
    let gap = f64::from(options.gap_value());

    for index in 0..number_of_points {
        let category_key = category_keys[index];
        let group_key = group_keys[index];
        let groups = ordinals
            .get(&category_key)
            .ok_or(SVError::InvalidConfiguration {
                message: format!(
                    "the dodge received the category code {category_key} without declaring it"
                ),
            })?;
        let number_of_groups = groups.len();
        let ordinal = *groups
            .get(&group_key)
            .ok_or(SVError::InvalidConfiguration {
                message: format!(
                    "the dodge received the group code {group_key} without declaring it"
                ),
            })?;

        if number_of_groups == 0 {
            continue;
        }

        let width_per_group = total_width * (1.0 - gap) / number_of_groups as f64;
        let centre =
            -total_width / 2.0 + total_width * (ordinal as f64 + 0.5) / number_of_groups as f64;

        state.horizontal_offsets[index] += centre;
        state.widths[index] = width_per_group;
    }

    state.widths_produced = true;
    Ok(())
}
