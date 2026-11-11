//! Shift: a fixed displacement in data units.

use crate::sv_error::SVError;
use crate::sv_move::SVShiftOptions;
use crate::sv_move::working_state::SVMoveState;

/// Applies a shift to the working state.
///
/// This is the identity for now; the task's tests drive the implementation.
pub(crate) fn sv_apply_shift(
    _state: &mut SVMoveState,
    _options: &SVShiftOptions,
) -> Result<(), SVError> {
    Ok(())
}
