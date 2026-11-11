//! Normalising: a group rescaled to a fraction or a percentage.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVMoveRequest, SVNormalizeOptions};

/// Applies a normalize to the working state.
///
/// This is the identity for now; the task's tests drive the implementation.
pub(crate) fn sv_apply_normalize(
    _state: &mut SVMoveState,
    _request: &SVMoveRequest<'_>,
    _options: &SVNormalizeOptions,
) -> Result<(), SVError> {
    Ok(())
}
