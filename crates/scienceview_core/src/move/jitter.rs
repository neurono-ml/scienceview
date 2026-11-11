//! Jitter: a deterministic displacement within a category.

use crate::sv_error::SVError;
use crate::sv_move::working_state::SVMoveState;
use crate::sv_move::{SVJitterOptions, SVMoveRequest};

/// Applies a jitter to the working state.
///
/// This is the identity for now; the task's tests drive the implementation.
pub(crate) fn sv_apply_jitter(
    _state: &mut SVMoveState,
    _request: &SVMoveRequest<'_>,
    _options: &SVJitterOptions,
) -> Result<(), SVError> {
    Ok(())
}
