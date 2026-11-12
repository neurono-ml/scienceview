//! Shift: a fixed displacement in data units.

use crate::sv_error::SVError;
use crate::sv_move::column::SVMoveColumn;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVShiftDirection, SVShiftOptions};

/// Applies a shift to the working state.
///
/// A shift changes no value and no width: it only moves the marks, so it is the
/// one move that composes after every displacement and before normalising. It
/// shares the borrowed value column and never copies it; only the shifted
/// displacement column is materialised.
///
/// The displacement is uniform, so a sparse column records it once in its
/// implicit contribution instead of storing one offset per logical position:
/// the shift walks no logical position and keeps the column's stored entry
/// count unchanged.
pub(crate) fn sv_apply_shift(
    state: &mut SVMoveState<'_>,
    options: &SVShiftOptions,
) -> Result<(), SVError> {
    let amount = options.amount_value();
    let source = state.source;
    let offsets = match options.direction_value() {
        SVShiftDirection::Horizontal => &mut state.horizontal_offsets,
        SVShiftDirection::Vertical => &mut state.vertical_offsets,
    }
    .get_or_insert_with(|| SVMoveColumn::sv_zeroed_like(source));

    offsets.sv_add_uniform(amount);
    Ok(())
}
