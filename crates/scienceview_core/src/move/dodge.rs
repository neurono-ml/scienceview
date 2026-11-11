//! Dodging: group members side by side inside a category slot.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVDodgeOptions, SVMoveRequest};

/// Applies a dodge to the working state.
///
/// This is the identity for now; the task's tests drive the implementation.
pub(crate) fn sv_apply_dodge(
    _state: &mut SVMoveState,
    _request: &SVMoveRequest<'_>,
    _options: &SVDodgeOptions,
) -> Result<(), SVError> {
    Ok(())
}
