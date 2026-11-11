//! Shift: a fixed displacement in data units.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVShiftDirection, SVShiftOptions};

/// Applies a shift to the working state.
///
/// A shift changes no value and no width: it only moves the marks, so it is the
/// one move that composes after every displacement and before normalising.
pub(crate) fn sv_apply_shift(
    state: &mut SVMoveState,
    options: &SVShiftOptions,
) -> Result<(), SVError> {
    let amount = options.amount_value();
    match options.direction_value() {
        SVShiftDirection::Horizontal => {
            for offset in &mut state.horizontal_offsets {
                *offset += amount;
            }
        }
        SVShiftDirection::Vertical => {
            for offset in &mut state.vertical_offsets {
                *offset += amount;
            }
        }
    }
    Ok(())
}
