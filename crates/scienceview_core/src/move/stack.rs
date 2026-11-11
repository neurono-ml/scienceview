//! Stacking: a shared baseline for a group.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMoveRequest, SVStackOptions};

/// Applies a stack to the working state.
///
/// This is the identity for now; the task's tests drive the implementation.
pub(crate) fn sv_apply_stack(
    _state: &mut SVMoveState,
    _request: &SVMoveRequest<'_>,
    _options: &SVStackOptions,
) -> Result<(), SVError> {
    Ok(())
}
